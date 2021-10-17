# cf-linux

## Usage
```
USAGE:
    cf-linux [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    install    Install a new modpack
    list       List the modpacks you have installed
    mods       List the mods installed in a modpack
    run        Run a modpack
```
To get help information about a specific subcommand, use `cf-linux <subcommand> --help`

## Installation
Make sure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed 
```
cargo install cf-linux
```