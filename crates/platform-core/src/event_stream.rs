//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! HTTP response streaming — the producer side of the multi-shot reply route
//! (Java `EventStreamWriter`).
//!
//! Streaming is native to the event system: a caller provides a `reply_to`
//! address and the callee may send it as many events as it likes. A streaming
//! HTTP response is a *sequence* of events to the caller's reply route, each
//! marked with the reserved envelope header `x-event-stream: data | eof |
//! exception`, until end of transmission. The marker is internal protocol
//! consumed by the REST automation edge — it never appears on the HTTP wire.
//! The first data event commits the HTTP head (status, content type, optional
//! idle-allowance override); writes after close are dropped.

use crate::envelope::EventEnvelope;
use crate::function::AppError;
use crate::platform::Platform;
use crate::post_office::PostOffice;

/// Reserved envelope header marking one event of a streaming HTTP response
/// (Java `EventStreamWriter.X_EVENT_STREAM`). Values: [`DATA`], [`EOF`],
/// [`EXCEPTION`]. Absence of the header = single-shot response.
pub const X_EVENT_STREAM: &str = "x-event-stream";

/// Optional companion header naming a data segment — maps to the SSE
/// `event:` field (Java `EventStreamWriter.X_EVENT_NAME`).
pub const X_EVENT_NAME: &str = "x-event-name";

/// One segment of the stream.
pub const DATA: &str = "data";
/// End of transmission (optional body = trailing metadata).
pub const EOF: &str = "eof";
/// In-band failure (body = the standard error key-values
/// `'{"type": "error", "status": n, "message": text}'`).
pub const EXCEPTION: &str = "exception";
/// Reserved SSE event name of the Event-over-HTTP envelope-mode wire dialect:
/// a frame with this name carries one base64-encoded serialized EventEnvelope
/// (Java `EventStreamWriter.ENVELOPE`).
pub const ENVELOPE: &str = "envelope";

/// Producer helper for a streaming HTTP response — thin sugar over plain
/// event sends to the caller's reply route (Java `EventStreamWriter`).
pub struct EventStreamWriter {
    po: PostOffice,
    reply_to: String,
    correlation_id: Option<String>,
    first_status: i32,
    first_content_type: Option<String>,
    first_ttl_seconds: u64,
    head_sent: bool,
    closed: bool,
}

impl EventStreamWriter {
    /// Create a writer for a reply route and correlation id.
    ///
    /// Returns HTTP-400 when the reply route is empty — a streaming producer
    /// without a reply address has nowhere to stream.
    pub fn new(
        platform: &Platform,
        reply_to: &str,
        correlation_id: Option<&str>,
    ) -> Result<Self, AppError> {
        if reply_to.is_empty() {
            return Err(AppError::new(
                400,
                "Streaming producer requires a reply_to address",
            ));
        }
        Ok(Self {
            po: PostOffice::new(platform),
            reply_to: reply_to.to_string(),
            correlation_id: correlation_id.map(str::to_string),
            first_status: 200,
            first_content_type: None,
            first_ttl_seconds: 0,
            head_sent: false,
            closed: false,
        })
    }

    /// Create a writer from the incoming request envelope (the usual form for
    /// an interceptor function).
    pub fn from_request(platform: &Platform, request: &EventEnvelope) -> Result<Self, AppError> {
        Self::new(
            platform,
            request.reply_to().unwrap_or(""),
            request.correlation_id(),
        )
    }

    /// Optional head control carried by the first outgoing event: response
    /// status and content type. Later events cannot change the head.
    pub fn first(&mut self, status: i32, content_type: &str) -> &mut Self {
        self.first_status = status;
        self.first_content_type = Some(content_type.to_string());
        self
    }

    /// Head control plus an idle-allowance override in seconds between
    /// segments (rides the first event as the `x-ttl` envelope header).
    pub fn first_with_ttl(
        &mut self,
        status: i32,
        content_type: &str,
        ttl_seconds: u64,
    ) -> &mut Self {
        self.first_ttl_seconds = ttl_seconds;
        self.first(status, content_type)
    }

    /// Send one `data` segment (String, bytes or map — any serializable body).
    pub async fn write<T: serde::Serialize>(&mut self, segment: T) -> Result<(), AppError> {
        self.send(segment, None).await
    }

    /// Send one named segment — the name maps to the SSE `event:` field.
    pub async fn write_named<T: serde::Serialize>(
        &mut self,
        event_name: &str,
        segment: T,
    ) -> Result<(), AppError> {
        self.send(segment, Some(event_name)).await
    }

    /// Declare end of transmission.
    pub async fn close(&mut self) -> Result<(), AppError> {
        self.close_with(serde_json::Value::Null).await
    }

    /// Declare end of transmission with trailing metadata (rendered as the
    /// terminal SSE event's data; ignored in chunked mode).
    pub async fn close_with<T: serde::Serialize>(&mut self, metadata: T) -> Result<(), AppError> {
        if self.closed {
            return Ok(());
        }
        self.closed = true;
        let event = self.envelope(EOF, metadata, None)?;
        self.po.send(event).await
    }

    /// Declare an in-band failure and end the stream.
    pub async fn fail(&mut self, error: &AppError) -> Result<(), AppError> {
        if self.closed {
            return Ok(());
        }
        self.closed = true;
        let status = if error.status() >= 400 {
            error.status()
        } else {
            500
        };
        // the standard error key-values: '{"type": "error", "status": n, "message": text}'
        let body =
            serde_json::json!({"type": "error", "status": status, "message": error.message()});
        let event = self.envelope(EXCEPTION, body, None)?.set_status(status);
        self.po.send(event).await
    }

    /// True when the stream has been closed or failed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    async fn send<T: serde::Serialize>(
        &mut self,
        body: T,
        event_name: Option<&str>,
    ) -> Result<(), AppError> {
        if self.closed {
            log::debug!(
                "Segment to {} dropped - stream already closed",
                self.reply_to
            );
            return Ok(());
        }
        let event = self.envelope(DATA, body, event_name)?;
        self.po.send(event).await
    }

    fn envelope<T: serde::Serialize>(
        &mut self,
        marker: &str,
        body: T,
        event_name: Option<&str>,
    ) -> Result<EventEnvelope, AppError> {
        let mut event = EventEnvelope::new()
            .set_to(&self.reply_to)
            .set_header(X_EVENT_STREAM, marker)
            .set_body(body)?;
        if let Some(cid) = &self.correlation_id {
            event = event.set_correlation_id(cid);
        }
        if let Some(name) = event_name.filter(|n| !n.is_empty()) {
            event = event.set_header(X_EVENT_NAME, name);
        }
        if !self.head_sent {
            self.head_sent = true;
            event = event.set_status(self.first_status);
            if let Some(content_type) = &self.first_content_type {
                event = event.set_header("content-type", content_type);
            }
            if self.first_ttl_seconds > 0 {
                event = event.set_header("x-ttl", &self.first_ttl_seconds.to_string());
            }
        }
        Ok(event)
    }
}
