use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};

pub(crate) struct Context {
    pub(crate) quiet: bool,
}

impl Context {
    pub fn new(quiet: bool) -> Context {
        Context { quiet }
    }
}

pub(crate) fn build_cli() -> Command {
    command!()
        .infer_long_args(true)
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .conflicts_with("quiet")
                .action(ArgAction::Count)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .conflicts_with("verbose")
                .help("Suppresses all output"),
        )
        .arg(
            Arg::new("config-file")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate completions")
                .long_about("Generate completions for FIDO")
                .arg(
                    Arg::new("generator")
                        .long("generate")
                        .help("The shell to generate completions for")
                        .value_parser(value_parser!(Shell)),
                ),
        )
        .subcommand(crate::integrations::business_central::cli::build_command())
        .subcommand(crate::integrations::zendesk::cli::build_command())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

pub(crate) async fn process_matches(
    context: Context,
    config_builder: figment::Figment,
    matches: ArgMatches,
) {
    if let Some(matches) = matches.subcommand_matches("completions") {
        if let Some(generator) = matches.get_one::<Shell>("generator") {
            let mut cmd = build_cli();
            print_completions(*generator, &mut cmd);
        }
    } else if let Some(matches) = matches.subcommand_matches("business-central") {
        crate::integrations::business_central::cli::process_matches(
            context,
            config_builder,
            matches,
        )
        .await;
    } else if let Some(matches) = matches.subcommand_matches("zendesk") {
        crate::integrations::zendesk::cli::process_matches(context, config_builder, matches).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_app_works() {
        let app = build_cli();
        app.debug_assert();
    }
}
