This example code generates Typescript types from types in Rust source code,
written for my [blog post on the subject](https://imfeld.dev/writing/generating_typescript_types_from_rust).
This can be useful when you have a server written in Rust, but want to automatically synchronize types with
frontend Typescript code.

It generates JSON schema files from the Rust source, and then Typescript types from that.

This example assumes that you have the `cargo-watch` crate installed globally. `cargo install cargo-watch` will install it
if you haven't it already.
