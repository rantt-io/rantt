mod init;
mod set;
mod unset;

use clap::*;

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("config")
        .version(crate_version!())
        .about("Get and set rantt options")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![
            init::build_cli(),
            set::build_cli(),
            unset::build_cli(),
        ])
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("init", Some(m)) => init::run(m),
        ("set", Some(m)) => set::run(m),
        ("unset", Some(m)) => unset::run(m),
        _ => unreachable!(),
    }
}
