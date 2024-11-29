# brainf

[Brainfuck](https://wikipedia.org/wiki/Brainfuck) interpreter in rust (btw).

## Usage

```bash
$ cargo build release
```

```bash
$ ./target/release/brainf --help
Usage: ./target/release/brainf [ARGS] <FILE>

Where <FILE> is the bf file you want to execute, and [ARGS] are:

    -n CAP      Make the byte array have CAP elements (default: 30000)
    -A          Print raw digits (separated by space) instead of ASCII
    -h, --help  Print out this text
```
