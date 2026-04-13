use crate::r#const::ConstAsJsonValueStr;

use super::Value;

impl<'s> Value<&'s str> {
    /// Panics if `s` is not a json value or `s` contains any json whitespaces.
    pub const fn new_no_json_whitespace(s: &'s str) -> Self {
        crate::r#const::assert_json_value(s);
        Self::new_without_validation(s)
    }

    pub(crate) const EMPTY_ARRAY: Self = Self("[]");
    pub(crate) const EMPTY_OBJECT: Self = Self("{}");
}

impl<'s> ConstAsJsonValueStr<Value<&'s str>> {
    pub const fn const_as_json_value_str(&self) -> Value<&'s str> {
        self.0
    }
}
