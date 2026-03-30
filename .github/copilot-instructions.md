# Copilot Instructions for Rust in This Repository

## Project Goal

Build and maintain a Rust CLI that:
- accepts a target path and target language,
- finds Markdown files under that path,
- translates their contents,
- writes translated output based on CLI options.

## Architecture Expectations

- Keep `src/main.rs` thin (wiring only).
- Put argument parsing in `src/cli.rs`.
- Put file discovery logic in `src/markdown.rs`.
- Put workflow orchestration in `src/app.rs`.
- Put translation abstraction and implementations in `src/translator.rs`.
- Prefer small, focused functions and clear module boundaries.

## Rust Coding Guidelines

- Target stable Rust and idiomatic patterns.
- Prefer `Result<T, anyhow::Error>` at app boundaries.
- Add context to propagated errors using `anyhow::Context`.
- Avoid `unwrap()`/`expect()` in production code unless failure is unrecoverable.
- Prefer explicit, readable code over clever abstractions.
- Keep comments concise and only where logic is non-obvious.

## Traits and Abstractions

- Keep `MarkdownTranslator` as the abstraction for translation behavior.
- Use concrete translator names for real implementations (for example, `AzureTranslator`).
- Use `impl Trait` for compile-time polymorphism when runtime swapping is not needed.
- Use `dyn Trait` only when runtime selection/storage of different translators is required.

## CLI and File Handling

- Validate CLI inputs with clear user-facing errors.
- Support recursive Markdown discovery (`.md`, `.markdown`, case-insensitive).
- Preserve relative paths when writing to an output directory.
- Never overwrite files unless explicitly requested (`--in-place`).

## Markdown Translation Behavior

- Preserve Markdown structure and formatting during translation.
- Do not translate code fences, inline code, frontmatter keys, or URLs unless explicitly requested.
- Keep link/image targets intact.
- Keep line ending behavior consistent with input where practical.

## Testing and Validation

When changing Rust code, Copilot should run:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Add tests for:
- CLI parsing edge cases,
- Markdown file detection/discovery,
- output path mapping,
- translator behavior with mocked/stubbed implementations.

## Dependency and Security Preferences

- Keep dependencies minimal and well-maintained.
- Prefer established crates over custom implementations for common tasks.
- Avoid adding async runtime dependencies unless required by translation API design.

## Pull Request Quality Bar

- Changes should compile and tests should pass.
- Public function behavior should be covered by tests.
- Error messages should help users self-correct input issues.
- Keep changes scoped; avoid unrelated refactors in the same PR.
