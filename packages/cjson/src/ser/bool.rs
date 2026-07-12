use crate::ser::{IntoJson, json_kinds::JsonKind};

use super::{ConsumeJson, Consumed, ToJson, json_kinds, texts};

impl ToJson for bool {
    type ToJson<'a>
        = texts::Boolean
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        texts::Boolean(*self)
    }
}

impl IntoJson for bool {
    type JsonKind = json_kinds::AnyValue;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        w.consume_any_value(texts::Value::bool(self), ())
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

mod r#const {
    use crate::{r#const::ConstIntoJson, ser::texts};

    impl ConstIntoJson<bool> {
        pub const fn const_into_json(self) -> texts::Boolean {
            texts::Boolean(self.0)
        }
    }
}
