// The #[event_interceptor] marker with no #[preload] on the item is a
// compile error pointing at the inline equivalent.
#[platform_core::event_interceptor]
struct Fixture;

fn main() {}
