use crate::ser::{IntoJson, json_kinds, traits};

use super::Value;

impl<T: traits::IntoTextChunks> IntoJson for Value<T> {
    type JsonKind = json_kinds::AnyValue;

    fn json_provide_into<
        W: crate::ser::ConsumeJson<
                ConsumeJsonKind: crate::ser::json_kinds::JsonKind<Contains<Self::JsonKind> = ()>,
            >,
    >(
        self,
        w: W,
    ) -> crate::ser::Consumed<Self::JsonKind, W> {
        w.consume_any_value(self, ())
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}
