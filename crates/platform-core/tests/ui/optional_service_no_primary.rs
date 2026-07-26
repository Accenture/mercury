// #[optional_service] must be stacked with one of the four primary
// attributes; alone it is a compile error.
#[platform_core::optional_service("app.env=dev")]
struct Fixture;

fn main() {}
