# Deta Rust Library (SDK)

This library is at a very initial stage. Not been published to [`crates.io`](https://crates.io).

To use this library, add the following under the `dependencies` section in `Cargo.toml`.

```
deta = { git = "https://github.com/emmanuelantony2000/deta-rust" }
```

To test the library, clone this repo and run: (Ensure that the API key is available as an environment variable, under the name `DETA_PROJECT_KEY`)

```
cargo t
```

To render out the documentation for the library, clone this repo and run:

```
cargo rustdoc --open
```