use crate::ser::IntoJson;

use super::{helpers::json_fns, json_kinds, texts};

impl IntoJson for bool {
    type JsonKind = json_kinds::AnyValue;

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        |self, w| w.consume_any_value(texts::Value::bool(self), ())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

impl super::ToJsonByCopyIntoJson for bool {}

mod r#const {
    use crate::{r#const::ConstIntoJson, ser::texts};

    impl ConstIntoJson<bool> {
        pub const fn const_into_json(self) -> texts::Boolean {
            texts::Boolean(self.0)
        }
    }
}
