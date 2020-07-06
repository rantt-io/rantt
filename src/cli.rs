use clap::*;
use std::ffi::OsString;

pub fn run<I, T>(args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    app_from_crate!().get_matches_from(args);
}
