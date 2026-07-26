// The #[zero_tracing] marker with no #[preload] on the item is a compile
// error pointing at the inline equivalent.
#[platform_core::zero_tracing]
struct Fixture;

fn main() {}
