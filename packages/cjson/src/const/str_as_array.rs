use super::str_as_array_vec::StrAsArrayVec;

pub(crate) struct StrAsArray<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> StrAsArray<LEN> {
    pub(crate) const fn as_str(&self) -> &str {
        // SAFETY: self.0 is valid utf8 string
        unsafe { str::from_utf8_unchecked(&self.0) }
    }

    pub(crate) const fn from_array_vec_assert_len_is_cap(v: StrAsArrayVec<LEN>) -> Self {
        assert!(v.len() == LEN);
        let buf = v.into_buf();
        debug_assert!(str::from_utf8(&buf).is_ok());
        Self(buf)
    }
}
