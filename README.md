ecp
===

[![Project Status: Abandoned – Initial development has started, but there has not yet been a stable, usable release; the project has been abandoned and the author(s) do not intend on continuing development.](https://www.repostatus.org/badges/latest/abandoned.svg)](https://www.repostatus.org/#abandoned)

easily build compiler plugins.

Document
--------

Sorry, you need to build the document with `cargo doc`.

Prerequisites
-------------

ecp required latest **Nightly** Rust.

Why use "compiler plugin" instead of "Macros 1.1"?
--------------------------------------------------

I need not only "custom derive", but also more "compiler plugin" features.
~~Currently "Macros 1.1" is also unstable.~~

"Macros 1.1" has been stabilize in Rust 1.15, so you should use it.

I will consider adding support for [syn](https://crates.io/crates/syn).

Usage
-----

Add dependencies to Cargo.toml

```toml
[dependencies]
ecp = "^0.1"
```

In your `main.rs` or `lib.rs`:

```rust
#![feature(quote, plugin_registrar, rustc_private)]
extern crate syntax;
extern crate ecp;
```

Examples
--------

Create a simple macro which prints "Hello world!":

```rust
#![feature(quote, plugin_registrar, rustc_private)]
extern crate syntax;
extern crate ecp;
use ecp::prelude::*;

#[plugin_registrar]
pub fn register(reg: &mut Registry) {
    reg.register_macro("test", test);
}

pub fn test(cx: &mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'static> {
    MacEager::expr(quote_expr!(cx, println!("Hello world!")))
}
```

License
-------

ecp is primarily distributed under the terms of the MIT license.
See [LICENSE](LICENSE) for details.
