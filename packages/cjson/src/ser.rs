pub mod iter_text_chunk;
pub mod texts;
pub mod traits;

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

mod slice;
