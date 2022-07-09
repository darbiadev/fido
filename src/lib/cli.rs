use clap::{crate_authors, crate_version, value_parser, Arg, ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};

pub(crate) struct Context {
    pub(crate) quiet: bool,
}

impl Context {
    pub fn new(quiet: bool) -> Context {
        Context { quiet }
    }
}

pub(crate) fn build_cli() -> Command<'static> {
    Command::new("fido")
        .about("FIDO CLI")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .infer_long_args(true)
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .conflicts_with("quiet")
                .help("Sets the level of verbosity")
                .multiple_occurrences(true),
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
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate completions")
                .long_about("Generate completions for FIDO")
                .arg(
                    Arg::new("shell")
                        .long("shell")
                        .help("The shell to generate completions for")
                        .value_parser(value_parser!(Shell)),
                ),
        )
        .subcommand(crate::lib::integrations::business_central::cli::build_command())
        .subcommand(crate::lib::integrations::zendesk::cli::build_command())
}

fn print_completions<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut std::io::stdout());
}

pub(crate) fn process_matches(
    context: Context,
    config_builder: figment::Figment,
    matches: ArgMatches,
) {
    if let Some(matches) = matches.subcommand_matches("completions") {
        if let Ok(shell) = matches.value_of_t::<Shell>("shell") {
            let mut app = build_cli();
            print_completions(shell, &mut app);
        }
    } else if let Some(matches) = matches.subcommand_matches("business-central") {
        crate::lib::integrations::business_central::cli::process_matches(
            context,
            config_builder,
            matches,
        )
    } else if let Some(matches) = matches.subcommand_matches("zendesk") {
        crate::lib::integrations::zendesk::cli::process_matches(context, config_builder, matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_app_works() {
        let app = build_cli();

        assert_eq!(app.get_name(), "fido");
    }
}
