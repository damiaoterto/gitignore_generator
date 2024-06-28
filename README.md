# .gitignore Generator CLI
A simple command-line interface (CLI) tool to generate `.gitignore` files.

## Overview
This tool allows you to generate `.gitignore` files by specifying templates. It fetches the templates from a predefined API and combines them into a single `.gitignore` file.

## Installation
To install this CLI tool, clone the repository and build the project using Cargo:

```sh
git clone https://github.com/yourusername/gitignore-generator.git
cd gitignore-generator
cargo build --release
cargo install --path .
```

## Usage
The CLI accepts two parameters:
- `-t` or `--template`: Specifies the name of the templates to use. Multiple templates can be specified by separating them with commas (e.g., `node,python).

- `-f` or `--filename`: Specifies the name of the output file. Defaults to .gitignore` if not provided.

## Examples
Generate a .gitignore file for Node.js:

```sh
gig -t node
```

Generate a .gitignore file for both Node.js and Python:
```sh
gig -t node,python
```

Generate a .gitignore file for Node.js and save it to custom.gitignore:
```sh
gig -t node -f custom.gitignore
```

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Acknowledgements
This project uses the following crates:
- [clap](https://github.com/clap-rs/clap)
- [inquire](https://github.com/mikaelmello/inquire)
- [reqwest](https://github.com/seanmonstar/reqwest)
