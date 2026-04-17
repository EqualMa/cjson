use alloc::{rc::Rc, vec::Vec};

use crate::ser::{ToJson, ToJsonArray};

impl<T: ToJson> ToJson for Vec<T> {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}

impl<T: ToJson> ToJsonArray for Vec<T> {
    type ToJsonArray<'a>
        = <[T] as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <[T] as ToJsonArray>::to_json_array(self)
    }
}
