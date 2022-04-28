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
    let client = business_central::BusinessCentralServices::new(
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
                let sales_order = executor::block_on(
                    business_central::resources::sales_orders::handlers::get_order(
                        client,
                        order_number,
                    ),
                )
                .unwrap();
                tracing::error!("{:#?}", sales_order)
            }
        }
    }
}
