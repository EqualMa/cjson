use cjson::{
    impl_json, json_fns,
    ser::{IntoJson, IntoJsonArray, ToJsonArray2 as ToJsonArray, json_kinds},
};

pub struct ChainArray2<
    A: IntoJson<JsonKind = json_kinds::Array>,
    B: IntoJson<JsonKind = json_kinds::Array>,
>(pub A, pub B);

impl<A: IntoJson<JsonKind = json_kinds::Array>, B: IntoJson<JsonKind = json_kinds::Array>> IntoJson
    for ChainArray2<A, B>
{
    type JsonKind = json_kinds::Array;
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
        A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;

    json_fns!(|self| #[json_x]
    if const { A::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
        json_x!((self.1))
    } else if const { B::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
        json_x!((self.0))
    } else {
        json_x!([..(self.0), ..(self.1)])
    });
}

pub struct ChainArray2Both<A, B>(pub A, pub B);

impl_json!(
    impl_generics![A, B],
    where_clause_to![A: ToJsonArray, B: ToJsonArray],
    where_clause_into![A: IntoJsonArray, B: IntoJsonArray],
    JsonKind![json_kinds::Array],
    IS_CHAINABLE_AND_ALWAYS_EMPTY![
        A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY
    ],
    |self: ChainArray2Both<A, B>| #[json_x(macro(json_x))]
    if const { A::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
        json_x!((auto_ref!(self.1)))
    } else if const { B::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
        json_x!((auto_ref!(self.0)))
    } else {
        json_x!([..(auto_ref!(self.0)), ..(auto_ref!(self.1))])
    }
);
