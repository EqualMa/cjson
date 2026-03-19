use crate::{ToJson, ser::ToJsonStringFragment};

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

impl<T: ToJsonStringFragment> ToJsonStringFragment for Option<T> {
    type ToJsonStringFragment<'a>
        = Either<T::ToJsonStringFragment<'a>, crate::ser::texts::Empty>
    where
        Self: 'a;

    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_> {
        match self {
            Some(this) => Either::A(T::to_json_string_fragment(this)),
            None => Either::B(crate::ser::texts::Empty),
        }
    }
}
