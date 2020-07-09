use clap::*;

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("unset")
        .version(crate_version!())
        .about("Unset a config value")
}

pub fn run(_matches: &ArgMatches) {
    println!("config unset");
}
