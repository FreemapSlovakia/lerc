# lerc

Safe and idiomatic Rust wrapper for [Esri's LERC](https://github.com/Esri/lerc) raster compression library.

Provides encoding, decoding, and metadata access for multi-band and multi-dimensional raster data with optional validity masks.

Built on top of [`lerc-sys`](https://crates.io/crates/lerc-sys), with vendored LERC C++ source by default. Supports dynamic linking with `--no-default-features`.

## License

MIT OR Apache-2.0
