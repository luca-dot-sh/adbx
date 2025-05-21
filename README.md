# ADBX - ADB eXtended

ADBX is a command line tool that extends the functionality of ADB with additional features and autocompletion for bash.

## Building

To build ADBX, you can use the following command:

```bash
cargo build --release
```

This will build the ADBX binary in the release mode.


## Installation

To install ADBX, you can use the following command:

```bash
cp target/release/adbx /usr/local/bin && chmod +x /usr/local/bin/adbx && adbx autocomplete > /usr/local/share/bash-completion/completions/adbx && source /usr/local/share/bash-completion/completions/adbx
```

This will copy the ADBX binary to `/usr/local/bin`, make it executable, generate the autocompletion script for bash, and source it.

## Usage

To use ADBX and see the available commands, you can use the following command:

```bash
adbx --help
```
