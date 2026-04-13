use crate::{
    ToJson, ser::ToJsonArray, ser::ToJsonString, utils::impl_many, values::Either as CrateEither,
};

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
            = CrateEither<A::ToJson<'a>, B::ToJson<'a>>
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            match self {
                EitherA!(this) => CrateEither::A(A::to_json(this)),
                EitherB!(this) => CrateEither::B(B::to_json(this)),
            }
        }
    }

    impl<A: ToJsonArray, B: ToJsonArray> ToJsonArray for Either<A, B> {
        type ToJsonArray<'a>
            = CrateEither<A::ToJsonArray<'a>, B::ToJsonArray<'a>>
        where
            Self: 'a;

        fn to_json_array(&self) -> Self::ToJsonArray<'_> {
            match self {
                EitherA!(this) => CrateEither::A(A::to_json_array(this)),
                EitherB!(this) => CrateEither::B(B::to_json_array(this)),
            }
        }
    }

    impl<A: ToJsonString, B: ToJsonString> ToJsonString for Either<A, B> {
        type ToJsonString<'a>
            = CrateEither<A::ToJsonString<'a>, B::ToJsonString<'a>>
        where
            Self: 'a;

        fn to_json_string(&self) -> Self::ToJsonString<'_> {
            match self {
                EitherA!(this) => CrateEither::A(A::to_json_string(this)),
                EitherB!(this) => CrateEither::B(B::to_json_string(this)),
            }
        }
    }
});
