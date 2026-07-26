// #[websocket_service] without a service name is a compile error.
#[platform_core::websocket_service]
struct Fixture;

fn main() {}
