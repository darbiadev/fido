use business_central::{Client, UrlKeyValue};
use clap::ArgMatches;
use figment::Figment;
use futures::executor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    base_url: String,
    tenant_id: String,
    environment: String,
    company_name: String,
    username: String,
    web_service_access_key: String,
}

pub(crate) fn process_matches(config_builder: Figment, matches: &ArgMatches) {
    let config: Config = config_builder.select("business_central").extract().unwrap();
    let client = Client::new(
        config.base_url,
        config.tenant_id,
        config.environment,
        config.company_name,
        config.username,
        config.web_service_access_key,
    );

    if let Some(matches) = matches.subcommand_matches("orders") {
        if let Some(matches) = matches.subcommand_matches("get") {
            if let Some(order_number) = matches.value_of("order-number") {
                executor::block_on(get_order(client, order_number));
            }
        }
    }
}

async fn get_order(client: Client, order_number: &str) {
    let response = client
        .make_odata_request(
            reqwest::Method::GET,
            "salesOrder".to_string(),
            Option::from(vec![
                UrlKeyValue::Text(String::from("Order")),
                UrlKeyValue::Text(order_number.to_string()),
            ]),
            Default::default(),
            None,
        )
        .await
        .unwrap();
    tracing::error!("{:#?}", response);
}
