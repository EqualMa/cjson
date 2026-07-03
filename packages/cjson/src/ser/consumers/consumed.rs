use core::marker::PhantomData;

use crate::ser::json_kinds;

use super::json_kinds::JsonKind;

pub struct Consumed<K: JsonKind, W: ?Sized>(K, PhantomData<W>);

impl<W: ?Sized> Consumed<json_kinds::JsonString, W> {
    pub(super) const ASSERT_STRING: Self = Consumed(json_kinds::JsonString, PhantomData);
}

impl<W: ?Sized> Consumed<json_kinds::Array, W> {
    pub(super) const ASSERT_ARRAY: Self = Consumed(json_kinds::Array, PhantomData);
}

impl<W: ?Sized> Consumed<json_kinds::Object, W> {
    pub(super) const ASSERT_OBJECT: Self = Consumed(json_kinds::Object, PhantomData);
}
