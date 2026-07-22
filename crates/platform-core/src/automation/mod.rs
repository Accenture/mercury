//! REST automation — the HTTP protocol boundary (Rust port of the Java
//! `org.platformlambda.automation` package, increment-6 core scope; see
//! `docs/design/platform-core-port.md` §5e).

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
pub use server::{server_address, start_http_server, MY_CORRELATION_ID};
pub use ws_server::register_ws_service;
