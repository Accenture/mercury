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

//! REST automation — the HTTP protocol boundary (Rust port of the Java
//! `org.platformlambda.automation` package, increment-6 core scope; see
//! `draft-design-specs/platform-core-port.md` §5e).

pub mod event_api;
pub mod http_client;
pub mod routing;
pub mod server;
pub mod ws_server;

pub use event_api::{
    event_over_http, event_over_http_with_headers, get_event_http_target, EventApiService,
    EventHttpTarget, EVENT_API_SERVICE, X_EVENT_API,
};
pub use http_client::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
pub use routing::{AssignedRoute, CorsInfo, HeaderInfo, RouteInfo, RoutingTable};
pub use server::{
    available_lanes, checkout_lane, release_lane, server_address, start_http_server,
    AsyncHttpResponseService, StreamLaneService, ASYNC_HTTP_RESPONSE,
    ASYNC_HTTP_RESPONSE_STREAM_POOL, MY_CORRELATION_ID,
};
pub use ws_server::register_ws_service;
