mod add;
mod show;

use clap::*;
use rantt::options::Options;
use rantt::plan;

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("task")
        .version(crate_version!())
        .about("Manage tasks")
        .subcommands(vec![add::build_cli(), show::build_cli()])
}

pub fn run(matches: &ArgMatches, options: &Options) {
    let plan = plan::get_or_create_plan(&options.data.dir);
    match matches.subcommand() {
        ("add", Some(m)) => add::run(m, options),
        ("show", Some(m)) => show::run(m, &plan),
        _ => show::run(matches, &plan),
    }
}
