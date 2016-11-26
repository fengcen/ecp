
ecp
===
easily build compiler plugins.

## [Document](http://fengcen.github.io/ecp)

## Prerequisites
ecp required latest **Nightly** Rust.

## Why use "compiler plugin" instead of "Macros 1.1"?
I need not only "custom derive", but also more "compiler plugin" features.
Currently "Macros 1.1" is also unstable.

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
