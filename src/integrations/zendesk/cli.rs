//! Using Zendesk

use clap::{Arg, ArgMatches, Command};
use figment::Figment;
use serde::Deserialize;
use zendesk::resources::tickets::handlers::get_ticket;

use crate::Context;

/// Auth config
#[derive(Deserialize)]
struct Config {
    /// Base URL of the instance
    base_url: String,
    /// Account email
    email: String,
    /// Auth token
    api_token: String,
    /// OAUTH application client ID
    oauth_client_id: String,
}

/// Build the Clap command
pub(crate) fn build_command() -> Command {
    Command::new("zendesk")
        .about("Interact with Zendesk")
        .subcommand(
            Command::new("tickets")
                .about("Interact with tickets")
                .subcommand(
                    Command::new("get")
                        .about("Get ticket")
                        .long_about("Get a ticket from Zendesk")
                        .arg(Arg::new("ticket-number").help("Ticket number")),
                ),
        )
}

/// Process parsed matches and dispatch to functions
pub(crate) async fn process_matches(config_builder: Figment, matches: &ArgMatches) {
    let context = Context::from_matches(matches);
    let config: Config = config_builder.select("zendesk").extract().unwrap();
    let mut client = zendesk::Client::new(
        config.base_url,
        config.email,
        config.api_token,
        config.oauth_client_id,
    );

    if let Some(matches) = matches.subcommand_matches("tickets") {
        if let Some(matches) = matches.subcommand_matches("get") {
            if let Some(ticket_number) = matches.get_one::<String>("ticket-number") {
                client.update_token().await;
                let ticket = get_ticket(client, ticket_number).await.unwrap();
                if !context.quiet {
                    println!("{ticket:#?}");
                }
            }
        }
    }
}
