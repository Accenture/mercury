// #[preload] without a route is a compile error.
#[platform_core::preload(instances = 2)]
struct Fixture;

fn main() {}
