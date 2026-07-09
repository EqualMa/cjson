use super::State;

// TODO: rename to StatedStr
#[derive(Debug)]
pub struct StatedChunkStr<'a> {
    prev_state: State,
    next_state: State,
    chunk: &'a str,
}

impl<'a> StatedChunkStr<'a> {
    pub(crate) const fn copied(&self) -> Self {
        Self {
            prev_state: self.prev_state.copied(),
            next_state: self.next_state.copied(),
            chunk: self.chunk,
        }
    }

    pub(super) const fn from_ref_array<const LEN: usize>(
        s: &'a super::stated_str_as_array::StatedChunkString<LEN>,
    ) -> Self {
        Self {
            prev_state: s.prev_state().copied(),
            next_state: s.next_state().copied(),
            chunk: s.inner().as_str(),
        }
    }

    pub(super) const fn from_ref_array_vec<const CAP: usize>(
        s: &'a super::stated_str_as_array_vec::StatedChunkBuf<CAP>,
    ) -> Self {
        Self {
            prev_state: s.prev_state().copied(),
            next_state: s.next_state().copied(),
            chunk: s.inner().as_str(),
        }
    }

    pub const fn into_next_state(self) -> State {
        self.next_state
    }
    pub(crate) const fn next_state(&self) -> &State {
        &self.next_state
    }

    pub(crate) const fn into_prev_state(self) -> State {
        self.prev_state
    }
    pub(crate) const fn prev_state(&self) -> &State {
        &self.prev_state
    }

    pub(crate) const fn into_inner(self) -> &'a str {
        self.chunk
    }
    pub(crate) const fn inner(&self) -> &'a str {
        self.chunk
    }

    pub(crate) const fn remove_surrounding_group(self) -> Self {
        self.prev_state().assert_same(&State::INIT);
        self.next_state().assert_eof();

        match self.inner().as_bytes() {
            [b'[', inner @ .., b']'] => Self {
                prev_state: State::INIT_AFTER_ARRAY_START,
                next_state: if inner.is_empty() {
                    State::INIT_AFTER_ARRAY_START
                } else {
                    State::INIT_AFTER_ARRAY_ITEM
                },
                chunk: unsafe { str::from_utf8_unchecked(inner) },
            },
            [b'{', inner @ .., b'}'] => Self {
                prev_state: State::INIT_AFTER_OBJECT_START,
                next_state: if inner.is_empty() {
                    State::INIT_AFTER_OBJECT_START
                } else {
                    State::INIT_AFTER_OBJECT_FIELD_VALUE
                },
                chunk: unsafe { str::from_utf8_unchecked(inner) },
            },
            [b'"', inner @ .., b'"'] => Self {
                prev_state: State::INIT_IN_STRING,
                next_state: State::INIT_IN_STRING,
                chunk: unsafe { str::from_utf8_unchecked(inner) },
            },
            _ => panic!("no valid surrounding group"),
        }
    }

    pub(crate) const fn remove_group_open(self) -> Self {
        self.prev_state().assert_same(&State::INIT);

        match self.chunk.as_bytes() {
            [b'[', rest @ ..] => Self {
                prev_state: State::INIT_AFTER_ARRAY_START,
                next_state: self.next_state,
                chunk: unsafe { str::from_utf8_unchecked(rest) },
            },
            [b'{', rest @ ..] => Self {
                prev_state: State::INIT_AFTER_OBJECT_START,
                next_state: self.next_state,
                chunk: unsafe { str::from_utf8_unchecked(rest) },
            },
            [b'"', rest @ ..] => Self {
                prev_state: State::INIT_IN_STRING,
                next_state: self.next_state,
                chunk: unsafe { str::from_utf8_unchecked(rest) },
            },
            _ => panic!("no valid group open"),
        }
    }

    pub(crate) const fn remove_group_close(self) -> Self {
        self.next_state().assert_eof();
        match self.chunk.as_bytes() {
            [head @ .., b']'] => Self {
                prev_state: self.prev_state.copied(),
                next_state: if head.is_empty() {
                    self.prev_state
                } else {
                    if self.prev_state().is_init() {
                        match head {
                            [b'['] => State::INIT_AFTER_ARRAY_START,
                            [b'[', ..] => State::INIT_AFTER_ARRAY_ITEM,
                            _ => panic!(),
                        }
                    } else {
                        State::INIT_AFTER_ARRAY_ITEM
                    }
                },
                chunk: unsafe { str::from_utf8_unchecked(head) },
            },
            [head @ .., b'}'] => Self {
                prev_state: self.prev_state.copied(),
                next_state: if head.is_empty() {
                    self.prev_state
                } else {
                    if self.prev_state.is_init() {
                        match head {
                            [b'{'] => State::INIT_AFTER_OBJECT_START,
                            [b'{', ..] => State::INIT_AFTER_OBJECT_FIELD_VALUE,
                            _ => panic!(),
                        }
                    } else {
                        State::INIT_AFTER_OBJECT_FIELD_VALUE
                    }
                },
                chunk: unsafe { str::from_utf8_unchecked(head) },
            },
            [head @ .., b'"'] => {
                const IN_STRING: State = State::INIT.double_quote();
                Self {
                    prev_state: self.prev_state.copied(),
                    next_state: IN_STRING,
                    chunk: unsafe { str::from_utf8_unchecked(head) },
                }
            }
            _ => panic!("no valid group close"),
        }
    }
}
