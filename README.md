# Folder Flattener

A Rust CLI tool to flatten directory structures while preserving file paths.

## Features

- Flattens complex directory structures
- Preserves original file paths as comments
- Supports `.flattenerignore` for custom ignore patterns
- Provides progress feedback

## Installation

```
git clone https://github.com/cjnghn/folder-flattener.git
cd folder-flattener
cargo build --release
```

The binary will be available at `./target/release/folder-flattener`

## Usage

Basic usage:
```
folder-flattener --source <SOURCE_DIR> --output <OUTPUT_DIR>
```

With a custom ignore file:
```
folder-flattener --source <SOURCE_DIR> --output <OUTPUT_DIR> --ignore-file <IGNORE_FILE>
```

### .flattenerignore

Create a `.flattenerignore` file in your source directory to specify ignore patterns:

```
*.log
node_modules/
secrets.txt
```

## Contributing

Contributions are welcome. Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.