use crate::r#const::ConstAsJsonValueStr;

use super::Value;

impl<'s> ConstAsJsonValueStr<Value<&'s str>> {
    pub const fn const_as_json_value_str(&self) -> Value<&'s str> {
        self.0
    }
}
