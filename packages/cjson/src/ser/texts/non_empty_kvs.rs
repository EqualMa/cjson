use crate::ser::traits::{self, IntoTextChunks, Kvs, sealed};

use super::NonEmptyKvs;

impl<T: IntoTextChunks> sealed::Kvs for NonEmptyKvs<T> {}
impl<T: IntoTextChunks> Kvs for NonEmptyKvs<T> {
    traits::impl_Kvs_for_NonEmptyKvs! {}
}
