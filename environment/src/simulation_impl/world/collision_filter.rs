use super::to_body_handle;
use alga::general::Real;
use crate::simulation_impl::BodyHandle;
use ncollide2d::broad_phase::BroadPhasePairFilter;
use ncollide2d::world::CollisionObject;
use nphysics2d::object::ColliderData;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub trait IgnoringCollisionFilter<N>: BroadPhasePairFilter<N, ColliderData<N>> + Debug
where
    N: Real,
{
    fn add_ignored_body_handle(&mut self, body_handle: BodyHandle);
    fn is_body_ignored(&self, body_handle: BodyHandle) -> bool;
    fn remove_ignored_body_handle(&mut self, body_handle: BodyHandle);
}

#[derive(Debug, Default)]
pub struct IgnoringCollisionFilterImpl {
    ignored_body_handles: HashSet<BodyHandle>,
}

impl IgnoringCollisionFilterImpl {}

impl<N> IgnoringCollisionFilter<N> for IgnoringCollisionFilterImpl
where
    N: Real,
{
    fn add_ignored_body_handle(&mut self, body_handle: BodyHandle) {
        self.ignored_body_handles.insert(body_handle);
    }

    fn is_body_ignored(&self, body_handle: BodyHandle) -> bool {
        self.ignored_body_handles.contains(&body_handle)
    }

    fn remove_ignored_body_handle(&mut self, body_handle: BodyHandle) {
        self.ignored_body_handles.remove(&body_handle);
    }
}

impl<N> BroadPhasePairFilter<N, ColliderData<N>> for IgnoringCollisionFilterImpl
where
    N: Real,
{
    fn is_pair_valid(
        &self,
        b1: &CollisionObject<N, ColliderData<N>>,
        b2: &CollisionObject<N, ColliderData<N>>,
    ) -> bool {
        let body_handle1 = to_body_handle(b1.handle());
        let body_handle2 = to_body_handle(b2.handle());

        !(self.ignored_body_handles.contains(&body_handle1)
            || self.ignored_body_handles.contains(&body_handle2))
    }
}

#[derive(Debug)]
pub struct IgnoringCollisionFilterWrapper<N> {
    pub(crate) collision_filter: Arc<RwLock<dyn IgnoringCollisionFilter<N>>>,
}

impl<N> BroadPhasePairFilter<N, ColliderData<N>> for IgnoringCollisionFilterWrapper<N>
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
            .is_pair_valid(b1, b2)
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
    pub(crate) struct IgnoringCollisionFilterMock<N>
    where
        N: Real,
    {
        expect_add_ignored_body_handle: Option<BodyHandle>,
        expect_is_body_ignored_and_return: RwLock<VecDeque<(BodyHandle, bool)>>,
        expect_remove_ignored_body_handle: Option<BodyHandle>,
        expect_is_pair_valid_and_return: Option<(
            (
                CollisionObject<N, ColliderData<N>>,
                CollisionObject<N, ColliderData<N>>,
            ),
            bool,
        )>,

        add_ignored_body_handle_was_called: AtomicBool,
        is_body_ignored_was_called: AtomicU32,
        remove_ignored_body_handle_was_called: AtomicBool,
        is_pair_valid_was_called: AtomicBool,
    }

    impl<N> IgnoringCollisionFilterMock<N>
    where
        N: Real,
    {
        pub fn expect_add_ignored_body_handle(&mut self, body_handle: BodyHandle) -> &mut Self {
            self.expect_add_ignored_body_handle = Some(body_handle);
            self
        }

        pub fn expect_is_body_ignored_and_return(
            &mut self,
            expected_calls: VecDeque<(BodyHandle, bool)>,
        ) -> &mut Self {
            self.expect_is_body_ignored_and_return = RwLock::new(expected_calls);
            self
        }

        pub fn expect_remove_ignored_body_handle(&mut self, body_handle: BodyHandle) -> &mut Self {
            self.expect_remove_ignored_body_handle = Some(body_handle);
            self
        }

        pub fn expect_is_pair_valid_and_return(
            &mut self,
            o1: CollisionObject<N, ColliderData<N>>,
            o2: CollisionObject<N, ColliderData<N>>,
            is_valid: bool,
        ) -> &mut Self {
            self.expect_is_pair_valid_and_return = Some(((o1, o2), is_valid));
            self
        }
    }

    impl<N> Drop for IgnoringCollisionFilterMock<N>
    where
        N: Real,
    {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
        }
    }

    impl<N> Debug for IgnoringCollisionFilterMock<N>
    where
        N: Real,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct(name_of_type!(IgnoringCollisionFilterMock<N>))
                .finish()
        }
    }

    impl<N> IgnoringCollisionFilter<N> for IgnoringCollisionFilterMock<N>
    where
        N: Real,
    {
        fn add_ignored_body_handle(&mut self, body_handle: BodyHandle) {
            self.add_ignored_body_handle_was_called
                .store(true, Ordering::SeqCst);

            if let Some(expected_input) = self.expect_add_ignored_body_handle {
                if body_handle != expected_input {
                    panic!(
                        "add_ignored_body_handle() was called with an unexpected input value: {:?}",
                        body_handle
                    )
                }
            } else {
                panic!("add_ignored_body_handle() was called unexpectedly")
            }
        }

        fn is_body_ignored(&self, body_handle: BodyHandle) -> bool {
            self.is_body_ignored_was_called.store(
                self.is_body_ignored_was_called.load(Ordering::SeqCst) + 1,
                Ordering::SeqCst,
            );

            if let Some((expected_input, expected_output)) = self
                .expect_is_body_ignored_and_return
                .write()
                .expect("RwLock was poisoned")
                .pop_front()
            {
                if body_handle != expected_input {
                    panic!(
                        "is_body_ignored() was called with an unexpected input value: {:?}",
                        body_handle
                    )
                }

                expected_output
            } else {
                panic!("is_body_ignored() was called unexpectedly")
            }
        }

        fn remove_ignored_body_handle(&mut self, body_handle: BodyHandle) {
            self.remove_ignored_body_handle_was_called
                .store(true, Ordering::SeqCst);

            if let Some(expected_input) = self.expect_remove_ignored_body_handle {
                if body_handle != expected_input {
                    panic!(
                        "remove_ignored_body_handle() was called with an unexpected input value: {:?}",
                        body_handle
                    )
                }
            } else {
                panic!("remove_ignored_body_handle() was called unexpectedly")
            }
        }
    }

    impl<N> BroadPhasePairFilter<N, ColliderData<N>> for IgnoringCollisionFilterMock<N>
    where
        N: Real,
    {
        fn is_pair_valid(
            &self,
            b1: &CollisionObject<N, ColliderData<N>>,
            b2: &CollisionObject<N, ColliderData<N>>,
        ) -> bool {
            self.is_pair_valid_was_called.store(true, Ordering::SeqCst);

            if let Some(((ref expected_b1, ref expected_b2), expected_output)) =
                self.expect_is_pair_valid_and_return
            {
                if b1.handle() != expected_b1.handle() || b2.handle() != expected_b2.handle() {
                    panic!(
                        "is_pair_valid() was called with an unexpected input values: handle1: {:?} and handle2: {:?}",
                        b1.handle(),
                        b2.handle()
                    )
                }

                expected_output
            } else {
                panic!("is_pair_valid() was called unexpectedly")
            }
        }
    }
}
