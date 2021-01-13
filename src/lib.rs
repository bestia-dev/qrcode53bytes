// region: lmake_md_to_doc_comments include README.md A //!
//! # qrcode53bytes
//!
//! **QR code for 53 bytes**  
//! ***[repo](https://github.com/LucianoBestia/qrcode53bytes); version: 1.1.1  date: 2021-01-13 authors: Luciano Bestia***  
//!
//!  [![crates.io](https://meritbadge.herokuapp.com/qrcode53bytes)](https://crates.io/crates/qrcode53bytes) [![Documentation](https://docs.rs/qrcode53bytes/badge.svg)](https://docs.rs/qrcode53bytes/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/qrcode53bytes.svg)](https://web.crev.dev/rust-reviews/crate/qrcode53bytes/) [![RustActions](https://github.com/LucianoBestia/qrcode53bytes/workflows/rust/badge.svg)](https://github.com/LucianoBestia/qrcode53bytes/) [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://lucianobestia.github.io/qrcode53bytes/qrcode53bytes/index.html) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/qrcode53bytes/blob/master/LICENSE)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1474-green.svg)](https://github.com/LucianoBestia/qrcode53bytes/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-258-blue.svg)](https://github.com/LucianoBestia/qrcode53bytes/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-86-purple.svg)](https://github.com/LucianoBestia/qrcode53bytes/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-31-yellow.svg)](https://github.com/LucianoBestia/qrcode53bytes/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/qrcode53bytes/)
//!
//! I need to generate a simple QR code for url that has max 53 bytes. I want to do this in wasm.  
//! QR codes are pretty complicated. So specifying only one single use-case makes the code smaller. But it looses universality.  
//! Smaller code is good for wasm.  
//! The url I want to encode is like this:  
//! `https://bestia.dev/mem6/#p04.1234`  
//! There is a hash symbol here so I cannot use the `alphanumeric mode`.  
//! I must use the `byte mode`.  
//! There are 33 characters. It means there is some free space for future uses.  
//! The smallest QR code for that is:
//!
//! - version 3
//! - 29x29 modules
//! - ECC Level L
//! - data bits 440
//! - 53 bytes
//! - ISO-8859-1
//!
//! The code is written by treeman. I just removed all the variants I don't need.  
//!
//! I use this code my wasm project <https://github.com/LucianoBestia/mem6_game>.  
//!
//! ## make
//!
//! List all the prepared commands and tasks with `$ cargo make`.  
//! <https://github.com/sagiegurari/cargo-make>
//!
//! ## References
//!
//! <https://github.com/treeman/rqr>  
//! <https://www.thonky.com/qr-code-tutorial>  
//!
//! ## cargo crev reviews and advisory
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! On the web use this url to read crate reviews. Example:  
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  
//!
//! ## changelog
//!
//! 1.1.0 bitvec dependency was yanked. updated to 1.17.4, but later versions have breaking changes  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

pub mod builder;
pub use builder::*;

pub mod data;
pub use data::*;

pub mod ec;
pub use ec::*;

pub mod info;
pub use info::*;

pub mod mask;
pub use mask::*;

pub mod matrix;
pub use matrix::{Matrix, Module};

pub mod mode;
pub use mode::Mode;

pub mod qr;
pub use qr::Qr;

pub mod rendercommons;
pub use rendercommons::*;

pub mod rendersvg;
pub use rendersvg::*;

pub mod renderstring;
pub use renderstring::*;

pub mod renderfordodrio;
pub use renderfordodrio::*;

pub mod version;
pub use version::Version;
