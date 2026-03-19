use crate::{
    ser::{
        iter_text_chunk::EitherTextChunks,
        traits::{self, IntoTextChunks},
    },
    values::Either,
};

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

impl<A: traits::Text, B: traits::Text> traits::sealed::Text for Either<A, B> {}
impl<A: traits::Text, B: traits::Text> traits::Text for Either<A, B> {}

impl<A: traits::Value, B: traits::Value> traits::sealed::Value for Either<A, B> {}
impl<A: traits::Value, B: traits::Value> traits::Value for Either<A, B> {}

impl<A: traits::JsonStringFragment, B: traits::JsonStringFragment>
    traits::sealed::JsonStringFragment for Either<A, B>
{
}
impl<A: traits::JsonStringFragment, B: traits::JsonStringFragment> traits::JsonStringFragment
    for Either<A, B>
{
}
