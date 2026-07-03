use super::{State, stated_str::StatedChunkStr, str_as_array::StrAsArray};

// TODO: rename to StatedStrAsArray
pub struct StatedChunkString<const LEN: usize> {
    prev_state: State,
    next_state: State,
    chunk: StrAsArray<LEN>,
}

impl<const LEN: usize> StatedChunkString<LEN> {
    pub const fn as_str(&self) -> StatedChunkStr<'_> {
        StatedChunkStr::from_ref_array(self)
    }

    pub(super) const fn from_array_vec_assert_len_is_cap(
        v: super::stated_str_as_array_vec::StatedChunkBuf<LEN>,
    ) -> Self {
        let (prev_state, next_state, inner) = v.into_triple();
        Self {
            prev_state,
            next_state,
            chunk: inner.assert_len_is_cap(),
        }
    }

    pub(crate) const fn prev_state(&self) -> &State {
        &self.prev_state
    }

    pub(crate) const fn next_state(&self) -> &State {
        &self.next_state
    }

    pub(crate) const fn inner(&self) -> &StrAsArray<LEN> {
        &self.chunk
    }

    pub(crate) const fn into_inner(self) -> StrAsArray<LEN> {
        self.chunk
    }
}
