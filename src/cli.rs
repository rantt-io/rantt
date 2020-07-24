mod config;
mod task;

use clap::*;
use rantt::options;
use std::ffi::OsString;

pub fn run<I, T>(args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let options = options::get_or_create_config();

    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![config::build_cli(), task::build_cli()])
        .get_matches_from(args);

    match matches.subcommand() {
        ("config", Some(m)) => config::run(m, &options),
        ("task", Some(m)) => task::run(m, &options),
        _ => unreachable!(),
    }
}
