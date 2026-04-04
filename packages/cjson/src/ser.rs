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

pub trait ToJsonStringFragment {
    type ToJsonStringFragment<'a>: traits::JsonStringFragment
    where
        Self: 'a;
    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_>;
}

impl<'this, T: ?Sized + ToJsonStringFragment> ToJsonStringFragment for &'this T {
    type ToJsonStringFragment<'a>
        = T::ToJsonStringFragment<'this>
    where
        Self: 'a;

    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_> {
        T::to_json_string_fragment(self)
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

mod bool;
mod int;
mod string;

mod slice;
