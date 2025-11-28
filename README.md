# fin_converter_cli

A small, fast command‑line tool to read, convert, and compare simple financial records across CSV, TXT, and BIN formats. It is powered by the shared library `fin_converter_lib`.

## Features
- Read and pretty‑print records from `.csv`, `.txt`, or `.bin` files
- Convert files between CSV, TXT, and BIN formats
- Compare two files (any supported combination) and show differences
- Simple, single‑binary CLI with clear flags

## Requirements
- Rust (stable). If you don’t have it yet:
  - Windows PowerShell:
    ```powershell
    winget install Rustlang.Rustup
    ```
  - Or visit https://rustup.rs

## Getting Started
Clone and build the CLI:

```powershell
# Clone
git clone https://github.com/jekispank98/fin_converter_cli.git
cd fin_converter_cli

# Build (debug)
cargo build

# Or run directly
cargo run -- --help
```

The dependency `fin_converter_lib` is fetched from GitHub automatically (via Cargo) the first time you build or run.

## Usage
The CLI is powered by `clap`, so `--help` shows all options:

```text
Usage: fin_converter_cli [OPTIONS]

Options:
  -p, --path <PATH>                    Path to the source file
  -c, --path-to-compare <PATH>         Second file path (required for compare)
  -a, --action <ACTION>                Action to perform [default: read] [possible values: read, convert, compare]
  -t, --to-format <FORMAT>             Target format for convert [default: csv] [possible values: txt, bin, csv]
  -h, --help                           Print help
  -V, --version                        Print version
```

### Actions
- `read` (default): parse a file and print its records in a readable form.
- `convert`: parse a file and write it out in the requested format.
- `compare`: parse two files and display differences (by records).

### Examples
Read a CSV file:
```powershell
cargo run -- -p ./data/transactions.csv -a read
```

Convert a TXT file to CSV:
```powershell
cargo run -- -p ./data/statement.txt -a convert -t csv
```

Convert a CSV file to BIN:
```powershell
cargo run -- -p ./data/statement.csv -a convert -t bin
```

Compare two files (any supported formats):
```powershell
cargo run -- -p ./data/january.csv -c ./data/january.txt -a compare
```

Output files are created next to the source with the appropriate extension (e.g., `input.csv` → `input.txt`).

## Supported Formats
- CSV: comma‑separated values, one record per line
- TXT: plain text serialization supported by `fin_converter_lib`
- BIN: compact binary serialization supported by `fin_converter_lib`

Note: Exact record schema is defined in the `fin_converter_lib` models (e.g., timestamps, amount, etc.). This CLI focuses on I/O and orchestration.

## Error Messages & Logging
- The tool prints helpful diagnostics (e.g., when a file doesn’t exist or the extension isn’t supported).
- Unsupported or missing extensions are reported as parse/format errors.

## Development
- Toolchain: Rust 1.81+ recommended (Cargo will use your installed stable).
- Build: `cargo build` or `cargo run -- ...`
- Lint (optional): if you use `rustfmt`/`clippy`:
  ```powershell
  cargo fmt
  cargo clippy -- -D warnings
  ```

## Contributing
Issues and pull requests are welcome! If you change the public CLI (flags or actions), please update the examples above and add a short note to the README.

## License
MIT. See the repository’s `LICENSE` file if present, or the license header in `Cargo.toml`.
