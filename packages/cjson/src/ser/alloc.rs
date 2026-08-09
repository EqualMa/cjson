use alloc::{boxed::Box, rc::Rc, sync::Arc};

use crate::utils::impl_many;

use super::{IntoJson, ToJson as ToJson, helpers::json_fns};

/// - For Sized T, we just unbox it.
/// - For `Box<[T]>`, same as `Vec<T>`
/// - For `Box<dyn __>`, // TODO:
/// - For `Box<OtherUnsizedTypes>`, we allow downstream crates to implement IntoJson
impl<T: IntoJson> IntoJson for Box<T> {
    type JsonKind = T::JsonKind;
    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        use trait_mod;
        |self, w| <T as trait_mod::XHelpers>::json_provide_into_x(*self, w)
    });
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = T::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}

// TODO: should we impl IntoJson for Rc/Arc<impl ToJson> ?

impl_many!({
    {
        // TODO: downstream crates may implement trait `core::marker::Copy` for type `alloc::boxed::Box<_>`
        // {
        //     use Box as POINTER;
        // }
        {
            use Rc as POINTER;
        }
        {
            use Arc as POINTER;
        }
    }

    impl<T: ?Sized + ToJson> ToJson for POINTER<T> {
        type ToJsonKind = T::ToJsonKind;
        json_fns!({
            json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
            use trait_mod;
            |&self, w| <T as trait_mod::XHelpers>::json_provide_to_x(self, w)
        });
        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = T::IS_CHAINABLE_AND_ALWAYS_EMPTY;
    }
});
