use crate::cli::RunConfig;
use crate::markdown;
use crate::translator::MarkdownTranslator;
use anyhow::Result;

pub async fn run(config: RunConfig, translator: &impl MarkdownTranslator) -> Result<()> {
    let markdown_files = markdown::collect_markdown_files(&config.target_path, config.max_depth.unwrap_or(usize::MAX))?;

    for file in markdown_files {
        let content = std::fs::read_to_string(&file)?;

        for target_language in &config.languages {
            println!(
                "Translating file '{}' to '{}'",
                file.display(),
                target_language
            );

            let translation = translator
                .translate_markdown(&content, target_language)
                .await?;
            markdown::save_translated_file(file.as_path(), translation.as_str(), target_language)?;
        }
    }

    Ok(())
}
