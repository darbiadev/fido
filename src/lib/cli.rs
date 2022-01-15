use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};
use figment::Figment;

pub(crate) fn build_cli() -> App<'static> {
    App::new("fido")
        .about("FIDO CLI")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .global_setting(AppSettings::InferLongArgs)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
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
}

pub(crate) fn process_matches(config_builder: Figment, matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("business-central") {
        crate::lib::integrations::business_central::cli::process_matches(config_builder, matches)
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
