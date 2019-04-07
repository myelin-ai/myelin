//! Random value generation

#![feature(box_syntax, specialization)]
#![warn(missing_docs, clippy::dbg_macro, clippy::unimplemented)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use myelin_clone_box::clone_box;
pub use random_impl::*;
use std::fmt::Debug;

mod random_impl;

/// Dedicated random number generator
#[cfg_attr(any(test, feature = "use-mocks"), mockable(static_references))]
pub trait Random: Debug + RandomClone {
    /// Returns a random boolean with equal chances of returning `true` or `false`.
    fn flip_coin(&self) -> bool;

    /// Returns a random boolean with a given probability of returning `true`.
    /// The probability is defined in the range `[0.0; 1.0]` where `0.0` means
    /// always return `false` and `1.0` means always return `true`.
    /// # Panics
    /// Panics if `probability` is outside the range [0.0; 1.0].
    fn flip_coin_with_probability(&self, probability: f64) -> bool;

    /// Returns a random element from the specified range [min; max)
    fn random_number_in_range(&self, min: i32, max: i32) -> i32;

    /// Returns a random floating point number in the specified range [min; max)
    fn random_float_in_range(&self, min: f64, max: f64) -> f64;
}

clone_box!(Random, RandomClone);
