![Rust](https://github.com/Gogopex/psswd/workflows/Rust/badge.svg?branch=master&event=push)

# psswd
A simple Rust command line utility to manage your passwords. 
It uses [age (Actually Good Encryption)](https://github.com/FiloSottile/age) to encrypt your passwords, and supports decryption using a passphrase.

## Description
@TODO

## Installation
```
git clone https://github.com/Gogopex/psswd.git
cd psswd
# psswd requires cargo/rustc
cargo build --release
```
This will generate a bin file in `target/release/build`. 

## Usage
If you've just ran `cargo build --release`, you can start using `./target/release/psswd <command>` or add an alias for `psswd` direcly.

## Demo
![Usage of psswd](https://i.imgur.com/THgxjV6.gif)

## Help
```bash
⚡ psswd
psswd 0.1.1

USAGE:
    psswd <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Creates a new entry containing an encrypted password
    delete    Deletes entries or a given entry
    help      Prints this message or the help of the given subcommand(s)
    list      Lists all entries
    show      Displays a specific entry

Feel free to report any issue you find here: https://github.com/Gogopex/psswd/issues
```
