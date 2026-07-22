//! REST automation — the HTTP protocol boundary (Rust port of the Java
//! `org.platformlambda.automation` package, increment-6 core scope; see
//! `docs/design/platform-core-port.md` §5e).

pub mod event_api;
pub mod http_client;
pub mod routing;
pub mod server;
pub mod ws_server;

pub use event_api::{event_over_http, EventApiService, EVENT_API_SERVICE};
pub use http_client::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
pub use routing::{AssignedRoute, CorsInfo, HeaderInfo, RouteInfo, RoutingTable};
pub use server::{server_address, start_http_server, MY_CORRELATION_ID};
pub use ws_server::register_ws_service;
