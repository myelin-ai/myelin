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

#[macro_export]
macro_rules! clone_box {
    ($trait_ident: ident, $clone_trait_ident: ident) => {
        #[doc(hidden)]
        pub trait $clone_trait_ident {
            fn clone_box<'a>(&self) -> Box<dyn $trait_ident + 'a>
            where
                Self: 'a;
        }

        impl<T> $clone_trait_ident for T
        where
            T: $trait_ident + Clone,
        {
            default fn clone_box<'a>(&self) -> Box<dyn $trait_ident + 'a>
            where
                Self: 'a,
            {
                box self.clone()
            }
        }

        impl Clone for Box<dyn $trait_ident> {
            fn clone(&self) -> Self {
                self.clone_box()
            }
        }
    };
}
