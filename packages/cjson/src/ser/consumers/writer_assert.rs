use super::{
    ConsumeJsonText,
    consumed::Consumed,
    json_kinds::JsonKind,
    yes_or_no::{No, YesOrNo},
};

pub trait WriterAssertIsFromConsumeJsonText<This: ?Sized, YN: YesOrNo>: Sized {
    fn writer_assert_is_from_consume_json_text<K: JsonKind>(
        consumed: Consumed<K, ConsumeJsonText<Self>>,
        yes: YN,
    ) -> Consumed<K, This>;
}

impl<W> WriterAssertIsFromConsumeJsonText<ConsumeJsonText<W>, ()> for W {
    fn writer_assert_is_from_consume_json_text<K: JsonKind>(
        consumed: Consumed<K, ConsumeJsonText<W>>,
        (): (),
    ) -> Consumed<K, ConsumeJsonText<W>> {
        consumed
    }
}

impl<C, W> WriterAssertIsFromConsumeJsonText<C, No> for W {
    fn writer_assert_is_from_consume_json_text<K: JsonKind>(
        _: Consumed<K, ConsumeJsonText<Self>>,
        yes: No,
    ) -> Consumed<K, C> {
        match yes {}
    }
}
