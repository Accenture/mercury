// A comma-separated route list with an empty segment must fail at compile
// time (validate_route_list).
#[platform_core::preload(route = "a.b, ,c.d")]
struct Fixture;

fn main() {}
