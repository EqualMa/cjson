use crate::{
    ToJson,
    ser::{ToJsonArray, ToJsonString},
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

impl ToJsonString for Never {
    type ToJsonString<'a>
        = Self
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
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
