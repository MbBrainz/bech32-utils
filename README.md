# Bech32 Address Converter

This project provides a library and a command-line interface (CLI) for converting Bech32 addresses with different prefixes. It's written in Rust and is optimized for maximum performance.

## Structure

The project is divided into two main parts:

1. [`bech32-addr-converter/src`](./bech32-addr-converter/src).

2. [``bech32-addr-converter-cli``](./bech32-addr-converter-cli): This is a CLI tool that uses the [``bech32-addr-converter``](./bech32-addr-converter) library to convert addresses. The source code is located in [`bech32-addr-converter-cli/src`](./bech32-addr-converter-cli/src).

## Install

install the cli using cargo: 

```sh
cargo install bech32-addr-converter-cli
```

or add the converter library to your project 
```sh
cargo add bech32-addr-converter
```

## Usage

To use the CLI tool, you need to build the project first. After building, you can run the [``bech32-addr-converter``](./bech32-addr-converter) binary with the appropriate arguments. try `bech32-addr-converter-cli -h` for description.

## Tests

Tests are included in the [`tests`](./bech32-addr-converter/src/converter.rs) module in the [`bech32-addr-converter/src/converter.rs`](./bech32-addr-converter/src/converter.rs) file.

## License

This project is licensed under either of

- MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in [``bech32-addr-converter``](./bech32-addr-converter) 
