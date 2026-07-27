use alloc::{boxed::Box, vec::Vec};

use crate::{
    ser::{IntoJson, ToJson2 as ToJson, helpers::json_fns, json_kinds},
    utils::impl_many,
    values::ArrayOfIter,
};

impl_many!({
    {
        {
            use Vec as TOwnedSlice;
        }
        {
            type TOwnedSlice<T> = Box<[T]>;
        }
    }

    impl<T: IntoJson> IntoJson for TOwnedSlice<T> {
        type JsonKind = json_kinds::Array;

        json_fns!({
            json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
            use trait_mod;
            |self, w| {
                use trait_mod::XHelpers as _;
                ArrayOfIter::<::alloc::vec::IntoIter<T>>(self.into_iter()).json_provide_into_x(w)
            }
        });

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
    }
});

impl<T: ToJson> ToJson for Vec<T> {
    type ToJsonKind = json_kinds::Array;
    json_fns!({
        json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
        use trait_mod;
        |&self, w| <&[T] as trait_mod::XHelpers>::json_provide_into_x(self, w)
    });
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}
