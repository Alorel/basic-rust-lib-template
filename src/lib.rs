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
