pub(crate) fn map_fold<T, B, Acc>(
    mut f: impl FnMut(T) -> B,
    mut g: impl FnMut(Acc, B) -> Acc,
) -> impl FnMut(Acc, T) -> Acc {
    move |acc, elt| g(acc, f(elt))
}

/// [`core::iter::Map`]
macro_rules! impl_iter_map {
    ($f:expr) => {
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map($f)
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }

        fn fold<Acc, G>(self, init: Acc, g: G) -> Acc
        where
            G: FnMut(Acc, Self::Item) -> Acc,
        {
            self.iter
                .fold(init, crate::utils::iter_map::map_fold($f, g))
        }
    };
}

pub(crate) use impl_iter_map;
