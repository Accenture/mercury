//! REST automation — the HTTP protocol boundary (Rust port of the Java
//! `org.platformlambda.automation` package, increment-6 core scope; see
//! `docs/design/platform-core-port.md` §5e).

pub mod routing;
pub mod server;

pub use routing::{AssignedRoute, CorsInfo, HeaderInfo, RouteInfo, RoutingTable};
pub use server::{start_http_server, MY_CORRELATION_ID};
