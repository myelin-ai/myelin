//! Implementations for [`IgnoringCollisionFilter`]

use alga::general::Real;
use crate::simulation_impl::AnyHandle;
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
///
/// | First        | Second       | Collision |
/// |--------------|--------------|-----------|
/// | In no list   | In no list   | true      |
/// | In whitelist | In no list   | true      |
/// | In whitelist | In blacklist | true      |
/// | In whitelist | In whitelist | true      |
/// | In blacklist | In no list   | false     |
/// | In blacklist | In blacklist | false     |
pub trait IgnoringCollisionFilter: Send + Sync + Debug {
    /// Registers a handle that should be ignored by this filter.
    fn add_blacklisted_handle(&mut self, handle: AnyHandle);

    /// Registers a handle that should not be ignored by this filter.
    fn add_whitelisted_handle(&mut self, handle: AnyHandle);

    /// Checks if a handle has been previously registered as ignored with
    /// [`add_blacklisted_handle`].
    ///
    /// [`add_blacklisted_handle`]: #tymethod.add_blacklisted_handle
    fn is_handle_blacklisted(&self, handle: AnyHandle) -> bool;

    /// Unregisters a handle that has been previously registered as ignored with
    /// [`add_blacklisted_handle`].
    ///
    /// [`add_blacklisted_handle`]: #tymethod.add_blacklisted_handle
    fn remove_blacklisted_handle(&mut self, handle: AnyHandle);

    /// Unregisters a handle that has been previously registered as not ignored with
    /// [`add_whitelisted_handle`].
    ///
    /// [`add_whitelisted_handle`]: #tymethod.add_blacklisted_handle
    fn remove_whitelisted_handle(&mut self, handle: AnyHandle);

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
    blacklisted_handles: HashSet<AnyHandle>,
    whitelisted_handles: HashSet<AnyHandle>,
}

impl IgnoringCollisionFilterImpl {}

impl IgnoringCollisionFilter for IgnoringCollisionFilterImpl {
    fn add_blacklisted_handle(&mut self, handle: AnyHandle) {
        self.blacklisted_handles.insert(handle);
    }

    fn add_whitelisted_handle(&mut self, handle: AnyHandle) {
        self.whitelisted_handles.insert(handle);
    }

    fn is_handle_blacklisted(&self, handle: AnyHandle) -> bool {
        self.blacklisted_handles.contains(&handle)
    }

    fn remove_blacklisted_handle(&mut self, handle: AnyHandle) {
        self.blacklisted_handles.remove(&handle);
    }

    fn remove_whitelisted_handle(&mut self, handle: AnyHandle) {
        self.whitelisted_handles.remove(&handle);
    }

    fn is_pair_valid(&self, pair: UnorderedPair<AnyHandle>) -> bool {
        self.whitelisted_handles.contains(&pair.0)
            || self.whitelisted_handles.contains(&pair.1)
            || !(self.blacklisted_handles.contains(&pair.0)
                || self.blacklisted_handles.contains(&pair.1))
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
    fn pair_is_not_valid_if_blacklisted() {
        let blacklisted_handle = AnyHandle(0);

        let mut collision_filter = IgnoringCollisionFilterImpl::default();
        collision_filter.add_blacklisted_handle(blacklisted_handle);

        assert!(collision_filter.is_handle_blacklisted(blacklisted_handle));
        assert!(!collision_filter.is_pair_valid(UnorderedPair(blacklisted_handle, AnyHandle(1))));
        assert!(!collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), blacklisted_handle)));
    }

    #[test]
    fn pair_is_valid_if_not_blacklisted() {
        let collision_filter = IgnoringCollisionFilterImpl::default();

        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(0), AnyHandle(1))));
        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), AnyHandle(0))));
    }

    #[test]
    fn pair_is_valid_if_one_element_is_whitelisted() {
        let whitelisted_handle = AnyHandle(0);

        let mut collision_filter = IgnoringCollisionFilterImpl::default();
        collision_filter.add_whitelisted_handle(whitelisted_handle);

        assert!(collision_filter.is_pair_valid(UnorderedPair(whitelisted_handle, AnyHandle(1))));
        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), whitelisted_handle)));
    }

    #[test]
    fn pair_is_valid_if_one_element_is_whitelisted_and_the_other_blacklisted() {
        let whitelisted_handle = AnyHandle(0);
        let blacklisted_handle = AnyHandle(1);

        let mut collision_filter = IgnoringCollisionFilterImpl::default();
        collision_filter.add_whitelisted_handle(whitelisted_handle);
        collision_filter.add_blacklisted_handle(blacklisted_handle);

        assert!(
            collision_filter.is_pair_valid(UnorderedPair(whitelisted_handle, blacklisted_handle))
        );
        assert!(
            collision_filter.is_pair_valid(UnorderedPair(blacklisted_handle, whitelisted_handle))
        );
    }

    #[test]
    fn pair_is_valid_when_handle_added_and_removed_from_blacklist() {
        let blacklisted_handle = AnyHandle(0);

        let mut collision_filter = IgnoringCollisionFilterImpl::default();
        collision_filter.add_blacklisted_handle(blacklisted_handle);

        assert!(collision_filter.is_handle_blacklisted(blacklisted_handle));

        collision_filter.remove_blacklisted_handle(blacklisted_handle);

        assert!(collision_filter.is_pair_valid(UnorderedPair(blacklisted_handle, AnyHandle(1))));
        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), blacklisted_handle)));
    }

    #[test]
    fn pair_is_not_valid_when_handle_added_and_removed_from_whitelist() {
        let whitelisted_handle = AnyHandle(0);

        let mut collision_filter = IgnoringCollisionFilterImpl::default();
        collision_filter.add_whitelisted_handle(whitelisted_handle);
        collision_filter.remove_whitelisted_handle(whitelisted_handle);

        assert!(collision_filter.is_pair_valid(UnorderedPair(whitelisted_handle, AnyHandle(1))));
        assert!(collision_filter.is_pair_valid(UnorderedPair(AnyHandle(1), whitelisted_handle)));
    }

    #[derive(Default)]
    pub(crate) struct IgnoringCollisionFilterMock {
        expect_add_blacklisted_handle: Option<AnyHandle>,
        expect_add_whitelisted_handle: bool,
        expect_is_handle_blacklisted_and_return: RwLock<VecDeque<(AnyHandle, bool)>>,
        expect_remove_blacklisted_handle: Option<AnyHandle>,
        expect_remove_whitelisted_handle: Option<AnyHandle>,
        expect_is_pair_valid_and_return: RwLock<HashMap<UnorderedPair<AnyHandle>, bool>>,

        add_blacklisted_handle_was_called: AtomicBool,
        add_whitelisted_handle_was_called: AtomicBool,
        is_handle_blacklisted_was_called: AtomicBool,
        remove_blacklisted_handle_was_called: AtomicBool,
        remove_whitelisted_handle_was_called: AtomicBool,
        is_pair_valid_was_called: AtomicBool,
    }

    impl IgnoringCollisionFilterMock {
        pub fn expect_add_blacklisted_handle(&mut self, handle: AnyHandle) -> &mut Self {
            self.expect_add_blacklisted_handle = Some(handle);
            self
        }

        pub fn expect_add_whitelisted_handle(&mut self) -> &mut Self {
            self.expect_add_whitelisted_handle = true;
            self
        }

        pub fn expect_is_handle_blacklisted_and_return(
            &mut self,
            expected_calls: VecDeque<(AnyHandle, bool)>,
        ) -> &mut Self {
            self.expect_is_handle_blacklisted_and_return = RwLock::new(expected_calls);
            self
        }

        pub fn expect_remove_blacklisted_handle(&mut self, handle: AnyHandle) -> &mut Self {
            self.expect_remove_blacklisted_handle = Some(handle);
            self
        }

        pub fn expect_remove_whitelisted_handle(&mut self, handle: AnyHandle) -> &mut Self {
            self.expect_remove_whitelisted_handle = Some(handle);
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
                if self.expect_add_blacklisted_handle.is_some() {
                    assert!(
                        self.add_blacklisted_handle_was_called
                            .load(Ordering::SeqCst),
                        "add_blacklisted_handle() was not called, but was expected"
                    );
                }

                if self.expect_add_whitelisted_handle {
                    assert!(
                        self.add_whitelisted_handle_was_called
                            .load(Ordering::SeqCst),
                        "add_whitelisted_handle() was not called, but was expected"
                    );
                }

                if !self
                    .expect_is_handle_blacklisted_and_return
                    .read()
                    .expect("Lock was poisoned")
                    .is_empty()
                {
                    assert!(
                        self.is_handle_blacklisted_was_called.load(Ordering::SeqCst),
                        "is_handle_blacklisted() was not called, but was expected"
                    );
                }

                if self.expect_remove_blacklisted_handle.is_some() {
                    assert!(
                        self.remove_blacklisted_handle_was_called
                            .load(Ordering::SeqCst),
                        "remove_blacklisted_handle() was not called, but was expected"
                    );
                }

                if self.expect_remove_whitelisted_handle.is_some() {
                    assert!(
                        self.remove_whitelisted_handle_was_called
                            .load(Ordering::SeqCst),
                        "remove_whitelisted_handle() was not called, but was expected"
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
        fn add_blacklisted_handle(&mut self, handle: AnyHandle) {
            self.add_blacklisted_handle_was_called
                .store(true, Ordering::SeqCst);

            if let Some(expected_input) = self.expect_add_blacklisted_handle {
                if handle != expected_input {
                    panic!(
                        "add_blacklisted_handle() was called with an unexpected input value: {:?}",
                        handle
                    )
                }
            } else {
                panic!("add_blacklisted_handle() was called unexpectedly")
            }
        }

        fn add_whitelisted_handle(&mut self, handle: AnyHandle) {
            self.add_whitelisted_handle_was_called
                .store(true, Ordering::SeqCst);

            if !self.expect_add_whitelisted_handle {
                panic!("add_whitelisted_handle() was called unexpectedly")
            }
        }

        fn is_handle_blacklisted(&self, handle: AnyHandle) -> bool {
            self.is_handle_blacklisted_was_called
                .store(true, Ordering::SeqCst);

            if let Some((expected_input, expected_output)) = self
                .expect_is_handle_blacklisted_and_return
                .write()
                .expect("RwLock was poisoned")
                .pop_front()
            {
                if handle != expected_input {
                    panic!(
                        "is_handle_blacklisted() was called with an unexpected input value: {:?}",
                        handle
                    )
                }

                expected_output
            } else {
                panic!("is_handle_blacklisted() was called unexpectedly")
            }
        }

        fn remove_blacklisted_handle(&mut self, handle: AnyHandle) {
            self.remove_blacklisted_handle_was_called
                .store(true, Ordering::SeqCst);

            if let Some(expected_input) = self.expect_remove_blacklisted_handle {
                if handle != expected_input {
                    panic!(
                        "remove_blacklisted_handle() was called with an unexpected input value: {:?}",
                        handle
                    )
                }
            } else {
                panic!("remove_blacklisted_handle() was called unexpectedly")
            }
        }

        fn remove_whitelisted_handle(&mut self, handle: AnyHandle) {
            self.remove_whitelisted_handle_was_called
                .store(true, Ordering::SeqCst);

            if let Some(expected_input) = self.expect_remove_whitelisted_handle {
                if handle != expected_input {
                    panic!(
                        "remove_whitelisted_handle() was called with an unexpected input value: {:?}",
                        handle
                    )
                }
            } else {
                panic!("remove_whitelisted_handle() was called unexpectedly")
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
