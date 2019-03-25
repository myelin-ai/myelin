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
    }
}
