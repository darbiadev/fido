use clap::{Arg, ArgMatches, Command};
use figment::Figment;
use shelby::Shelby;

use crate::Context;

pub(crate) fn build_command() -> Command {
    Command::new("shelby")
        .about("Interact with shelby")
        .subcommand(
            Command::new("eval")
                .about("Evaluate data")
                .arg(Arg::new("data").help("The data to evaluate")),
        )
}

pub(crate) fn process_matches(_config_builder: &Figment, matches: &ArgMatches) {
    let context = Context::from_matches(matches);
    if let Some(matches) = matches.subcommand_matches("eval") {
        if let Some(data) = matches.get_one::<String>("data") {
            let result = run_shelby(data.to_string());
            if !context.quiet {
                println!("{result:#?}");
            }
        }
    }
}

fn run_shelby(data: String) -> String {
    let client = Shelby::new(true);
    client.parse(data)
}
