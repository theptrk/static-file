static-file [![Build Status](https://secure.travis-ci.org/iron/static-file.png?branch=master)](https://travis-ci.org/iron/static-file)
====

> Static file-serving middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate iron;
extern crate http;
use iron::{Iron, ServerT, Chain, Request, Response, Alloy};

fn main() {
    let mut server: ServerT = Iron::new();
    server.chain.link(hello_world); // Add middleware to the server's stack
    server.listen(::std::io::net::ip::Ipv4Addr(127, 0, 0, 1), 3000);
}

fn hello_world(_: &mut Request, res: &mut Response, _: &mut Alloy) {
    res.serve(::http::Ok, "Hello, world!");
}
```

## Overview

static-file is a part of Iron's [core bundle](https://github.com/iron/core).

- ...
- ...

## Installation

If you're using a `Cargo` to manage dependencies, just add static-file to the toml:

```toml
[dependencies.staticfile]

git = "https://github.com/iron/static-file.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://docs.ironframework.io/core/staticfile)

Along with the [online documentation](http://docs.ironframework.io/core/staticfile),
you can build a local copy with `make doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
