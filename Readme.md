# head_rust

**head_rust** is a Rust implementation of the `head` command, a standard Unix utility for displaying the beginning of a file.

## Dependencies

This program has been developed using Rust version 1.74.0. Command-line argument management has been simplified thanks to the use of the crate clap in its version 4.4.11. clap offers a declarative interface for defining and parsing program arguments, making it easy to create a friendly and efficient user experience.

## Installation

1. Clone the repository: `git clone https://github.com/tuusuario/head_rust.git`
2. Navigate to the project directory: `cd head_rust`
3. Compile the project: `cargo build --release`

## Options

- `-c, --bytes`: Print specified bytes of the file.
- `-n, --lines`: Print specified lines of the file.
- `-h, --help`: Show the program's help.
- `-V, --version`: Show the program's version.

## Usage

To use `head_rust`, you can compile the program and run it from the command line. It supports various options to customize the output:

```bash
# Example usage with options
cd target/release
./head-rust file1.txt file2.txt -c 100 -n 5
```
