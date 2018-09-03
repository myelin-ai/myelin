use crate::physics_handler::CollisionMut;
use crate::properties::Object;

pub(crate) fn collision_mut_from_container_at<'a>(
    container: &'a mut [Object],
    first_index: usize,
    second_index: usize,
) -> CollisionMut<'a> {
    CollisionMut {
        // Because we are accessing two separate ressources and locking the container
        // for further modification, no parallel mutable access can happen.
        first: unsafe { &mut *(container.get_unchecked_mut(first_index) as *mut _) },
        second: &mut container[second_index],
    }
}
