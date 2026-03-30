# ar-translate

CLI tool to recursively translate Markdown files in a target path.

## Project layout

- `src/main.rs`: entrypoint and app wiring
- `src/cli.rs`: argument parsing and runtime configuration
- `src/app.rs`: orchestration (discover -> translate -> write)
- `src/markdown.rs`: Markdown file discovery helpers
- `src/translator.rs`: translator trait and placeholder implementation

## Usage

```bash
cargo run -- <target-path> <target-language> [--in-place] [--out-dir <path>]
```

Examples:

```bash
cargo run -- ./docs es
cargo run -- ./docs es --out-dir ./translated
cargo run -- ./docs es --in-place
```

## Implement real translation calls

Replace `PassthroughTranslator` in `src/translator.rs` with your API-backed translator implementation and keep the `MarkdownTranslator` trait contract.

