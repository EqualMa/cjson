use crate::{
    ser::{
        iter_text_chunk::EitherTextChunks,
        traits::{self, IntoTextChunks},
    },
    values::Either,
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

impl<A: IntoTextChunks, B: IntoTextChunks> IntoTextChunks for Either<A, B> {
    type IntoTextChunks = EitherTextChunks<A::IntoTextChunks, B::IntoTextChunks>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        match self {
            Either::A(this) => EitherTextChunks::A(A::into_text_chunks(this)),
            Either::B(this) => EitherTextChunks::B(B::into_text_chunks(this)),
        }
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        match self {
            Either::A(this) => A::_private_into_text_chunks_vec(this),
            Either::B(this) => B::_private_into_text_chunks_vec(this),
        }
    }
}

derive_either!(
    //
    Text,
    Value,
    JsonStringFragment,
);

impl<A: traits::Array, B: traits::Array> traits::sealed::Array for Either<A, B> {}
impl<A: traits::Array, B: traits::Array> traits::Array for Either<A, B> {
    type IntoCommaSeparatedElements =
        Either<A::IntoCommaSeparatedElements, B::IntoCommaSeparatedElements>;
    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
        match self {
            Either::A(this) => Either::A(A::into_comma_separated_elements(this)),
            Either::B(this) => Either::B(B::into_comma_separated_elements(this)),
        }
    }
}

impl<A: traits::JsonString, B: traits::JsonString> traits::sealed::JsonString for Either<A, B> {}
impl<A: traits::JsonString, B: traits::JsonString> traits::JsonString for Either<A, B> {
    type IntoJsonStringFragments = Either<A::IntoJsonStringFragments, B::IntoJsonStringFragments>;

    fn into_json_string_fragments(self) -> Self::IntoJsonStringFragments {
        match self {
            Either::A(this) => Either::A(A::into_json_string_fragments(this)),
            Either::B(this) => Either::B(B::into_json_string_fragments(this)),
        }
    }
}

mod crate_either;
