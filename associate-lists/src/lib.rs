use std::cmp::Ordering;

/// Associates elements of two lists with each other.
/// Elements of the shorter list are copied and distributed evenly to match the larger list.
///
/// # Panics
/// If one or both of the given lists are empty.
pub fn associate_lists<T>(first_list: &[T], second_list: &[T]) -> Vec<(T, T)>
where
    T: Copy,
{
    assert!(!first_list.is_empty());
    assert!(!second_list.is_empty());

    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => associate_lists_with_equal_lengths(first_list, second_list).collect(),
        Ordering::Less => associate_lists_where_second_is_longer(first_list, second_list).collect(),
        Ordering::Greater => associate_lists_where_second_is_longer(second_list, first_list)
            .map(|(first, second)| (second, first))
            .collect(),
    }
}

fn associate_lists_where_second_is_longer<'a, T>(
    first_list: &'a [T],
    second_list: &'a [T],
) -> impl Iterator<Item = (T, T)> + 'a
where
    T: Copy,
{
    let last_index_of_first_list = first_list.len() - 1;
    let last_index_of_second_list = second_list.len() - 1;
    let ratio = last_index_of_first_list as f64 / last_index_of_second_list as f64;

    second_list
        .iter()
        .copied()
        .enumerate()
        .map(move |(index, second_value)| {
            let first_list_index = (ratio * index as f64).round() as usize;
            let first_value = first_list[first_list_index];

            (first_value, second_value)
        })
}

fn associate_lists_with_equal_lengths<'a, T>(
    first_list: &'a [T],
    second_list: &'a [T],
) -> impl Iterator<Item = (T, T)> + 'a
where
    T: Copy,
{
    first_list.iter().copied().zip(second_list.iter().copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_result_when_first_list_is_longer_than_second() {
        let first_list = vec![10, 11, 12, 13, 14];
        let second_list = vec![20, 21, 22];
        let expected_pairs = vec![(10, 20), (11, 21), (12, 21), (13, 22), (14, 22)];

        assert_eq!(expected_pairs, associate_lists(&first_list, &second_list));
    }

    #[test]
    fn generates_correct_result_when_second_list_is_longer_than_first() {
        let first_list = vec![10, 11, 12];
        let second_list = vec![20, 21, 22, 23, 24];
        let expected_pairs = vec![(10, 20), (11, 21), (11, 22), (12, 23), (12, 24)];

        assert_eq!(expected_pairs, associate_lists(&first_list, &second_list));
    }

    #[test]
    fn generates_correct_result_when_both_lists_have_the_equal_length() {
        let first_list = vec![10, 20, 30];
        let second_list = vec![40, 50, 60];
        let expected_pairs = vec![(10, 40), (20, 50), (30, 60)];

        assert_eq!(expected_pairs, associate_lists(&first_list, &second_list));
    }

    #[test]
    #[should_panic]
    fn panics_when_first_list_is_empty() {
        let first_list = vec![];
        let second_list = vec![10, 11, 12];
        let _: Vec<_> = associate_lists(&first_list, &second_list);
    }

    #[test]
    #[should_panic]
    fn panics_when_second_list_is_empty() {
        let first_list = vec![10, 11, 12];
        let second_list = vec![];
        let _: Vec<_> = associate_lists(&first_list, &second_list);
    }

    #[test]
    #[should_panic]
    fn panics_when_both_lists_are_empty() {
        let first_list: Vec<()> = vec![];
        let second_list = vec![];
        let _: Vec<_> = associate_lists(&first_list, &second_list);
    }
}
