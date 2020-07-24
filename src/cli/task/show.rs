use clap::*;
use rantt::plan::Plan;

pub fn build_cli<'b, 'a>() -> App<'a, 'b> {
    App::new("show")
        .about("Show current tasks (default)")
        .version(crate_version!())
}

pub fn run(_matches: &ArgMatches, plan: &Plan) {
    println!("{}", serde_json::to_string_pretty(plan).unwrap());
}
