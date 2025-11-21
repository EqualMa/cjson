use crate::ser::texts;

use super::ConstAsJsonValueStr;

pub struct JsonValueArrayStr<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> JsonValueArrayStr<LEN> {
    pub(crate) const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub(crate) const fn new_without_validation(bytes: [u8; LEN]) -> Self {
        debug_assert!(str::from_utf8(&bytes).is_ok()); // not sufficient
        Self(bytes)
    }
}

impl<const LEN: usize> ConstAsJsonValueStr<JsonValueArrayStr<LEN>> {
    pub const fn const_as_json_value_str(&self) -> texts::Value<&str> {
        texts::Value::new_without_validation({
            let bytes = &self.0.0;
            // SAFETY: JsonValueArrayStr is valid UTF-8
            unsafe { str::from_utf8_unchecked(bytes) }
        })
    }
}
