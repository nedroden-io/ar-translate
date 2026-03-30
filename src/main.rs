use std::process;

mod app;
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
    let config = cli::parse_args(std::env::args())?;

    // Placeholder type to keep module wiring in place while implementations are pending.
    let translator = translator::AzureTranslator;

    app::run(config, &translator)
}
