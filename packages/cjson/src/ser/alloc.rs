use alloc::{boxed::Box, rc::Rc};

use crate::utils::impl_many;

use super::{ToJson, ToJsonArray, ToJsonObject, ToJsonString};

impl_many!({
    {
        {
            use Box as BoxOrRc;
        }
        {
            use Rc as BoxOrRc;
        }
    }

    impl<T: ?Sized + ToJson> ToJson for BoxOrRc<T> {
        type ToJson<'a>
            = T::ToJson<'a>
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            T::to_json(self)
        }
    }

    impl<T: ?Sized + ToJsonString> ToJsonString for BoxOrRc<T> {
        type ToJsonString<'a>
            = T::ToJsonString<'a>
        where
            Self: 'a;

        fn to_json_string(&self) -> Self::ToJsonString<'_> {
            T::to_json_string(self)
        }
    }

    impl<T: ?Sized + ToJsonArray> ToJsonArray for BoxOrRc<T> {
        type ToJsonArray<'a>
            = T::ToJsonArray<'a>
        where
            Self: 'a;

        fn to_json_array(&self) -> Self::ToJsonArray<'_> {
            T::to_json_array(self)
        }
    }

    impl<T: ?Sized + ToJsonObject> ToJsonObject for BoxOrRc<T> {
        type ToJsonObject<'a>
            = T::ToJsonObject<'a>
        where
            Self: 'a;

        fn to_json_object(&self) -> Self::ToJsonObject<'_> {
            T::to_json_object(self)
        }
    }
});
