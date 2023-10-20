# Sieve Parser

[![build](https://img.shields.io/github/actions/workflow/status/plivox/sieve-parser/test.yml?branch=main&style=flat-square&logo=github)](https://github.com/plivox/sieve-parser/actions)
[![crates.io](https://img.shields.io/crates/v/sieve-parser?style=flat-square&logo=rust)](https://crates.io/crates/sieve-parser)
[![docs.rs](https://img.shields.io/badge/docs.rs-sieve--parser-blue?style=flat-square&logo=docs.rs)](https://docs.rs/sieve-parser)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

_Sieve Parser_ is a fast parser and AST exporter implemented in Rust for the [Sieve](https://datatracker.ietf.org/doc/html/rfc5228) language.

## Quick start for development

Read and follow the instructions in [Developing inside a Container](.devcontainer/README.md).

## Build

    make build

## Usage (CLI)

With `-f|--file` argument:

    sieve-parser --file resources/assets/tricky-rule1.sieve

With stdin:

    cat resources/assets/tricky-rule1.sieve | sieve-parser

> or `sieve-parser < resources/assets/tricky-rule1.sieve`

## Testing

    cargo test

## License

Sieve Parser is distributed under [AGPL-3.0-only](LICENSE).
