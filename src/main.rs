use crate::azure::AzureClient;
use std::process;

mod app;
mod app_settings;
mod azure;
mod cli;
mod markdown;
mod translator;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error:#}");
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let app_config = app_settings::AppConfig::load()?;
    let run_settings = cli::parse_args()?;

    let azure_client = AzureClient::new(
        app_config.azure_api_key,
        app_config.azure_api_region,
        app_config.azure_api_endpoint,
    );

    // Placeholder type to keep module wiring in place while implementations are pending.
    let translator = translator::AzureTranslator::new(&azure_client);

    app::run(run_settings, &translator)
}
