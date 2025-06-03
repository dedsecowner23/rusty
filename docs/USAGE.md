# Usage Guide

The obfuscator reads a Luau file from standard input and writes the obfuscated version to standard output.

```bash
./target/release/rusty < input.luau > output.luau
```

Obfuscated code contains a small decoding routine for encrypted strings and may include dummy code blocks. The resulting script should run the same as the original within Roblox Studio or any Luau interpreter.
