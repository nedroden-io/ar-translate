# ar-translate

CLI tool to recursively translate Markdown files in a target path.

## Usage

```bash
ar-translate --target-path <target-path> --target-language <target-language> [--max-depth <max-depth>]
```

Examples:

```bash
# Long parameters and multiple target languages
ar-translate --target-path ./docs --target-language de,es,fr

# Short parameters and max depth of 1
ar-translate -t . -l de,nl -m 1
```
