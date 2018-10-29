use alga::general::Real;
use crate::simulation_impl::Handle;
use ncollide2d::broad_phase::BroadPhasePairFilter;
use ncollide2d::world::CollisionObject;
use nphysics2d::object::ColliderData;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub trait IgnoringCollisionFilter: Send + Sync + Debug {
    fn add_ignored_handle(&mut self, handle: Handle);
    fn is_handle_ignored(&self, handle: Handle) -> bool;
    fn remove_ignored_handle(&mut self, handle: Handle);
    fn is_pair_valid(&self, h1: Handle, h2: Handle) -> bool;
}

#[derive(Debug, Default)]
pub struct IgnoringCollisionFilterImpl {
    ignored_handles: HashSet<Handle>,
}

impl IgnoringCollisionFilterImpl {}

impl IgnoringCollisionFilter for IgnoringCollisionFilterImpl {
    fn add_ignored_handle(&mut self, handle: Handle) {
        self.ignored_handles.insert(handle);
    }

    fn is_handle_ignored(&self, handle: Handle) -> bool {
        self.ignored_handles.contains(&handle)
    }

    fn remove_ignored_handle(&mut self, handle: Handle) {
        self.ignored_handles.remove(&handle);
    }

    fn is_pair_valid(&self, b1: Handle, b2: Handle) -> bool {
        !(self.ignored_handles.contains(&b1) || self.ignored_handles.contains(&b2))
    }
}

#[derive(Debug)]
pub struct IgnoringCollisionFilterWrapper {
    pub(crate) collision_filter: Arc<RwLock<dyn IgnoringCollisionFilter>>,
}

impl<N> BroadPhasePairFilter<N, ColliderData<N>> for IgnoringCollisionFilterWrapper
where
    N: Real,
{
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<N, ColliderData<N>>,
        b2: &CollisionObject<N, ColliderData<N>>,
    ) -> bool {
        self.collision_filter
            .read()
            .expect("Lock was poisoned")
            .is_pair_valid(b1.handle(), b2.handle())
    }
}

#[cfg(test)]
pub(crate) use self::mock::IgnoringCollisionFilterMock;

#[cfg(test)]
mod mock {
    use super::*;
    use std::collections::VecDeque;
    use std::fmt::{self, Debug};
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::thread::panicking;

    #[derive(Default)]
    pub(crate) struct IgnoringCollisionFilterMock {
        expect_add_ignored_handle: Option<Handle>,
        expect_is_handle_ignored_and_return: RwLock<VecDeque<(Handle, bool)>>,
        expect_remove_ignored_handle: Option<Handle>,
        expect_is_pair_valid_and_return: Option<((Handle, Handle), bool)>,

        add_ignored_handle_was_called: AtomicBool,
        is_handle_ignored_was_called: AtomicU32,
        remove_ignored_handle_was_called: AtomicBool,
        is_pair_valid_was_called: AtomicBool,
    }

    impl IgnoringCollisionFilterMock {
        pub fn expect_add_ignored_handle(&mut self, handle: Handle) -> &mut Self {
            self.expect_add_ignored_handle = Some(handle);
            self
        }

        pub fn expect_is_handle_ignored_and_return(
            &mut self,
            expected_calls: VecDeque<(Handle, bool)>,
        ) -> &mut Self {
            self.expect_is_handle_ignored_and_return = RwLock::new(expected_calls);
            self
        }

        pub fn expect_remove_ignored_handle(&mut self, handle: Handle) -> &mut Self {
            self.expect_remove_ignored_handle = Some(handle);
            self
        }

        pub fn expect_is_pair_valid_and_return(
            &mut self,
            b1: Handle,
            b2: Handle,
            is_valid: bool,
        ) -> &mut Self {
            self.expect_is_pair_valid_and_return = Some(((b1, b2), is_valid));
            self
        }
    }

    impl Drop for IgnoringCollisionFilterMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
        }
    }

    impl Debug for IgnoringCollisionFilterMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct(name_of_type!(IgnoringCollisionFilterMock))
                .finish()
        }
    }

    impl IgnoringCollisionFilter for IgnoringCollisionFilterMock {
        fn add_ignored_handle(&mut self, handle: Handle) {
            self.add_ignored_handle_was_called
                .store(true, Ordering::SeqCst);

            if let Some(expected_input) = self.expect_add_ignored_handle {
                if handle != expected_input {
                    panic!(
                        "add_ignored_handle() was called with an unexpected input value: {:?}",
                        handle
                    )
                }
            } else {
                panic!("add_ignored_handle() was called unexpectedly")
            }
        }

        fn is_handle_ignored(&self, handle: Handle) -> bool {
            self.is_handle_ignored_was_called.store(
                self.is_handle_ignored_was_called.load(Ordering::SeqCst) + 1,
                Ordering::SeqCst,
            );

            if let Some((expected_input, expected_output)) = self
                .expect_is_handle_ignored_and_return
                .write()
                .expect("RwLock was poisoned")
                .pop_front()
            {
                if handle != expected_input {
                    panic!(
                        "is_handle_ignored() was called with an unexpected input value: {:?}",
                        handle
                    )
                }

                expected_output
            } else {
                panic!("is_handle_ignored() was called unexpectedly")
            }
        }

        fn remove_ignored_handle(&mut self, handle: Handle) {
            self.remove_ignored_handle_was_called
                .store(true, Ordering::SeqCst);

            if let Some(expected_input) = self.expect_remove_ignored_handle {
                if handle != expected_input {
                    panic!(
                        "remove_ignored_handle() was called with an unexpected input value: {:?}",
                        handle
                    )
                }
            } else {
                panic!("remove_ignored_handle() was called unexpectedly")
            }
        }

        fn is_pair_valid(&self, b1: Handle, b2: Handle) -> bool {
            self.is_pair_valid_was_called.store(true, Ordering::SeqCst);

            if let Some(((ref expected_b1, ref expected_b2), expected_output)) =
                self.expect_is_pair_valid_and_return
            {
                if (b1 != *expected_b1 || b2 != *expected_b2)
                    && (b1 != *expected_b2 || b2 != *expected_b1)
                {
                    println!("{:?}, {:?}", *expected_b1, *expected_b2);

                    panic!(
                        "is_pair_valid() was called with unexpected input values: handle1: {:?} and handle2: {:?}",
                        b1,
                        b2
                    )
                }

                expected_output
            } else {
                panic!("is_pair_valid() was called unexpectedly")
            }
        }
    }
}
