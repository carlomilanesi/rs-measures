# rs-measures

[![Crates.io](https://img.shields.io/crates/v/rs-measures.svg)](https://crates.io/crates/rs-measures)
[![Docs.rs](https://docs.rs/rs-measures/badge.svg)](https://docs.rs/rs-measures)
[![CI](https://github.com/carlomilanesi/rs-measures/workflows/Continuous%20Integration/badge.svg)](https://github.com/carlomilanesi/rs-measures/actions)
[![Coverage Status](https://coveralls.io/repos/github/carlomilanesi/rs-measures/badge.svg?branch=master)](https://coveralls.io/github/carlomilanesi/rs-measures?branch=master)

## Description

This repository contains the source code and the documentation of two Rust-language crates: `rs-measures` and `units-relation`.

Their purpose is to improve the readability and correctness of applications using numeric values having units of measurement.
These can be the ones used in physics or in geometry, but also the ones commonly used in industry.

This purpose is achieved by encapsulating such numbers into objects whose type represents their unit of measurement, and providing for such types only the operations which make sense.

The documentation is in these files:
* [Motivation](docs/Motivation.md). It describes the advantages of using these crates instead of other crates or naked numbers.
* [Tutorial](docs/Tutorial.md). It is a step-by-step course on the use of these crates.
* [Architecture](docs/Architecture.md). It explains the design choices of the crates.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
