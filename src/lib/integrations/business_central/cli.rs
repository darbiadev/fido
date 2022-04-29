use business_central::UrlKeyValue;
use clap::{Arg, ArgMatches, Command};
use figment::Figment;
use futures::executor;
use serde::Deserialize;

pub(crate) fn build_command() -> Command<'static> {
    Command::new("business-central")
        .about("Interact with Business Central")
        .subcommand(
            Command::new("api")
                .about("Make authenticated HTTP requests")
                .subcommand(
                    Command::new("odata")
                        .about("Make authenticated odata requests")
                        .arg(
                            Arg::new("path")
                                .help("path")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("resource_values")
                                .help("resource_values")
                                .multiple_values(true)
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    Command::new("unbound")
                        .about("Make authenticated unbound requests")
                        .arg(
                            Arg::new("path")
                                .help("path")
                                .required(true)
                                .multiple_values(true)
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("orders")
                .about("Interact with orders")
                .subcommand(
                    Command::new("get")
                        .about("Get order")
                        .long_about("Get an order from Business Central")
                        .arg(
                            Arg::new("order-number")
                                .help("Order number")
                                .takes_value(true),
                        ),
                ),
        )
}

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
    if let Some(matches) = matches.subcommand_matches("api") {
        if let Some(matches) = matches.subcommand_matches("odata") {
            if let Some(path) = matches.value_of("path") {
                let mut resource_values: Vec<UrlKeyValue> = Vec::new();
                if matches.is_present("resource_values") {
                    let values: Vec<&str> = matches.values_of("resource_values").unwrap().collect();
                    for value in values {
                        let parsed_value = value.parse::<i16>();
                        if let Ok(..) = parsed_value {
                            resource_values.push(UrlKeyValue::Number(parsed_value.unwrap()))
                        } else {
                            resource_values.push(UrlKeyValue::Text(value.to_string()))
                        }
                    }
                }
                let sales_order = executor::block_on(
                    business_central::resources::sales_orders::handlers::get_generic(
                        client,
                        path,
                        resource_values,
                    ),
                )
                .unwrap();
                tracing::error!("{:#?}", sales_order)
            }
        } else if let Some(_matches) = matches.subcommand_matches("unbound") {
            tracing::error!("{}", "unimplemented!");
        }
    } else if let Some(matches) = matches.subcommand_matches("orders") {
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
