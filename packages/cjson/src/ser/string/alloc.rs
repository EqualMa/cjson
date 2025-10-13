use alloc::string::String;

use crate::ser::ToJson;

impl ToJson for String {
    type ToJson<'a>
        = <&'a str as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        str::to_json(self)
    }
}
