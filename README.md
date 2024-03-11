# ✨📄 Sparkle Template

Opinionated template for [Rust](https://www.rust-lang.org) projects using [Twilight](https://api.twilight.rs)

## ✨ Features

### Common Twilight Crates Included

Boilerplate for every Twilight crate with every feature enabled is in this template, you can simply remove the things
you don't need

For Twilight Gateway and Twilight HTTP, `rustls-webpki-roots`, `simd-json`, `zlib-simd` and `trust-dns` are enabled

These crates aren't included because they aren't used very often:

- `twilight-gateway-queue`
- `twilight-http-ratelimiting`
- `twilight-lavalink`

### Context

A `Context` struct is given to share stateful data throughout the program, such as `twilight_http::Client`

The struct is a wrapper around `Arc<ContextInner>` and implements `Clone`, and `Deref` to `ContextInner`, this provides
an abstraction over the `Arc` management while making `Context` cheap to clone and safe to use in multi-threads

### Error Handling with Anyhow

[`anyhow`](https://docs.rs/anyhow) is used to handle errors in a polymorphic way

The executable uses `anyhow` to create ad-hoc errors

Panics and errors also include backtraces

### Configuration with Environment Variables

Environment variables are used to set info such as the bot's token

A `Config` struct is provided to centralize all the configuration, which has a constructor that loads it from
environment variables

Support for `.env` files is added with the [`dotenvy`](https://docs.rs/dotenvy) crate

### Tracing

[`tracing-subscriber`](https://docs.rs/tracing-subscriber) crate is configured to use the environment
variable `RUST_LOG` to print [`tracing`](https://docs.rs/tracing) messages

`tracing` is also used for the messages generated in the executable

Compatibility between [`log`](https://docs.rs/log) and `tracing` is also provided
with [`tracing-log`](https://docs.rs/tracing-log)

## 🤔 Usage

1. Click on _Use this template_ in the GitHub web UI
    - > Or copy-paste the code in some other way
2. Replace occurrences of `FILLME` with the actual values

## 🙋 Wanted: Issues

Feel free to create an issue or PR for suggestions
