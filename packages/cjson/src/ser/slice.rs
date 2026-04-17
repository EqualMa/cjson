use crate::utils::iter_map::impl_iter_map;

use super::{ToJson, ToJsonArray, texts};

pub struct IterMapToJson<'a, T: 'a + ToJson> {
    iter: core::slice::Iter<'a, T>,
}

impl<'a, T: 'a + ToJson> Iterator for IterMapToJson<'a, T> {
    type Item = T::ToJson<'a>;

    impl_iter_map!(|v| T::to_json(v));
}

impl<T: ToJson> ToJsonArray for [T] {
    type ToJsonArray<'a>
        = texts::ArrayOfIter<IterMapToJson<'a, T>>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        texts::ArrayOfIter(IterMapToJson { iter: self.iter() })
    }
}

impl<T: ToJson> ToJson for [T] {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}

impl<T: ToJson, const N: usize> ToJsonArray for [T; N] {
    type ToJsonArray<'a>
        = <[T] as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <[T] as ToJsonArray>::to_json_array(self)
    }
}

impl<T: ToJson, const N: usize> ToJson for [T; N] {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc;
