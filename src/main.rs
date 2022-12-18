//! Fido

#![forbid(clippy::missing_docs_in_private_items)]
#![allow(clippy::multiple_crate_versions)]
#![warn(rust_2018_idioms)]

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use crate::cli::{build_cli, process_matches, Context};

mod cli;
mod integrations;

#[tokio::main]
async fn main() {
    let mut config_toml_path = dirs::config_dir().expect("Failed to get user config directory");
    config_toml_path.push("darbia");
    config_toml_path.push("fido.toml");

    let mut config_builder = Figment::new()
        .merge(Toml::file(config_toml_path).nested())
        .merge(Env::prefixed("FIDO_"));

    let matches = build_cli().get_matches();

    if let Some(config_file) = matches.get_one::<String>("config-file") {
        config_builder = config_builder
            .clone()
            .merge(Toml::file(config_file).nested());
    }

    let mut log_level = match matches.get_count("verbose") {
        0 => tracing_subscriber::filter::LevelFilter::ERROR,
        1 => tracing_subscriber::filter::LevelFilter::WARN,
        2 => tracing_subscriber::filter::LevelFilter::INFO,
        3 => tracing_subscriber::filter::LevelFilter::DEBUG,
        _ => tracing_subscriber::filter::LevelFilter::TRACE,
    };

    if matches.contains_id("quiet") {
        log_level = tracing_subscriber::filter::LevelFilter::OFF;
    }

    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().pretty())
        .with_max_level(log_level)
        .init();

    process_matches(config_builder, matches).await;
}
