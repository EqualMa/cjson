use crate::r#const::{ConstAsJsonValueStr, array_string::ArrayString};

use super::{Number, Value};

impl<const CAP: usize> ConstAsJsonValueStr<Number<ArrayString<u8, CAP>>> {
    pub const fn const_as_json_value_str(&self) -> Value<&str> {
        Value::new_without_validation(self.0.inner().as_str())
    }
}
