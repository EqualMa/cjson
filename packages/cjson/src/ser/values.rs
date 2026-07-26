use super::{IntoJson, ToJsonByCopyIntoJson, consumers::json_kinds, helpers::json_fns};

#[derive(Debug, Clone, Copy)]
pub struct EmptyArray;
#[derive(Debug, Clone, Copy)]
pub struct EmptyObject;

impl IntoJson for EmptyArray {
    type JsonKind = json_kinds::Array;

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        |self, w| w.consume_empty_array(())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = true;
}

impl ToJsonByCopyIntoJson for EmptyArray {}

impl IntoJson for EmptyObject {
    type JsonKind = json_kinds::Object;

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        |self, w| w.consume_empty_object(())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = true;
}

impl ToJsonByCopyIntoJson for EmptyObject {}
