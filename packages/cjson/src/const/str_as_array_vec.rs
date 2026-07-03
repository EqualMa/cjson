use crate::{r#const::str_as_array::StrAsArray, ser::texts};

pub(crate) struct StrAsArrayVec<const CAP: usize> {
    buf: [u8; CAP],
    len: usize,
}

impl<const CAP: usize> StrAsArrayVec<CAP> {
    pub const DEFAULT: Self = Self {
        buf: [0u8; CAP],
        len: 0,
    };

    const fn with_byte(mut self, b: u8) -> Self {
        let (_, rest) = self.buf.split_at_mut(self.len);
        let (insert, _) = rest.split_first_mut().expect("not full");
        *insert = b;

        self.len += 1;

        self
    }

    pub const fn left_bracket(self) -> Self {
        self.with_byte(b'[')
    }

    pub const fn right_bracket(self) -> Self {
        self.with_byte(b']')
    }

    pub const fn left_brace(self) -> Self {
        self.with_byte(b'{')
    }

    pub const fn right_brace(self) -> Self {
        self.with_byte(b'}')
    }

    pub const fn comma(self) -> Self {
        self.with_byte(b',')
    }

    pub const fn colon(self) -> Self {
        self.with_byte(b':')
    }

    const fn with_bytes(mut self, bytes: &[u8]) -> Self {
        let (_, rest) = self.buf.split_at_mut(self.len);
        let (insert, _) = rest.split_at_mut(bytes.len());
        insert.copy_from_slice(bytes);
        self.len += bytes.len();
        self
    }

    const fn with_str(self, s: &str) -> Self {
        self.with_bytes(s.as_bytes())
    }

    pub const fn json_value(self, value: texts::Value<&'_ str>) -> Self {
        self.with_str(value.inner())
    }

    pub const fn double_quote(self) -> Self {
        self.with_byte(b'"')
    }

    pub(crate) const fn json_string_fragments(self, chunk: &[u8]) -> Self {
        self.with_bytes(chunk)
    }

    pub const fn assert_len_is_cap(self) -> StrAsArray<CAP> {
        StrAsArray::from_array_vec_assert_len_is_cap(self)
    }

    pub const fn as_str(&self) -> &str {
        let s = self.buf.split_at(self.len).0;
        debug_assert!(str::from_utf8(s).is_ok());
        // SAFETY: self.buf if valid json utf8 chunk
        unsafe { str::from_utf8_unchecked(s) }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub(crate) const fn into_buf(self) -> [u8; CAP] {
        self.buf
    }
}
