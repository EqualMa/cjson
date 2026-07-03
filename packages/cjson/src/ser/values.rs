use super::{
    IntoJson,
    consumers::{
        ConsumeJson, Consumed,
        json_kinds::{self, JsonKind},
    },
};

pub struct EmptyArray;
pub struct EmptyObject;

impl IntoJson for EmptyArray {
    type JsonKind = json_kinds::Array;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        w.consume_empty_array(())
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = true;
}

impl IntoJson for EmptyObject {
    type JsonKind = json_kinds::Object;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        w.consume_empty_object(())
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = true;
}
