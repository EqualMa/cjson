use super::{
    State, stated_str_as_array_vec::StatedChunkBuf, str_as_array::StrAsArray,
    str_as_array_vec::StrAsArrayVec,
};

/// The chunk represents an non-empty json array;
/// The chunk is represented as a byte array.
pub struct NonEmptyArrayAsArray<const LEN: usize>(StrAsArray<LEN>);
pub struct NonEmptyArrayAsArrayVec<const CAP: usize>(StrAsArrayVec<CAP>);
pub struct NonEmptyArrayAsStr<'a>(&'a str);

impl<const LEN: usize> NonEmptyArrayAsArray<LEN> {
    pub const fn from_array_vec(buf: StatedChunkBuf<LEN>) -> Self {
        let buf = buf.assert();
        {
            let chunk = buf.as_str().remove_surrounding_group();
            chunk
                .prev_state()
                .copied()
                .assert_is_top_level_after_array_start();
            chunk
                .next_state()
                .assert_same(&State::INIT_AFTER_ARRAY_ITEM);
        }
        Self(buf.into_inner())
    }
    pub const fn as_str(&self) -> NonEmptyArrayAsStr<'_> {
        NonEmptyArrayAsStr(self.0.as_str())
    }
}

impl<const CAP: usize> NonEmptyArrayAsArrayVec<CAP> {
    pub const fn from_array_vec(v: StatedChunkBuf<CAP>) -> Self {
        {
            let chunk = v.as_str().remove_surrounding_group();
            chunk
                .prev_state()
                .copied()
                .assert_is_top_level_after_array_start();
            chunk
                .next_state()
                .assert_same(&State::INIT_AFTER_ARRAY_ITEM);
        }
        Self(v.into_inner())
    }
    pub const fn as_str(&self) -> NonEmptyArrayAsStr<'_> {
        NonEmptyArrayAsStr(self.0.as_str())
    }
}

impl<'a> NonEmptyArrayAsStr<'a> {
    pub const fn as_str(self) -> &'a str {
        self.0
    }
}
