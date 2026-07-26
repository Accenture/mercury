// #[optional_service] requires a non-empty condition string.
#[platform_core::optional_service("")]
#[platform_core::preload(route = "ui.test")]
struct Fixture;

fn main() {}
