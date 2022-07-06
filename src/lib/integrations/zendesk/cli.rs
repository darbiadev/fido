use clap::{Arg, ArgMatches, Command};
use figment::Figment;
use futures::executor;
use serde::Deserialize;

use crate::Context;

#[derive(Debug, Deserialize)]
struct Config {
    base_url: String,
    email: String,
    api_token: String,
    oauth_client_id: String,
}

pub(crate) fn build_command() -> Command<'static> {
    Command::new("zendesk")
        .about("Interact with Zendesk")
        .subcommand(
            Command::new("tickets")
                .about("Interact with tickets")
                .subcommand(
                    Command::new("get")
                        .about("Get ticket")
                        .long_about("Get a ticket from Zendesk")
                        .arg(
                            Arg::new("ticket-number")
                                .help("Ticket number")
                                .takes_value(true),
                        ),
                ),
        )
}

pub(crate) fn process_matches(context: Context, config_builder: Figment, matches: &ArgMatches) {
    let config: Config = config_builder.select("zendesk").extract().unwrap();
    let mut client = zendesk::Client::new(
        config.base_url,
        config.email,
        config.api_token,
        config.oauth_client_id,
    );

    if let Some(matches) = matches.subcommand_matches("tickets") {
        if let Some(matches) = matches.subcommand_matches("get") {
            if let Some(ticket_number) = matches.value_of("ticket-number") {
                executor::block_on(client.update_token());
                let ticket = executor::block_on(zendesk::resources::tickets::handlers::get_ticket(
                    client,
                    ticket_number,
                ))
                .unwrap();
                if !context.quiet {
                    println!("{:#?}", ticket)
                }
            }
        }
    }
}
