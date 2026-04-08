use crate::ser::traits::{self, EmptyOrCommaSeparatedElements, IntoTextChunks, sealed};

use super::NonEmptyCommaSeparatedItems;

impl<T: IntoTextChunks> sealed::EmptyOrCommaSeparatedElements for NonEmptyCommaSeparatedItems<T> {}
impl<T: IntoTextChunks> EmptyOrCommaSeparatedElements for NonEmptyCommaSeparatedItems<T> {
    traits::impl_EmptyOrCommaSeparatedElements_for_NonEmptyCommaSeparatedElements! {}
}
