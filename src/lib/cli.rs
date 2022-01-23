use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};
use clap_complete::{generate, Generator, Shell};

pub(crate) fn build_cli() -> App<'static> {
    App::new("fido")
        .about("FIDO CLI")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .global_setting(AppSettings::InferLongArgs)
        .global_setting(AppSettings::InferSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
            App::new("completions")
                .about("Generate completions")
                .long_about("Generate completions for FIDO")
                .arg(
                    Arg::new("shell")
                        .long("shell")
                        .help("The shell to generate completions for")
                        .possible_values(Shell::possible_values()),
                ),
        )
        .subcommand(
            App::new("business-central")
                .about("Interact with Business Central")
                .subcommand(
                    App::new("orders").about("Interact with orders").subcommand(
                        App::new("get")
                            .about("Get order")
                            .long_about("Get an order from Business Central")
                            .arg(
                                Arg::new("order-number")
                                    .help("Order number")
                                    .takes_value(true),
                            ),
                    ),
                ),
        )
        .subcommand(
            App::new("zendesk")
                .about("Interact with Zendesk")
                .subcommand(
                    App::new("tickets")
                        .about("Interact with tickets")
                        .subcommand(
                            App::new("get")
                                .about("Get ticket")
                                .long_about("Get a ticket from Zendesk")
                                .arg(
                                    Arg::new("ticket-number")
                                        .help("Ticket number")
                                        .takes_value(true),
                                ),
                        ),
                ),
        )
}

fn print_completions<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut std::io::stdout());
}

pub(crate) fn process_matches(config_builder: figment::Figment, matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("completions") {
        if let Ok(shell) = matches.value_of_t::<Shell>("shell") {
            let mut app = build_cli();
            print_completions(shell, &mut app);
        }
    } else if let Some(matches) = matches.subcommand_matches("business-central") {
        crate::lib::integrations::business_central::cli::process_matches(config_builder, matches)
    } else if let Some(matches) = matches.subcommand_matches("zendesk") {
        crate::lib::integrations::zendesk::cli::process_matches(config_builder, matches)
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
