# Hack The Valley CLI
CLI for HTV administrators.

## Build

To build, you must have Rust and Cargo package manager installed, then run following commands:

```
$ cargo run # If you wish to debug
$ cargo build # If you wish to build binary
```

## Basic Usage

```
USAGE:
    htv-cli [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --key <KEY>    Sets the API key to use, required if accessing private resources.
    -u, --url <URL>    Optionally sets the API url to use, default https://api.hackthevalley.io.

SUBCOMMANDS:
    api-ver    Check API version
    hackers    Fetch hackers
    help       Prints this message or the help of the given subcommand(s)
```