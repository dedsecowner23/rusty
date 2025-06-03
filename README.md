# Rusty Luau Obfuscator

Rusty is a small Luau obfuscator written in Rust. It demonstrates simple techniques for transforming Luau scripts while preserving behavior.

## Features

- Variable and function renaming using random Unicode
- XOR string encryption with runtime decryption helper
- Constant rewriting for numeric literals
- Dead code insertion as dummy blocks

## Building

```bash
cargo build --release
```

## Usage

```bash
./target/release/rusty < input.luau > output.luau
```

Additional guides are available in the [docs](docs/) directory:

- [Setup](docs/SETUP.md)
- [Usage](docs/USAGE.md)
- [Configuration](docs/CONFIG.md)
- [Step by Step](docs/STEP_BY_STEP.md)
- [FAQ](docs/FAQ.md)

Example scripts can be found under [examples/](examples).

## License

MIT
