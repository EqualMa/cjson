use crate::{utils::iter_map::impl_iter_map, values::ArrayOfIter};

use super::{IntoJson, ToJson, ToJson2, ToJsonArray, helpers::json_fns, json_kinds, texts};

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

    json_fns!({
        json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
        use trait_mod;
        |&self, w| {
            use trait_mod::XHelpers as _;
            ArrayOfIter(self).json_provide_into_x(w)
        }
    });

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

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        use trait_mod;
        |self, w| {
            use trait_mod::XHelpers as _;
            ArrayOfIter(self).json_provide_into_x(w)
        }
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

impl<T, const N: usize> ToJson2 for [T; N]
where
    for<'a> &'a T: IntoJson,
{
    type ToJsonKind = json_kinds::Array;

    json_fns!({
        json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
        use trait_mod;
        |&self, w| {
            use trait_mod::XHelpers as _;
            <[T]>::json_provide_to_x(self, w)
        }
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

#[cfg(feature = "alloc")]
mod alloc;
