mod config;

use clap::*;
use std::ffi::OsString;

pub fn run<I, T>(args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(config::build_cli())
        .get_matches_from(args);

    match matches.subcommand() {
        ("config", Some(m)) => config::run(m),
        _ => unreachable!(),
    }
}
