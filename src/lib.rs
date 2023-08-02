//! [![MASTER CI status](https://github.com/{{github_name}}/{{crate_name}}-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/{{github_name}}/{{crate_name}}-rs/actions/workflows/ci.yml?query=branch%3Amaster)
//! [![crates.io badge](https://img.shields.io/crates/v/{{crate_name}})](https://crates.io/crates/{{crate_name}})
//! [![docs.rs badge](https://img.shields.io/docsrs/{{crate_name}}?label=docs.rs)](https://docs.rs/{{crate_name}})
//! [![dependencies badge](https://img.shields.io/librariesio/release/cargo/{{crate_name}})](https://libraries.io/cargo/{{crate_name}})

#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]

// #![warn(missing_docs)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

{% if proc_macro %}
use proc_macro::TokenStream as BaseTokenStream;

#[proc_macro_derive(Foo, attributes(foo))]
pub fn derive_potatoes(input: BaseTokenStream) -> BaseTokenStream {
  todo!()
}
{% else %}
pub const fn take_over_the_world() -> bool {
    false
}
{% endif %}
