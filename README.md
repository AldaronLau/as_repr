# As Repr

[![tests](https://github.com/AldaronLau/as_repr/actions/workflows/ci.yml/badge.svg)](https://github.com/AldaronLau/as_repr/actions/workflows/ci.yml)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/AldaronLau/as_repr)](https://github.com/AldaronLau/as_repr)
[![GitHub contributors](https://img.shields.io/github/contributors/AldaronLau/as_repr)](https://github.com/AldaronLau/as_repr/graphs/contributors)  
[![Crates.io](https://img.shields.io/crates/v/as_repr)](https://crates.io/crates/as_repr)
[![Crates.io](https://img.shields.io/crates/d/as_repr)](https://crates.io/crates/as_repr)
[![Crates.io (recent)](https://img.shields.io/crates/dr/as_repr)](https://crates.io/crates/as_repr)  
[![Crates.io](https://img.shields.io/crates/l/as_repr)](https://github.com/search?q=repo%3AAldaronLau%2Fas_repr+path%3A**%2FLICENSE*&type=code)
[![Docs.rs](https://docs.rs/as_repr/badge.svg)](https://docs.rs/as_repr/)

Rust trait for constant `#[repr(T)]` conversions 

Check out the [documentation] for examples.

### Features

 - Const trait workaround for stable Rust exposing a safe transmute to the
   `repr` type with a trait
 - Optionally-enabled const trait workarounds for generic integer and float
   operations and comparisons on the inherent representation
   - _**`inherent`**_: Providing `AsReprInherent` built on `AsRepr` but chooses
     a specific representation to interpret as for const operations
   - _**`cmp`**_: Providing const fns generic for comparable types
   - _**`float`**_: Providing const fns generic for floating point types
   - _**`int`**_: Providing const fns generic for integer types
   - _**`num`**_: Providing const fns generic for numeric types (floats and
     ints)
   - _**`ops`**_: Providing const fns generic for mathematical types (numeric
     types and `Duration`s)

## MSRV

The current MSRV is Rust 1.91.

Any future MSRV updates will follow the [Ardaku MSRV guidelines].

## License

Copyright © 2025-2026 The As Repr Contributors.

Licensed under any of
 - Apache License, Version 2.0, ([LICENSE\_APACHE] or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 - Boost Software License, Version 1.0, ([LICENSE\_BOOST] or
   <https://www.boost.org/LICENSE_1_0.txt>)
 - MIT License, ([LICENSE\_MIT] or <https://mit-license.org/>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## AI / LLM (Non-)Usage

This project was developed without using AI tooling.  AI usage for contributions
is strictly banned, with the exception of AI tooling trained on exclusively CC0,
Unlicense, or equivalently-licensed code.  By opening a PR on this repository,
you assert that the work (excluding any test data) is either human generated,
computer generated trained on works in the public domain, or a combination of
the two.  AI bug reports are welcome as issues, as long as there is a disclaimer
that it was discovered with AI.

## Help

If you want help using or contributing to this library, feel free to send me an
email at <aldaronlau@gmail.com>.

[Ardaku MSRV guidelines]: https://github.com/ardaku/.github/blob/v1/profile/MSRV.md
[LICENSE\_APACHE]: https://github.com/AldaronLau/as_repr/blob/v1/LICENSE_APACHE
[LICENSE\_BOOST]: https://github.com/AldaronLau/as_repr/blob/v1/LICENSE_BOOST
[LICENSE\_MIT]: https://github.com/AldaronLau/as_repr/blob/v1/LICENSE_MIT
[documentation]: https://docs.rs/as_repr
