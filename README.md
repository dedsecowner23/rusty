# Rusty Luau Obfuscator

Rusty is a small Luau obfuscator written in Rust. It demonstrates simple techniques for transforming Luau scripts while preserving behavior.

## Features

- Variable and function renaming using random Unicode.
- XOR string encryption with runtime decryption helper.
- Constant rewriting for numeric literals.
- Dead code insertion as dummy blocks.

## Building

```bash
cargo build --release
```

## Usage

```bash
./target/release/rusty < input.luau > output.luau
```

For more details see [docs/USAGE.md](docs/USAGE.md).

## License

MIT
