use clap::*;
use rantt::options::Options;

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("show")
        .about("Show current config values (default)")
        .version(crate_version!())
}

pub fn run(_matches: &ArgMatches, config: &Options) {
    println!("{}", config);
}
