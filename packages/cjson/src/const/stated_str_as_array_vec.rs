use crate::ser::texts;

use super::{
    State, stated_str::StatedChunkStr, stated_str_as_array::StatedChunkString,
    str_as_array_vec::StrAsArrayVec,
};

// TODO: rename to StatedStrAsArrayVec
pub struct StatedChunkBuf<const CAP: usize> {
    prev_state: State,
    cur_state: State,
    buf: StrAsArrayVec<CAP>,
}

impl<const CAP: usize> StatedChunkBuf<CAP> {
    pub const fn new(prev_state: State) -> Self {
        Self {
            prev_state: prev_state.copied(),
            cur_state: prev_state,
            buf: StrAsArrayVec::DEFAULT,
        }
    }

    pub const fn left_bracket(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.left_bracket(),
            buf: self.buf.left_bracket(),
        }
    }

    pub const fn right_bracket(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.right_bracket(),
            buf: self.buf.right_bracket(),
        }
    }

    pub const fn left_brace(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.left_brace(),
            buf: self.buf.left_brace(),
        }
    }

    pub const fn right_brace(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.right_brace(),
            buf: self.buf.right_brace(),
        }
    }

    pub const fn comma(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.comma(),
            buf: self.buf.comma(),
        }
    }

    pub const fn colon(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.colon(),
            buf: self.buf.colon(),
        }
    }

    pub(crate) const fn json_value(self, value: texts::Value<&'_ str>) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.json_value(),
            buf: self.buf.json_value(value),
        }
    }

    pub const fn double_quote(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.double_quote(),
            buf: self.buf.double_quote(),
        }
    }

    /// `chunk` must be valid string fragment
    pub(crate) const fn json_string_fragments(self, chunk: &[u8]) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.json_string_fragment(),
            buf: self.buf.json_string_fragments(chunk),
        }
    }

    // TODO: remove
    pub const fn assert(self) -> StatedChunkString<CAP> {
        self.assert_len_is_cap()
    }

    pub(crate) const fn assert_len_is_cap(self) -> StatedChunkString<CAP> {
        StatedChunkString::from_array_vec_assert_len_is_cap(self)
    }

    pub(crate) const fn as_str(&self) -> StatedChunkStr<'_> {
        StatedChunkStr::from_ref_array_vec(self)
    }

    pub const fn prev_state(&self) -> &State {
        &self.prev_state
    }

    pub const fn next_state(&self) -> &State {
        &self.cur_state
    }

    pub const fn inner(&self) -> &StrAsArrayVec<CAP> {
        &self.buf
    }

    pub const fn into_inner(self) -> StrAsArrayVec<CAP> {
        self.buf
    }

    pub(super) const fn into_triple(self) -> (State, State, StrAsArrayVec<CAP>) {
        (self.prev_state, self.cur_state, self.buf)
    }
}
