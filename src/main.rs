//! Fido

use std::error::Error;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use crate::lib::cli::{build_cli, process_matches};

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config_toml_path = dirs::config_dir().expect("Failed to get user config directory");
    config_toml_path.push("darbia");
    config_toml_path.push("fido.toml");

    let mut config_builder = Figment::new()
        .merge(Toml::file(config_toml_path).nested())
        .merge(Env::prefixed("FIDO_"));

    let matches = build_cli().get_matches();

    if let Some(passed_config_file) = matches.value_of("config-file") {
        config_builder = config_builder
            .clone()
            .merge(Toml::file(passed_config_file).nested());
    }

    let mut log_level = match matches.occurrences_of("verbose") {
        4 => tracing_subscriber::filter::LevelFilter::TRACE,
        3 => tracing_subscriber::filter::LevelFilter::DEBUG,
        2 => tracing_subscriber::filter::LevelFilter::INFO,
        1 => tracing_subscriber::filter::LevelFilter::WARN,
        _ => tracing_subscriber::filter::LevelFilter::ERROR,
    };

    if matches.is_present("quiet") {
        log_level = tracing_subscriber::filter::LevelFilter::OFF;
    }

    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().pretty())
        .with_max_level(log_level)
        .init();

    process_matches(config_builder, matches);

    Ok(())
}
