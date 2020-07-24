use clap::*;
use rantt::options::Options;

pub fn build_cli<'b, 'a>() -> App<'a, 'b> {
    App::new("add")
        .about("Adds a new task")
        .version(crate_version!())
}

pub fn run(_matches: &ArgMatches, _options: &Options) {
    println!("task add");
}
