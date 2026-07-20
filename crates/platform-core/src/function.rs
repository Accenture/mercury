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

//! The composable-function contract — Rust port of the Java
//! `LambdaFunction` / `TypedLambdaFunction<I, O>`
//! (`org.platformlambda.core.models`).
//!
//! [`ComposableFunction`] is the untyped registry currency (the `LambdaFunction`
//! analog): envelope in, envelope out. [`TypedFunction`] is the recommended
//! authoring surface (the `TypedLambdaFunction<I, O>` analog); wrap one in a
//! [`TypedAdapter`] to register it. [`AppError`] is the `AppException` analog —
//! HTTP-style status + message; a worker converts an `Err` into a response
//! envelope carrying that status.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::envelope::EventEnvelope;

/// The `AppException(status, message)` analog: HTTP-compatible status codes,
/// where >= 400 is an error.
#[derive(Clone, Debug, thiserror::Error)]
#[error("({status}) {message}")]
pub struct AppError {
    status: i32,
    message: String,
}

impl AppError {
    pub fn new(status: i32, message: impl Into<String>) -> Self {
        AppError {
            status,
            message: message.into(),
        }
    }

    pub fn status(&self) -> i32 {
        self.status
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// The untyped composable-function contract (the `LambdaFunction` analog) —
/// what the [`Platform`](crate::platform::Platform) registry stores.
///
/// `instance` is the 1-based worker number (Java parity). Functions must stay
/// stateless and never call other user functions directly — coupling is
/// route-name + envelope only (`inv-never-couple-functions`).
#[async_trait]
pub trait ComposableFunction: Send + Sync {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError>;
}

/// The typed authoring surface (the `TypedLambdaFunction<I, O>` analog):
/// implement this with concrete input/output types and register it via
/// [`TypedAdapter`].
#[async_trait]
pub trait TypedFunction<I, O>: Send + Sync
where
    I: DeserializeOwned + Send,
    O: Serialize,
{
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: I,
        instance: usize,
    ) -> Result<O, AppError>;
}

/// Bridges a [`TypedFunction`] to the untyped [`ComposableFunction`] registry
/// currency: deserializes the envelope body into `I`, invokes the typed
/// handler, and wraps `O` back into a response envelope.
pub struct TypedAdapter<T, I, O> {
    inner: T,
    _marker: PhantomData<fn(I) -> O>,
}

impl<T, I, O> TypedAdapter<T, I, O> {
    pub fn new(inner: T) -> Self {
        TypedAdapter {
            inner,
            _marker: PhantomData,
        }
    }

    /// Convenience: wrap directly into the `Arc<dyn ComposableFunction>` the
    /// registry expects.
    pub fn arc(inner: T) -> Arc<dyn ComposableFunction>
    where
        T: TypedFunction<I, O> + 'static,
        I: DeserializeOwned + Send + Sync + 'static,
        O: Serialize + Send + Sync + 'static,
    {
        Arc::new(TypedAdapter::new(inner))
    }
}

#[async_trait]
impl<T, I, O> ComposableFunction for TypedAdapter<T, I, O>
where
    T: TypedFunction<I, O>,
    I: DeserializeOwned + Send + Sync,
    O: Serialize + Send + Sync,
{
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let typed_input: I = input
            .body_as()
            .map_err(|e| AppError::new(400, format!("unable to map input: {e}")))?;
        let output = self
            .inner
            .handle_event(headers, typed_input, instance)
            .await?;
        EventEnvelope::new().set_body(output)
    }
}

/// The convenient no-operation function for event scripts (Java
/// `NoOpFunction`, route `no.op`): an echo — reply headers and body mirror
/// the input.
pub struct NoOpFunction;

#[async_trait]
impl ComposableFunction for NoOpFunction {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let mut response = EventEnvelope::new();
        for (key, value) in &headers {
            response = response.set_header(key, value);
        }
        Ok(response.set_raw_body(input.body().clone()))
    }
}
