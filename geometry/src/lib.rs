//! Basic linear and vector geometry for two-dimensional Euclidean geometry

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    intra_doc_link_resolution_failure,
    clippy::doc_markdown,
    clippy::unimplemented
)]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate nearly_eq;

#[macro_use]
extern crate serde_derive;

mod aabb;
pub use self::aabb::*;

mod radians;
pub use self::radians::*;

mod polygon;
pub use self::polygon::*;

mod vector;
pub use self::vector::*;

mod point;
pub use self::point::*;

mod convex_hull;
pub use self::convex_hull::*;
