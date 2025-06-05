# NumEx (Number Expression Calculator)

A lightweight command-line utility for number base conversion written in Rust.

## Overview

NumEx is a simple yet practical tool designed to convert numbers between
different bases (hexadecimal, decimal, etc.) with an intuitive syntax. It was
developed as a learning project for the Rust programming language while solving
a real-world need.

## Usage

```bash
nmx <NUMBER>
```

Where `<NUMBER>` can be:
- A decimal number (e.g., `42`, `-123`)
- A hexadecimal number (e.g., `0x2A`, `0xffff`)

Type `nmx -h` to see how to use nmx:

```
❯ nmx -h
Usage: nmx [OPTIONS] <NUMBER>

Arguments:
  <NUMBER>

Options:
  -m, --monochrome  Display result without colors
  -h, --help        Print help
  -V, --version     Print version
```

## Comparison with Alternatives

While similar tools exist, NumEx focuses on simplicity and intuitive syntax:

### bc (Basic Calculator)
``` bash
echo "ibase=16; FFF"|bc    # Traditional bc approach
nmx 0xfff                # NumEx equivalent
```

### Benefits
- More intuitive syntax
- Less typing required
- No need to remember complex notation
- Straightforward command-line interface

## Project Goals
- Provide a user-friendly number conversion utility
- Serve as a practical Rust learning project
- Simplify common number base conversion tasks


## Installation

Ensure you have Rust installed on your system. Then clone this repository and build using cargo:

```bash
cargo build --release
```
The binary will be available at `target/release/nmx`.

Install with cargo like:

```bash
cargo install --path .
```

### Examples

Converting hexadecimal number 0xcafe:

```bash
❯ nmx 0xcafe
Dec:    51966
Hex:    0xcafe
Bin:    0b1100101011111110
```

## Error Handling

The program will display an error message if:
- The input number format is invalid
- The number is too large or small for a 64-bit signed integer
- The input contains invalid characters

## Dependencies

- clap (4.5.39) - Command line argument parsing
- anyhow (1.0.98) - Error handling
- regex (1.11.1) - Pattern matching for number validation
- colored (3.0.0) - Terminal coloring utilities

## License

MIT License
