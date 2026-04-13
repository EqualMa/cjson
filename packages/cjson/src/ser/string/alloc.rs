use alloc::string::String;

use crate::ser::{ToJson, ToJsonString};

impl ToJson for String {
    type ToJson<'a>
        = <Self as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_string(self)
    }
}

impl ToJsonString for String {
    type ToJsonString<'a>
        = <&'a str as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        str::to_json_string(self)
    }
}
