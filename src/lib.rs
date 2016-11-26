//! easily build compiler plugins.
//!
//! # Examples
//!
//! ```html
//! #![feature(quote, plugin_registrar, rustc_private)]
//! extern crate syntax;
//! extern crate ecp;
//! use ecp::prelude::*;
//!
//! #[plugin_registrar]
//! pub fn register(reg: &mut Registry) {
//!     reg.register_macro("test", test);
//! }
//!
//! pub fn test(cx: &mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'static> {
//!     MacEager::expr(quote_expr!(cx, println!("Hello world!")))
//! }
//! ```
//!
#![deny(missing_docs)]
#![feature(quote, plugin_registrar, rustc_private)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc_plugin;

/// Prelude for compiler plugin.
pub mod prelude;
/// Builders.
pub mod builder;
/// Compiler plugin utils.
pub mod utils;
