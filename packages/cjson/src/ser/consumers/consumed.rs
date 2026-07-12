use core::marker::PhantomData;

use crate::ser::json_kinds;

use super::json_kinds::JsonKind;

pub struct Consumed<K: JsonKind, W: ?Sized>(K, PhantomData<W>);

impl<K: JsonKind, W: ?Sized> Consumed<K, W> {
    pub(super) const fn assert(kind: K) -> Self {
        Consumed(kind, PhantomData)
    }
}

impl<W: ?Sized> Consumed<json_kinds::AnyValue, W> {
    pub(super) const ASSERT_ANY_VALUE: Self = Consumed(json_kinds::AnyValue, PhantomData);
}

impl<W: ?Sized> Consumed<json_kinds::JsonString, W> {
    pub(super) const ASSERT_STRING: Self = Consumed(json_kinds::JsonString, PhantomData);
}

impl<W: ?Sized> Consumed<json_kinds::Array, W> {
    pub(super) const ASSERT_ARRAY: Self = Consumed(json_kinds::Array, PhantomData);
}

impl<W: ?Sized> Consumed<json_kinds::Object, W> {
    pub(super) const ASSERT_OBJECT: Self = Consumed(json_kinds::Object, PhantomData);
}
