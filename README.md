# Luhn Algorithm Implementation in Rust (LAIR)

A lightweight Rust implementation of the [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm), commonly used for validating credit card numbers, IMEI codes, and other identification numbers.

## Features

* Validates numeric strings for Luhn compliance.
* Handles errors for invalid inputs (e.g., non-numeric strings).

## Usage

```txt
Usage: lair <KEY>

Arguments:
  <KEY>  Key to validate (digits only)
```

## Install

### Cargo

```sh
cargo install --git https://github.com/antwxne/LAIR
```

## Testing

To run the tests for this implementation, use the following command:

```sh
cargo test
```
