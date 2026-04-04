use crate::{
    ToJson,
    ser::{ToJsonArray, ToJsonStringFragment},
};

use super::Never;

impl ToJson for Never {
    type ToJson<'a>
        = Self
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        match *self {}
    }
}

impl ToJsonStringFragment for Never {
    type ToJsonStringFragment<'a>
        = Self
    where
        Self: 'a;

    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_> {
        match *self {}
    }
}

impl ToJsonArray for Never {
    type ToJsonArray<'a>
        = Self
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        match *self {}
    }
}
