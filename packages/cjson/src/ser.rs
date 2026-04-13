pub mod iter_text_chunk;
pub mod texts;
pub mod traits;

pub mod exts;

pub trait ToJson {
    type ToJson<'a>: traits::Text
    where
        Self: 'a;
    fn to_json(&self) -> Self::ToJson<'_>;
}

impl<'this, T: ?Sized + ToJson> ToJson for &'this T {
    type ToJson<'a>
        = T::ToJson<'this>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        T::to_json(self)
    }
}

pub trait ToJsonString {
    type ToJsonString<'a>: traits::JsonString
    where
        Self: 'a;
    fn to_json_string(&self) -> Self::ToJsonString<'_>;
}

impl<'this, T: ?Sized + ToJsonString> ToJsonString for &'this T {
    type ToJsonString<'a>
        = T::ToJsonString<'this>
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        T::to_json_string(self)
    }
}

pub trait ToJsonArray: ToJson {
    type ToJsonArray<'a>: traits::Array
    where
        Self: 'a;
    fn to_json_array(&self) -> Self::ToJsonArray<'_>;
}

impl<'this, T: ?Sized + ToJsonArray> ToJsonArray for &'this T {
    type ToJsonArray<'a>
        = T::ToJsonArray<'this>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        T::to_json_array(self)
    }
}

pub trait ToJsonObject: ToJson {
    type ToJsonObject<'a>: traits::Object
    where
        Self: 'a;
    fn to_json_object(&self) -> Self::ToJsonObject<'_>;
}

impl<'this, T: ?Sized + ToJsonObject> ToJsonObject for &'this T {
    type ToJsonObject<'a>
        = T::ToJsonObject<'this>
    where
        Self: 'a;

    fn to_json_object(&self) -> Self::ToJsonObject<'_> {
        T::to_json_object(self)
    }
}

mod bool;
mod int;
mod string;

mod slice;
