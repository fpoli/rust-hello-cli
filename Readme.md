Hello World CLI
===============

[![Build Status](https://travis-ci.org/fpoli/rust-hello-cli.svg?branch=master)](https://travis-ci.org/fpoli/rust-hello-cli)

Prints a nice 'Hello World!' message.


Install
-------

```
cargo install hello-cli
```


Examples
--------
```
$ hello
Hello World!
```

```
$ hello -n MyName
Hello MyName!
```


Usage
-----

```
USAGE:
    hello [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>    Use name (default: World)
```


Testing
-------

```
cargo test
```
