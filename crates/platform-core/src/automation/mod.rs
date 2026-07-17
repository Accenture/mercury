//! REST automation — the HTTP protocol boundary (Rust port of the Java
//! `org.platformlambda.automation` package, increment-6 core scope; see
//! `docs/design/platform-core-port.md` §5e).

pub mod http_client;
pub mod routing;
pub mod server;

pub use http_client::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
pub use routing::{AssignedRoute, CorsInfo, HeaderInfo, RouteInfo, RoutingTable};
pub use server::{start_http_server, MY_CORRELATION_ID};
