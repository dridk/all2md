# all2md

Converts DOC, DOCX, RTF, PDF to Markdown. Rust core with CLI and Python bindings.

## Structure

Cargo workspace with 3 crates:
- `crates/all2md-core` - library (strategy pattern, format detection, parsers)
- `crates/all2md-cli` - binary (`all2md`)
- `crates/all2md-python` - PyO3 bindings (maturin)

## Build

```bash
cargo build --release                    # CLI binary
cd crates/all2md-python && maturin develop  # Python wheel (dev)
```

## Test

```bash
cargo test
cd crates/all2md-python && maturin develop && python -m pytest python/tests/
```

## CLI Usage

```bash
all2md -i file.docx > out.md             # auto-detect format
all2md -i file.doc -f doc > out.md       # explicit format
```

## Architecture

- Strategy pattern: `FormatParser` trait in `strategy.rs`, one impl per format
- Magic-byte detection in `detect.rs`
- DOCX parser skips `RunChild::Drawing` and `RunChild::Shape` to exclude textbox text
- DOC uses `unword` crate, DOCX uses `docx-rs`, RTF uses `rtf-parser`, PDF uses `pdf_oxide`

## Key Conventions

- All parsers take `&[u8]` and return `Result<String, All2mdError>`
- Output is Markdown with `#` headings where detectable, plain text otherwise
