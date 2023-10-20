# Sieve Parser

[![build](https://img.shields.io/github/actions/workflow/status/plivox/sieve-parser/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/plivox/sieve-parser/actions)
[![crates.io](https://img.shields.io/crates/v/sieve-parser?style=flat-square&logo=rust)](https://crates.io/crates/sieve-parser)
[![docs.rs](https://img.shields.io/badge/docs.rs-sieve--parser-blue?style=flat-square&logo=docs.rs)](https://docs.rs/sieve-parser)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

_Sieve Parser_ is a fast parser and AST exporter implemented in Rust for the [Sieve](https://datatracker.ietf.org/doc/html/rfc5228) language.

## Quick start

1. Install Docker:

    [Install Docker](https://docs.docker.com/get-docker/) on your local environment.
    <br/>

2. Developing inside a Container:

    To get started, read and follow the instructions in [Developing inside a Container](https://code.visualstudio.com/docs/remote/containers). The [.devcontainer](.devcontainer) directory contains pre-configured `devcontainer.json` and `Dockerfile` files, which you can use to set up remote development with a docker container.
    <br/>

3. (optional) Export default Rust toolchain (used by the `rust-analyzer` extension):

    ```sh
    export RUSTUP_TOOLCHAIN=<TOOLCHAIN_TO_INSTALL>
    ```

4. (optional) Define the exported variables in the `.env` file:

    ```sh
    make generate-env-from-env-sample > .devcontainer/.env
    ```

5. Open Folder in Container

    Run the Dev Containers: `Open Folder in Container...` command from the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) or quick actions Status bar item, and select the project folder you would like to set up the container for.

    ![](https://code.visualstudio.com/assets/docs/devcontainers/containers/remote-dev-status-bar.png)

6. Run the application:

    Inside the container, run the following command:

    ```sh
    cargo run -- -f resources/assets/rule1.sieve
    ```

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
