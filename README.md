
ecp
===
[![Project Status: WIP - Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](http://www.repostatus.org/badges/latest/wip.svg)](http://www.repostatus.org/#wip)

easily build compiler plugins.

## Document
Sorry, you need to build the document with `cargo doc`.

## Prerequisites
ecp required latest **Nightly** Rust.

## Why use "compiler plugin" instead of "Macros 1.1"?
I need not only "custom derive", but also more "compiler plugin" features.
~~Currently "Macros 1.1" is also unstable.~~

"Macros 1.1" has been stabilize in Rust 1.15, so you should use it.

I will consider adding support for [syn](https://crates.io/crates/syn).

## Usage
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

## Examples

Create a simple macro which prints "Hello world!":

```
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

## License
ecp is primarily distributed under the terms of the MIT license.
See [LICENSE](LICENSE) for details.
