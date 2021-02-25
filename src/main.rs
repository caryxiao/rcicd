use clap::{load_yaml,App};
fn main() {
    let yaml = load_yaml!("cli.yaml");
    App::from(yaml).get_matches();
}
