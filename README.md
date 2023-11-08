# Bech32 Utils

`bech32-utils` is a command-line tool written in Rust that allows you to transform any Bech32 prefixed address into another address with a different prefix. It's designed to be simple, efficient, and easy to use.

## Installation

To install `bech32-utils`, you need to have Rust and Cargo installed on your machine. If you don't have them installed, please follow the instructions here: [Install Rust](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, you can install `bech32-utils` by cloning the repository and building the project:

```sh
git clone https://github.com/mbbrainz/bech32-utils.git
cd bech32-utils
cargo build --release
```

The built binary will be located at `target/release/bech32-utils`.

## Usage

To use `bech32-utils`, you need to provide the original Bech32 address and the prefix you want to convert it to. The tool also accepts an optional derivation path.

```sh
./bech32-utils <address> <new_prefix> [derivation_path]
```

### Parameters:

- `<address>`: The original Bech32 address you want to convert.
- `<new_prefix>`: The new prefix for the address.
- `[derivation_path]`: Optional. The derivation path if applicable.

### Examples:

Convert an address to a new prefix:

```sh
./bech32-utils bc1qp... zil1
```

Convert an address to a new prefix with a derivation path:

```sh
./bech32-utils bc1qp... zil1 m/44'/0'/0'/0/0
```

## Contributing

Contributions are what make the open-source community such a fantastic place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

Don't forget to give the project a star! Thanks again!

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Maurits Bos - maurits.bos@gmail.com

Project Link: [https://github.com/mbbrainz/bech32-utils](https://github.com/mbbrainz/bech32-utils)