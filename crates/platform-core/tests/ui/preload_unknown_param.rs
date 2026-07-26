// An unknown #[preload] parameter must fail compilation with the list of
// valid parameters (and the pointer to #[optional_service]).
#[platform_core::preload(route = "ui.test", nonsense = 1)]
struct Fixture;

fn main() {}
