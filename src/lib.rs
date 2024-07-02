#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]

//!# sage-moon-math
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// Contains neccessary methods related to an integer
pub mod integer;

/// Contains neccessary methods for conversion of data type
pub mod conversion;

/// Contains methods to solve polynomial eqaution, get factors, e.t.c
pub mod polynomial;