![Rust](https://github.com/Gogopex/psswd/workflows/Rust/badge.svg?branch=master&event=push)

# psswd
A simple Rust command line utility to encrypt and manage your passwords. 
It uses [age](https://github.com/FiloSottile/age) for encryption/decryption. 

## Installation
```
git clone https://github.com/Gogopex/psswd.git
cd psswd
# psswd requires cargo/rustc
cargo build --release
```
This will generate a bin file in `target/release/build`

## Usage
If you've just ran `cargo build --release`, replace `psswd` by `./target/release/psswd` in the following examples.
```bash
# use `psswd add` to add a password
$ psswd add
$ Enter the shortname for your password entry: 
$ youtube
$ Enter a password:
$ Enter a passphrase:

# you can display the plain password by decrypting it with `psswd show`
$ Enter the shortname for the password you want to show:
$ Enter your passphrase:
$ my-plain-password
```

## Demo


## Help
```bash
⚡ psswd
psswd 0.1.0

USAGE:
    psswd <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add
    config
    help      Prints this message or the help of the given subcommand(s)
    list
    show

Feel free to report any issue you find here: https://github.com/Gogopex/wthr/issues
```
