use crate::values::ArrayOfIter;

use super::{IntoJson, ToJson2, helpers::json_fns, json_kinds};

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
