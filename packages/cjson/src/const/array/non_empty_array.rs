use crate::{
    r#const::{RuntimeChunkSurroundedWithCompileTime, State},
    ser::traits::{self, IntoTextChunks},
};

use super::NonEmptyArray;

pub struct NonEmptyArraySer<'a, ARR: 'a + RuntimeChunkSurroundedWithCompileTime>(
    ARR::ToIntoTextChunks<'a>,
);

impl<'a, C: 'a + RuntimeChunkSurroundedWithCompileTime> NonEmptyArraySer<'a, C> {
    pub(super) fn from_non_empty_array(arr: &'a NonEmptyArray<C>) -> Self {
        const { () = Self::ASSERT }
        Self(arr.0.inner().to_into_text_chunks())
    }

    pub(super) const ASSERT: () = NonEmptyItems::<'a, C>::ASSERT;
}

impl<'a, C: 'a + RuntimeChunkSurroundedWithCompileTime> IntoTextChunks for NonEmptyArraySer<'a, C> {
    type IntoTextChunks = <C::ToIntoTextChunks<'a> as IntoTextChunks>::IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self.0.into_text_chunks()
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.0._private_into_text_chunks_vec()
    }
}

impl<C: RuntimeChunkSurroundedWithCompileTime> traits::sealed::Text for NonEmptyArraySer<'_, C> {}
impl<C: RuntimeChunkSurroundedWithCompileTime> traits::Text for NonEmptyArraySer<'_, C> {}
impl<C: RuntimeChunkSurroundedWithCompileTime> traits::sealed::Value for NonEmptyArraySer<'_, C> {}
impl<C: RuntimeChunkSurroundedWithCompileTime> traits::Value for NonEmptyArraySer<'_, C> {}
impl<C: RuntimeChunkSurroundedWithCompileTime> traits::sealed::Array for NonEmptyArraySer<'_, C> {}
impl<'a, C: RuntimeChunkSurroundedWithCompileTime> traits::Array for NonEmptyArraySer<'a, C> {
    type IntoCommaSeparatedElements = NonEmptyItems<'a, C>;

    /// `C::RemoveSurroundingGroup` has been validated in [Self::new](NonEmptyArray::new).
    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
        NonEmptyItems(C::ungroup_text_chunks(self.0))
    }
}

/// [`ITEMS`] must be [`<_ as RuntimeChunkSurroundedWithCompileTime>::RemoveSurroundingGroup`](RuntimeChunkSurroundedWithCompileTime::RemoveSurroundingGroup)
pub struct NonEmptyItems<'a, ARR: 'a + RuntimeChunkSurroundedWithCompileTime>(
    ARR::UngroupTextChunks<'a>,
);

impl<'a, ARR: 'a + RuntimeChunkSurroundedWithCompileTime> NonEmptyItems<'a, ARR> {
    const ASSERT: () = {
        let (prev_state, next_state) = ARR::UNGROUPED_STATES;
        prev_state.assert_same(State::INIT_AFTER_ARRAY_START);
        next_state.assert_same(State::INIT_AFTER_ARRAY_ITEM);
    };
}

impl<'a, ARR: 'a + RuntimeChunkSurroundedWithCompileTime> IntoTextChunks
    for NonEmptyItems<'a, ARR>
{
    type IntoTextChunks = <ARR::UngroupTextChunks<'a> as IntoTextChunks>::IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self.0.into_text_chunks()
    }

    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.0._private_into_text_chunks_vec()
    }
}

impl<ITEMS: RuntimeChunkSurroundedWithCompileTime> traits::sealed::EmptyOrCommaSeparatedElements
    for NonEmptyItems<'_, ITEMS>
{
}
impl<ITEMS: RuntimeChunkSurroundedWithCompileTime> traits::EmptyOrCommaSeparatedElements
    for NonEmptyItems<'_, ITEMS>
{
    traits::impl_EmptyOrCommaSeparatedElements_for_NonEmptyCommaSeparatedElements! {}
}

impl<ITEMS: RuntimeChunkSurroundedWithCompileTime> traits::sealed::NonEmptyCommaSeparatedElements
    for NonEmptyItems<'_, ITEMS>
{
}
impl<ITEMS: RuntimeChunkSurroundedWithCompileTime> traits::NonEmptyCommaSeparatedElements
    for NonEmptyItems<'_, ITEMS>
{
}
