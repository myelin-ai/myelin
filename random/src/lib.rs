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
pub use random_chance_checker_impl::*;
use std::fmt::Debug;

mod random_chance_checker_impl;

/// Dedicated random number generator
#[cfg_attr(any(test, feature = "use-mocks"), mockable(static_references))]
pub trait RandomChanceChecker: Debug + RandomChanceCheckerClone {
    /// Returns a random boolean with equal chances of returning `true` or `false`.
    fn flip_coin(&mut self) -> bool;

    /// Returns a random boolean with a given probability of returning `true`.
    /// The probability is defined in the range `[0.0; 1.0]` where `0.0` means
    /// always return `false` and `1.0` means always return `true`.
    /// # Panics
    /// Panics if `probability` is outside the range [0.0; 1.0].
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool;

    /// Returns a random element from the specified range [min; max)
    fn random_number_in_range(&mut self, min: i32, max: i32) -> i32;
}

/// Supertrait used to make sure that all implementors
/// of [`RandomChanceChecker`] are [`Clone`]. You don't need
/// to care about this type.
///
/// [`RandomChanceChecker`]: ./trait.RandomChanceChecker.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html
#[doc(hidden)]
pub trait RandomChanceCheckerClone {
    fn clone_box(&self) -> Box<dyn RandomChanceChecker>;
}

impl<T> RandomChanceCheckerClone for T
where
    T: RandomChanceChecker + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn RandomChanceChecker> {
        box self.clone()
    }
}

impl Clone for Box<dyn RandomChanceChecker> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
