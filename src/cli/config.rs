mod set;
mod show;
mod unset;

use clap::*;
use rantt::options::Options;

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("config")
        .version(crate_version!())
        .about("Get and set rantt options")
        .subcommands(vec![
            set::build_cli(),
            unset::build_cli(),
            show::build_cli(),
        ])
}

pub fn run(matches: &ArgMatches, options: &Options) {
    match matches.subcommand() {
        ("set", Some(m)) => set::run(m),
        ("unset", Some(m)) => unset::run(m),
        ("show", Some(m)) => show::run(m, options),
        _ => show::run(matches, options),
    }
}
