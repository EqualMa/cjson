use crate::{utils::iter_map::impl_iter_map, values::ArrayOfIter};

use super::{
    ConsumeJson, Consumed, IntoJson, ToJson, ToJson2, ToJsonArray,
    json_kinds::{self, JsonKind},
    texts,
};

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

impl<T> ToJson2 for [T]
where
    for<'a> &'a T: IntoJson,
{
    type ToJsonKind = json_kinds::Array;

    fn json_provide_to<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> Consumed<Self::ToJsonKind, W> {
        ArrayOfIter(self).json_provide_into(w)
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
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

impl<T: IntoJson, const N: usize> IntoJson for [T; N] {
    type JsonKind = json_kinds::Array;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        ArrayOfIter(self).json_provide_into(w)
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

impl<T, const N: usize> ToJson2 for [T; N]
where
    for<'a> &'a T: IntoJson,
{
    type ToJsonKind = json_kinds::Array;

    fn json_provide_to<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> Consumed<Self::ToJsonKind, W> {
        <[T]>::json_provide_to(self, w)
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

#[cfg(feature = "alloc")]
mod alloc;
