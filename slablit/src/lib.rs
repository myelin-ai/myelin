#![feature(decl_macro)]

#[allow(unused_macros)]
macro replace_expr($_t:tt $sub:expr) {
    $sub
}

#[allow(unused_macros)]
macro count_tts {
    ($($tts:tt)*) => {<[()]>::len(&[$(crate::replace_expr!($tts ())),*])};
}

pub macro slab {
    ($( $x:expr ),*) => {{
        let mut temp_slab = slab::Slab::with_capacity(crate::count_tts!($($x)*));

        $(
            temp_slab.insert($x);
        )*

        temp_slab
    }};
}

#[cfg(test)]
mod test {
    use super::slab;
    use slab::Slab;

    #[test]
    fn test_slab_macro() {
        let slab: Slab<usize> = slab![10, 20, 30];

        assert_eq!(
            vec![10, 20, 30],
            slab.into_iter()
                .map(|(_, value)| *value)
                .collect::<Vec<usize>>()
        );
    }
}
