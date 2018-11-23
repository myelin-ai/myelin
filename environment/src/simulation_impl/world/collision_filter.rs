//! Implementations for [`IgnoringCollisionFilter`]

use crate::simulation_impl::AnyHandle;
use alga::general::Real;
use ncollide2d::broad_phase::BroadPhasePairFilter;
use ncollide2d::world::CollisionObject;
use nphysics2d::object::ColliderData;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use unordered_pair::UnorderedPair;

/// A filter for the broad phase that checks if a pair should
/// be examined more closely for collisions. This filter allows the explicit
/// exclusion of body or sensor handles, which will have all their collisions
/// marked as invalid.
pub trait IgnoringCollisionFilter: Send + Sync + Debug {
    /// Registers a handle that should be ignored by this filter.
    fn add_ignored_handle(&mut self, handle: AnyHandle);
    /// Checks if a handle has been previously registered as ignored with
    /// [`add_ignored_handle`].
    ///
    /// [`add_ignored_handle`]: #tymethod.add_ignored_handle
    fn is_handle_ignored(&self, handle: AnyHandle) -> bool;
    /// Unregisters a handle that has been previously registered as ignored with
    /// [`add_ignored_handle`].
    ///
    /// [`add_ignored_handle`]: #tymethod.add_ignored_handle
    fn remove_ignored_handle(&mut self, handle: AnyHandle);
    /// Checks if the pair should be considered a collision or not.
    /// Returns `false` if the pair should be ignored.
    fn is_pair_valid(&self, pair: UnorderedPair<AnyHandle>) -> bool;
}

/// A filter for the broad phase that checks if a pair should
/// be examined more closely for collisions. This filter allows the explicit
/// exclusion of body or sensor handles, which will have all their collisions
/// marked as invalid.  
/// Implements [`BroadPhasePairFilter`] through [`IgnoringCollisionFilterWrapper`].
///
/// [`BroadPhasePairFilter`]: ./trait.BroadPhasePairFilter.html
/// [`IgnoringCollisionFilterWrapper`]: ./struct.IgnoringCollisionFilterWrapper.html
#[derive(Debug, Default)]
pub struct IgnoringCollisionFilterImpl {
    ignored_handles: HashSet<AnyHandle>,
}

impl IgnoringCollisionFilterImpl {}

impl IgnoringCollisionFilter for IgnoringCollisionFilterImpl {
    fn add_ignored_handle(&mut self, handle: AnyHandle) {
        self.ignored_handles.insert(handle);
    }

    fn is_handle_ignored(&self, handle: AnyHandle) -> bool {
        self.ignored_handles.contains(&handle)
    }

    fn remove_ignored_handle(&mut self, handle: AnyHandle) {
        self.ignored_handles.remove(&handle);
    }

    fn is_pair_valid(&self, pair: UnorderedPair<AnyHandle>) -> bool {
        !(self.ignored_handles.contains(&pair.0) || self.ignored_handles.contains(&pair.1))
    }
}

/// A wrapper around [`IgnoringCollisionFilter`] which can be passed to nphysics.
#[derive(Debug)]
pub(crate) struct IgnoringCollisionFilterWrapper {
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
            .is_pair_valid(UnorderedPair(b1.handle().into(), b2.handle().into()))
    }
}

#[cfg(test)]
pub(crate) use self::mock::IgnoringCollisionFilterMock;

#[cfg(test)]
mod mock {
    use super::*;
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use std::fmt::{self, Debug};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    #[test]
    fn pair_is_not_valid_if_ignored() {
        let ignored_handle = AnyHandle(0);

        let mut collision_filter = IgnoringCollisionFilterImpl::default();
        collision_filter.add_ignored_handle(ignored_handle);

        assert!(collision_filter.is_handle_ignored(ignored_handle));
        assert!(!collision_filter.is_pair_valid(UnorderedPair(ignored_handle, AnyHandle(1))));
        assert!(!collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), ignored_handle)));
    }

    #[test]
    fn pair_is_valid_if_not_ignored() {
        let collision_filter = IgnoringCollisionFilterImpl::default();

        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(0), AnyHandle(1))));
        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), AnyHandle(0))));
    }

    #[test]
    fn pair_is_valid_when_handle_added_and_removed() {
        let ignored_handle = AnyHandle(0);

        let mut collision_filter = IgnoringCollisionFilterImpl::default();
        collision_filter.add_ignored_handle(ignored_handle);

        assert!(collision_filter.is_handle_ignored(ignored_handle));

        collision_filter.remove_ignored_handle(ignored_handle);

        assert!(collision_filter.is_pair_valid(UnorderedPair(ignored_handle, AnyHandle(1))));
        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), ignored_handle)));
    }

    #[derive(Default)]
    pub(crate) struct IgnoringCollisionFilterMock {
        expect_add_ignored_handle: Option<AnyHandle>,
        expect_is_handle_ignored_and_return: RwLock<VecDeque<(AnyHandle, bool)>>,
        expect_remove_ignored_handle: Option<AnyHandle>,
        expect_is_pair_valid_and_return: RwLock<HashMap<UnorderedPair<AnyHandle>, bool>>,

        add_ignored_handle_was_called: AtomicBool,
        is_handle_ignored_was_called: AtomicBool,
        remove_ignored_handle_was_called: AtomicBool,
        is_pair_valid_was_called: AtomicBool,
    }

    impl IgnoringCollisionFilterMock {
        #[allow(dead_code)]
        pub fn expect_add_ignored_handle(&mut self, handle: AnyHandle) -> &mut Self {
            self.expect_add_ignored_handle = Some(handle);
            self
        }

        pub fn expect_is_handle_ignored_and_return(
            &mut self,
            expected_calls: VecDeque<(AnyHandle, bool)>,
        ) -> &mut Self {
            self.expect_is_handle_ignored_and_return = RwLock::new(expected_calls);
            self
        }

        #[allow(dead_code)]
        pub fn expect_remove_ignored_handle(&mut self, handle: AnyHandle) -> &mut Self {
            self.expect_remove_ignored_handle = Some(handle);
            self
        }

        pub fn expect_is_pair_valid_and_return(
            &mut self,
            expected_calls: HashMap<UnorderedPair<AnyHandle>, bool>,
        ) -> &mut Self {
            self.expect_is_pair_valid_and_return = RwLock::new(expected_calls);
            self
        }
    }

    impl Drop for IgnoringCollisionFilterMock {
        fn drop(&mut self) {
            if !panicking() {
                if self.expect_add_ignored_handle.is_some() {
                    assert!(
                        self.add_ignored_handle_was_called.load(Ordering::SeqCst),
                        "add_ignored_handle() was not called, but was expected"
                    );
                }

                if !self
                    .expect_is_handle_ignored_and_return
                    .read()
                    .expect("Lock was poisoned")
                    .is_empty()
                {
                    assert!(
                        self.is_handle_ignored_was_called.load(Ordering::SeqCst),
                        "is_handle_ignored() was not called, but was expected"
                    );
                }

                if self.expect_remove_ignored_handle.is_some() {
                    assert!(
                        self.remove_ignored_handle_was_called.load(Ordering::SeqCst),
                        "remove_ignored_handle() was not called, but was expected"
                    );
                }

                if !self
                    .expect_is_pair_valid_and_return
                    .read()
                    .expect("Lock was poisoned")
                    .is_empty()
                {
                    assert!(
                        self.is_pair_valid_was_called.load(Ordering::SeqCst),
                        "is_pair_valid() was not called, but was expected"
                    );
                }
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
        fn add_ignored_handle(&mut self, handle: AnyHandle) {
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

        fn is_handle_ignored(&self, handle: AnyHandle) -> bool {
            self.is_handle_ignored_was_called
                .store(true, Ordering::SeqCst);

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

        fn remove_ignored_handle(&mut self, handle: AnyHandle) {
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

        fn is_pair_valid(&self, pair: UnorderedPair<AnyHandle>) -> bool {
            self.is_pair_valid_was_called.store(true, Ordering::SeqCst);

            let expected_calls = self
                .expect_is_pair_valid_and_return
                .write()
                .expect("RwLock was poisoned");

            if expected_calls.is_empty() {
                panic!("is_pair_valid() was called unexpectedly");
            }

            if let Some(expected_output) = expected_calls.get(&pair) {
                return *expected_output;
            }

            panic!(
                "is_pair_valid() was called with unexpected input values: handle1: {:?} and handle2: {:?}",
                pair.0,
                pair.1
            )
        }
    }
}
