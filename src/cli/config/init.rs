use clap::*;

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("init")
        .version(crate_version!())
        .about("Initialize .rantrc")
}

pub fn run(_matches: &ArgMatches) {
    println!("config init");
}
