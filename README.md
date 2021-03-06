# varmint [![travis-badge][]][travis] [![cargo-badge][]][cargo] ![license-badge][]

A Rust implementation of the varint codec as used in [Google's Protocol
Buffers][protobuf]. Adds `read_*`/`write_*` methods for various sizes of varints
on top of the standard IO traits.

## Developing

This project uses [clippy][] and denies warnings in CI builds. To ensure your
changes will be accepted please check them with `cargo clippy` (available via
`cargo install clippy` on nightly rust) before submitting a pull request (along
with `cargo test` as usual).

Both the nightly date and clippy version used in CI are pinned in the
`.travis.yml` as clippy sometimes breaks for a few days after a new nightly is
published, feel free to update to a new known good pair anytime as part of a
pull request.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[travis-badge]: https://img.shields.io/travis/mycorrhiza/varmint-rs/master.svg?style=flat-square
[travis]: https://travis-ci.org/mycorrhiza/varmint-rs
[cargo-badge]: https://img.shields.io/crates/v/varmint.svg?style=flat-square
[cargo]: https://crates.io/crates/varmint
[license-badge]: https://img.shields.io/badge/license-MIT/Apache--2.0-lightgray.svg?style=flat-square

[protobuf]: https://developers.google.com/protocol-buffers/docs/encoding#varints
[clippy]: https://github.com/Manishearth/rust-clippy
