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
