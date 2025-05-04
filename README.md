# Repo Hoarder

Repo Hoarder is a CLI tool designed to mass clone repositories from Codeberg. It supports asynchronous cloning with multiple threads, making it efficient for downloading repositories from users or organizations.

## Features

- Clone all repositories from a Codeberg user or organization.
- Multi-threaded cloning for improved performance.
- Option to clone repositories recursively.
- Customizable output directory for cloned repositories.

## Installation

To use Repo Hoarder, you need to have Rust installed. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

1. Clone this repository:
   ```bash
   git clone https://github.com/ofluffydev/repo-hoarder.git
   cd repo-hoarder
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be available in the `target/release` directory:
   ```bash
   ./target/release/repo-hoarder --help
   ```

## Usage

Run the tool with the following options:

```bash
repo-hoarder [OPTIONS] <TARGET>
```

### Arguments

- `<TARGET>`: The target user or organization on Codeberg.

### Options

- `-t, --threads <THREADS>`: Number of threads to use (defaults to the number of CPU cores).
- `-o, --org`: Specify if the target is an organization.
- `-r, --recursive`: Clone repositories recursively.
- `-c, --clone-output <CLONE_OUTPUT>`: Specify an output directory for cloned repositories.

### Examples

1. Clone all repositories from a user:
   ```bash
   repo-hoarder ofluffydev
   ```

2. Clone all repositories from an organization with 8 threads:
   ```bash
   repo-hoarder -t 8 -o codeberg
   ```

3. Clone all repositories recursively into a specific directory:
   ```bash
   repo-hoarder -r -c ./cloned-repos ofluffydev
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.

## Acknowledgments

- [Codeberg](https://codeberg.org) for providing an open-source platform for hosting repositories.
- The Rust community for their excellent libraries and tools.
