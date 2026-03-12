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

impl<T: ?Sized + ToJson> ToJson for &T {
    type ToJson<'a>
        = T::ToJson<'a>
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

impl<T: ?Sized + ToJsonStringFragment> ToJsonStringFragment for &T {
    type ToJsonStringFragment<'a>
        = T::ToJsonStringFragment<'a>
    where
        Self: 'a;

    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_> {
        T::to_json_string_fragment(self)
    }
}

mod bool;
mod int;
mod string;

mod slice;
