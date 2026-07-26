use crate::ser::{IntoJson, ToJson2, helpers::json_fns};

pub mod prelude_refed {}

enum Never {}
pub struct Refed<T>(Never, ::core::marker::PhantomData<T>);

impl<T: ToJson2> IntoJson for Refed<T> {
    type JsonKind = T::ToJsonKind;

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        use trait_mod;
        |self, _| trait_mod::never_future!(match self.0 {})
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = <T as ToJson2>::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}
