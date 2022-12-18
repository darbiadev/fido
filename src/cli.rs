//! CLI

use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};

/// Command execution context
#[derive(Debug)]
pub(crate) struct Context {
    /// No output when enabled
    pub(crate) quiet: bool,
}

impl Context {
    /// Build context from matches
    pub(crate) fn from_matches(matches: &ArgMatches) -> Context {
        let quiet = matches.get_one::<bool>("quiet").unwrap();
        Context { quiet: *quiet }
    }
}

/// Build the main CLI
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
                .help("Sets the level of verbosity")
                .global(true),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .conflicts_with("verbose")
                .action(ArgAction::SetTrue)
                .help("Suppresses all output")
                .global(true),
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
        .subcommand(crate::integrations::zendesk::cli::build_command())
        .subcommand(crate::integrations::python::cli::build_command())
        .subcommand(crate::integrations::shelby::cli::build_command())
}

/// Generate completions and print to STDOUT
fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

/// Process parsed matches and dispatch to subcommands
pub(crate) async fn process_matches(config_builder: figment::Figment, matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("completions") {
        if let Some(generator) = matches.get_one::<Shell>("generator") {
            let mut cmd = build_cli();
            print_completions(*generator, &mut cmd);
        }
    } else if let Some(matches) = matches.subcommand_matches("zendesk") {
        crate::integrations::zendesk::cli::process_matches(config_builder, matches).await;
    } else if let Some(matches) = matches.subcommand_matches("python") {
        crate::integrations::python::cli::process_matches(&config_builder, matches);
    } else if let Some(matches) = matches.subcommand_matches("shelby") {
        crate::integrations::shelby::cli::process_matches(&config_builder, matches);
    }
}

#[test]
fn verify_cli() {
    build_cli().debug_assert();
}
