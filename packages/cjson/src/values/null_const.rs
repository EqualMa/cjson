use crate::ser::texts::literal_names;
use crate::{
    r#const::{ConstIntoJson, ConstIntoJsonValueString},
    ser::texts,
};

use super::Null;

impl ConstIntoJson<Null> {
    pub const fn const_into_json(self) -> Null {
        self.0
    }
}

impl ConstIntoJsonValueString<Null> {
    pub const fn const_into_json_value_string_len(self) -> usize {
        self.const_into_json_value_string_without_const_len()
            .inner()
            .len()
    }

    pub const fn const_into_json_value_string<const LEN: usize>(
        self,
    ) -> texts::Value<&'static str> {
        assert!(JSON_NULL.inner().len() == LEN);
        JSON_NULL
    }

    pub const fn const_into_json_value_string_without_const_len(
        self,
    ) -> texts::Value<&'static str> {
        JSON_NULL
    }
}

const JSON_NULL: texts::Value<&'static str> = literal_names::NullChunk::JSON_STR;
