use crate::{ToJson, ser::ToJsonArray, ser::ToJsonStringFragment, utils::impl_many};

impl_many!({
    {
        {
            use super::Either;
            macro_rules! EitherA { [$($t:tt)*] => [Either::A($($t)*)] }
            macro_rules! EitherB { [$($t:tt)*] => [Either::B($($t)*)] }
        }
        #[cfg(feature = "either")]
        {
            use ::either::Either;
            macro_rules! EitherA { [$($t:tt)*] => [Either::Left ($($t)*)] }
            macro_rules! EitherB { [$($t:tt)*] => [Either::Right($($t)*)] }
        }
    }

    impl<A: ToJson, B: ToJson> ToJson for Either<A, B> {
        type ToJson<'a>
            = Either<A::ToJson<'a>, B::ToJson<'a>>
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            match self {
                EitherA!(this) => EitherA!(A::to_json(this)),
                EitherB!(this) => EitherB!(B::to_json(this)),
            }
        }
    }

    impl<A: ToJsonArray, B: ToJsonArray> ToJsonArray for Either<A, B> {
        type ToJsonArray<'a>
            = Either<A::ToJsonArray<'a>, B::ToJsonArray<'a>>
        where
            Self: 'a;

        fn to_json_array(&self) -> Self::ToJsonArray<'_> {
            match self {
                EitherA!(this) => EitherA!(A::to_json_array(this)),
                EitherB!(this) => EitherB!(B::to_json_array(this)),
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
                EitherA!(this) => EitherA!(A::to_json_string_fragment(this)),
                EitherB!(this) => EitherB!(B::to_json_string_fragment(this)),
            }
        }
    }
});
