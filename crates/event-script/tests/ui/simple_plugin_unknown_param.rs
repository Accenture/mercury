// #[simple_plugin] accepts a positional name literal or name = "..." and
// nothing else; any other parameter is a compile error.
#[event_script::simple_plugin(instances = 2)]
fn fixture(_args: &[rmpv::Value]) -> Result<rmpv::Value, String> {
    Ok(rmpv::Value::Nil)
}

fn main() {}
