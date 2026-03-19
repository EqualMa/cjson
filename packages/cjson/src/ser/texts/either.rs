use crate::{
    ser::{
        iter_text_chunk::EitherTextChunks,
        traits::{self, IntoTextChunks},
    },
    utils::impl_many,
};

macro_rules! derive_either_one {
    ($Trait:ident) => {
        impl<A: traits::$Trait, B: traits::$Trait> traits::sealed::$Trait for Either<A, B> {}
        impl<A: traits::$Trait, B: traits::$Trait> traits::$Trait for Either<A, B> {}
    };
}

macro_rules! derive_either {
    ($($Trait:ident),+ $(,)?) => {
        $(derive_either_one! {
            $Trait
        })+
    };
}

impl_many!({
    {
        {
            use crate::values::Either;
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

    impl<A: IntoTextChunks, B: IntoTextChunks> IntoTextChunks for Either<A, B> {
        type IntoTextChunks = EitherTextChunks<A::IntoTextChunks, B::IntoTextChunks>;

        fn into_text_chunks(self) -> Self::IntoTextChunks {
            match self {
                EitherA!(this) => EitherTextChunks::A(A::into_text_chunks(this)),
                EitherB!(this) => EitherTextChunks::B(B::into_text_chunks(this)),
            }
        }

        #[cfg(feature = "alloc")]
        fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
            match self {
                EitherA!(this) => A::_private_into_text_chunks_vec(this),
                EitherB!(this) => B::_private_into_text_chunks_vec(this),
            }
        }
    }

    derive_either!(
        //
        Text,
        Value,
        JsonStringFragment,
    );
});
