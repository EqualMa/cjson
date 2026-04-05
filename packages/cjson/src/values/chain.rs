use crate::{
    ToJson,
    ser::{
        ToJsonArray,
        texts::{self, CommaSeparated},
        traits::{self, Array},
    },
};

use super::ChainArray;

impl<A: ToJsonArray, B: ToJsonArray> ToJson for ChainArray<A, B> {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}
impl<A: ToJsonArray, B: ToJsonArray> ToJsonArray for ChainArray<A, B> {
    type ToJsonArray<'a>
        = texts::Bracketed<
        CommaSeparated<
            <A::ToJsonArray<'a> as traits::Array>::IntoCommaSeparatedElements,
            <B::ToJsonArray<'a> as traits::Array>::IntoCommaSeparatedElements,
        >,
    >
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        texts::Bracketed(CommaSeparated(
            self.0.to_json_array().into_comma_separated_elements(),
            self.1.to_json_array().into_comma_separated_elements(),
        ))
    }
}
