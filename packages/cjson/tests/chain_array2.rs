use cjson::ser::{
    ConsumeChained as _, ConsumeJson, Consumed, IntoJson,
    json_kinds::{self, JsonKind},
};

pub struct ChainArray2<
    A: IntoJson<JsonKind = json_kinds::Array>,
    B: IntoJson<JsonKind = json_kinds::Array>,
>(pub A, pub B);

impl<A: IntoJson<JsonKind = json_kinds::Array>, B: IntoJson<JsonKind = json_kinds::Array>> IntoJson
    for ChainArray2<A, B>
{
    type JsonKind = json_kinds::Array;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        if const { A::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
            self.1.json_provide_into(w)
        } else if const { B::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
            self.0.json_provide_into(w)
        } else {
            let Self(a, b) = self;
            let mut w = w.start_to_consume_chained_arrays(());
            w.extend(a);
            w.end_with(b)
        }
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
        A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}
