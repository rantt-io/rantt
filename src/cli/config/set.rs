use clap::*;

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("set")
        .version(crate_version!())
        .about("Set a config value")
}

pub fn run(_matches: &ArgMatches) {
    println!("config set");
}
