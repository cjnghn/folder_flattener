# Folder Flattener

Folder Flattener is a CLI tool written in Rust that flattens directory structures while preserving original file paths.

## Why This Tool?

This tool was primarily developed to address the lack of folder upload functionality in AI code assistant platforms like Claude projects. Many AI platforms don't yet support uploading entire folder structures, making it challenging for developers to share complete projects with AI assistants.

With Folder Flattener, you can:
1. Flatten complex project structures into a single directory.
2. Preserve original file paths as comments at the top of each file, maintaining the relationship between files.
3. Easily upload flattened projects to AI platforms, providing context for the entire codebase.

## Key Features

- Flattens directory structures
- Preserves original file paths (added as comments at the top of each file)
- Ignores files/directories using a `.flattenerignore` file
- Uses appropriate comment styles based on file types
- Displays progress during operation

## Installation

(Add installation instructions here)

## Usage

```
folder-flattener -s <source_directory> -o <output_directory>
```

Options:
- `-s, --source`: Source directory to flatten
- `-o, --output`: Output directory for flattened files
- `-v, --verbose`: Increase verbosity (can be used multiple times, e.g., -v, -vv, -vvv)

## Ignoring Files

To ignore certain files or directories, create a `.flattenerignore` file in your source directory. This file uses the same format as `.gitignore`. For example:

```
# Ignore all .log files
*.log

# Ignore the node_modules directory
node_modules/

# Ignore a specific file
secrets.txt

# Don't ignore an important file
!important.txt
```

## Example

```bash
folder-flattener -s ./my_project -o ./flattened_project
```

This command flattens all files from the `my_project` directory into the `flattened_project` directory, ignoring any files or directories specified in `./my_project/.flattenerignore` if it exists.

## Contributing

Contributions are welcome! If you'd like to contribute, please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
