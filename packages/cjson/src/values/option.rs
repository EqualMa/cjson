use crate::ToJson;

use super::{Either, Null};

impl<T: ToJson> ToJson for Option<T> {
    type ToJson<'a>
        = Either<T::ToJson<'a>, Null>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        match self {
            Some(this) => Either::A(T::to_json(this)),
            None => Either::B(Null),
        }
    }
}
