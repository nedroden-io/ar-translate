use std::process;

mod app;
mod cli;
mod markdown;
mod translator;
mod app_settings;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error:#}");
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let app_config = app_settings::AppConfig::load()?;
    let run_settings = cli::parse_args()?;

    // Placeholder type to keep module wiring in place while implementations are pending.
    let translator = translator::AzureTranslator;

    app::run(app_config, run_settings, &translator)
}
