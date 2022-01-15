use clap::ArgMatches;
use figment::Figment;
use futures::executor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    base_url: String,
    email: String,
    api_token: String,
    oauth_client_id: String,
}

pub(crate) fn process_matches(config_builder: Figment, matches: &ArgMatches) {
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
                tracing::error!("{:#?}", ticket)
            }
        }
    }
}
