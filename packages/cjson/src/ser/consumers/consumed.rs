use core::marker::PhantomData;

use crate::ser::json_kinds;

use super::{
    ConsumeJsonText, json_kinds::JsonKind, writer_assert::WriterAssertIsFromConsumeJsonText,
};

pub struct Consumed<K: JsonKind, W: ?Sized>(K, PhantomData<W>);

impl<K: JsonKind, W: ?Sized> Consumed<K, W> {
    pub(super) const fn assert(kind: K) -> Self {
        Consumed(kind, PhantomData)
    }

    pub(crate) fn upcast<A: JsonKind<Contains<K> = ()>>(self) -> Consumed<A, W> {
        Consumed(self.0.upcast(), self.1)
    }

    // pub fn upcast_to_any_value(self) -> Consumed<json_kinds::AnyValue, W> {
    //     Consumed(self.0.upcast(), self.1)
    // }
}

impl<K: JsonKind, Writer> Consumed<K, ConsumeJsonText<Writer>> {
    pub fn assert_consume_json_text_and_upcast_to_any_value<C>(
        self,
    ) -> Consumed<json_kinds::AnyValue, C>
    where
        Writer: WriterAssertIsFromConsumeJsonText<C, ()>,
    {
        Writer::writer_assert_is_from_consume_json_text(self.upcast::<json_kinds::AnyValue>(), ())
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
