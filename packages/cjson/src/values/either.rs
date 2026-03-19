use crate::{ToJson, ser::ToJsonStringFragment};

use super::Either;

impl<A: ToJson, B: ToJson> ToJson for Either<A, B> {
    type ToJson<'a>
        = Either<A::ToJson<'a>, B::ToJson<'a>>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        match self {
            Either::A(this) => Either::A(A::to_json(this)),
            Either::B(this) => Either::B(B::to_json(this)),
        }
    }
}

impl<A: ToJsonStringFragment, B: ToJsonStringFragment> ToJsonStringFragment for Either<A, B> {
    type ToJsonStringFragment<'a>
        = Either<A::ToJsonStringFragment<'a>, B::ToJsonStringFragment<'a>>
    where
        Self: 'a;

    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_> {
        match self {
            Either::A(this) => Either::A(A::to_json_string_fragment(this)),
            Either::B(this) => Either::B(B::to_json_string_fragment(this)),
        }
    }
}
