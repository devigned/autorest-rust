extern crate pretty_env_logger;
extern crate sample;
use sample::resources::resource_groups::ResourceGroups;

fn main() {
    pretty_env_logger::init().unwrap();
    let mut groups = ResourceGroups::new();
    groups.get("hello", "westus");
}