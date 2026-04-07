use anyhow::Result;
use crate::cli::RunConfig;
use crate::translator::MarkdownTranslator;
use crate::markdown;

pub fn run(config: RunConfig, translator: &impl MarkdownTranslator) -> Result<()> {
    let markdown_files = markdown::collect_markdown_files(&config.target_path)?;

    for file in markdown_files {
        let content = std::fs::read_to_string(&file)?;

        for target_language in &config.languages {
            println!("Translating file '{}' to '{}'", file.display(), target_language);

            let _ = translator.translate_markdown(&content, target_language)?;
            //println!("{}", translation);
        }
    }

    Ok(())
}
