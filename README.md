# bgrep - binary grep

## Description

Want a simple tool for searching for binary patterns in a file? This does that.

## Install

*Note*: Requires rust to be installed on the target system

```
cargo build --release
```

The output binary should be in `./target/release/bgrep`

## Usage

```
bgrep <hex pattern> <input file>
```

## Example

```
$ bgrep "00 01" my_file
Found match in my_file
```

## TODO

- [ ] Optimize reads. Currently it just reads `pattern.len()` bytes and does a straight comparison.
- [ ] Add support for reading from stdin
