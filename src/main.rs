mod cli;
use std::env;

fn main() {
    cli::run(env::args());
}
