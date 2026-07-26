// #[fetch_feature] rejects anything beyond the feature name — the D3a
// boundary guard: #[optional_service("...")] STACKS as a separate marker
// attribute, it is not an inline parameter.
#[knowledge_graph::fetch_feature(name = "guarded", optional_service = "app.env=dev")]
struct Fixture;

fn main() {}
