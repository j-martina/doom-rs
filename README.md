# doom-rs

## About

This repository is where I develop Rust technologies related to Doom and Doom source ports. Ideally, these libraries can eventually be leveraged as part of future efforts to create Doom source ports and tooling, such as content editors or a language server.

### doomfront

`doomfront` aims to be a comprehensive suite of language frontends for the myriad of domain-specific languages recognised by the collective ecosystem of Doom source ports, including those of the [ZDoom](https://zdoom.org/index) family, [Eternity Engine](https://eternity.youfailit.net/wiki/Main_Page), ACS, DeHackEd, and UMAPINFO.

`doomfront` uses the `rowan` crate (see the attributions section) - which itself serves as the foundation for [rust-analyzer](https://rust-analyzer.github.io/) - to generate lossless syntax trees that are completely representative of the parsed source and easy to traverse.

### stardate

`stardate` aims to provide a single API for encoding and decoding all existing standards of Doom maps.

### subterra

`subterra` aims to be a parser for the [MUS file format](https://doomwiki.org/wiki/MUS), a modified MIDI specification. Strictly speaking, this format is for the [DMX](https://doomwiki.org/wiki/DMX) sound library, and not only a part of the Doom ecosystem, but in the present day the principal reason to support this format at all is Doom.

If any other Doom-specific sound- or music-related functionality reveals itself to be missing from the Rust ecosystem, it will go in here.

## Credits, Attributions, and Licensing Details

All libraries herein are provided under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option. The license chosen covers all source within these libraries, which can be assumed to be original unless explicitly specified otherwise.

---

### doomfront

- arrayvec
    - By [@bluss](https://github.com/bluss) et al.
    - Provided under the [Apache 2.0 License](https://github.com/bluss/arrayvec/blob/master/LICENSE-APACHE) and [MIT License](https://github.com/bluss/arrayvec/blob/master/LICENSE-MIT).
    - https://docs.rs/arrayvec/latest/arrayvec/
- chumsky
    - By Joshua Barretto ([@zesterer](https://github.com/zesterer)) et al.
    - Provided under the [MIT License](https://github.com/zesterer/chumsky/blob/master/LICENSE).
    - https://docs.rs/chumsky/latest/chumsky/
- rowan
    - By Alex Kladov ([@matklad](https://github.com/matklad)) et al.
    - Provided under the [Apache 2.0 License](https://github.com/rust-analyzer/rowan/blob/master/LICENSE-APACHE) and [MIT License](https://github.com/rust-analyzer/rowan/blob/master/LICENSE-MIT).
- serde
	- By the [serde-rs](https://github.com/serde-rs) organisation et al.
	- Provided under the [Apache 2.0 License](https://github.com/serde-rs/serde/blob/master/LICENSE-APACHE) and [MIT License](https://github.com/serde-rs/serde/blob/master/LICENSE-MIT).
	- https://docs.rs/serde/latest/serde/
- smallvec
    - By Matt Brubeck ([@mbrubeck](https://github.com/mbrubeck)) et al.
    - Provided under the [Apache 2.0 License](https://github.com/servo/rust-smallvec/blob/master/LICENSE-APACHE) and [MIT License](https://github.com/servo/rust-smallvec/blob/master/LICENSE-MIT).
    - https://docs.rs/smallvec/latest/smallvec/
