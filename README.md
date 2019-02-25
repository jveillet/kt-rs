# kt-rs

A drop-in cat replacement written in Rust.

## Disclaimer

This project is purely educationnal and serve as an example for a blog post on [demainilpleut](https://www.demainilpleut.fr/).

It is not mean to be a fully functionnal software and does not come with any guarantees of some sort.

## Prerequisites

This project uses `Rust` and `Cargo`, so make sure that you have the [Rust toolchain](https://rustup.rs/) installed on your environment.

## Installation

```
$ cd Development/
$ git clone git@github.com:jveillet/kt-rs.git
$ cd kt-rs/
$ cargo build --release
$ cargo install --path .
```

## Usage

```
$ kt --help
kt 0.1.0
Jérémie Veillet. @jveillet
A drop-in cat replacement written in Rust

USAGE:
    kt [FILE]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    File to print.
```
