use crate::r#const::HasConstCompileTimeChunk;

pub(crate) enum GroupOrComma {
    Nothing = 0,
    Group = 1,
    Comma = 2,
}

impl GroupOrComma {
    const fn try_from_u8(v: u8) -> Option<Self> {
        Some(match v {
            0 => Self::Nothing,
            1 => Self::Group,
            2 => Self::Comma,
            _ => return None,
        })
    }

    const fn as_u4(&self) -> u8 {
        match self {
            GroupOrComma::Nothing => 0,
            GroupOrComma::Group => 1,
            GroupOrComma::Comma => 2,
        }
    }

    #[must_use]
    const fn is_comma(&self) -> bool {
        matches!(self, Self::Comma)
    }
}

pub(crate) struct OpenClose {
    pub open: GroupOrComma,
    pub close: GroupOrComma,
}

impl OpenClose {
    pub const fn try_from_u8(v: u8) -> Option<Self> {
        let open = v >> 4;
        let close = v & 0xf;

        let Some(open) = GroupOrComma::try_from_u8(open) else {
            return None;
        };
        let Some(close) = GroupOrComma::try_from_u8(close) else {
            return None;
        };

        Some(Self { open, close })
    }

    pub const fn as_u8(&self) -> u8 {
        (self.open.as_u4() << 4) | (self.close.as_u4())
    }

    pub const BOTH_NOTHING: Self = Self {
        open: GroupOrComma::Nothing,
        close: GroupOrComma::Nothing,
    };

    pub const BOTH_GROUP: Self = Self {
        open: GroupOrComma::Group,
        close: GroupOrComma::Group,
    };

    pub const OPEN_GROUP: Self = Self {
        open: GroupOrComma::Group,
        close: GroupOrComma::Nothing,
    };

    pub const PREPEND_COMMA_CLOSE_GROUP: Self = Self {
        open: GroupOrComma::Comma,
        close: GroupOrComma::Group,
    };

    pub const PREPEND_COMMA: Self = Self {
        open: GroupOrComma::Comma,
        close: GroupOrComma::Nothing,
    };

    pub const APPEND_COMMA: Self = Self {
        open: GroupOrComma::Nothing,
        close: GroupOrComma::Comma,
    };
}

pub(crate) trait MakeChunks<const OPEN_CLOSE: u8> {
    const MADE_CHUNKS: MadeChunks;
}

impl<T: ?Sized + HasConstCompileTimeChunk, const OPEN_CLOSE: u8> MakeChunks<OPEN_CLOSE> for T {
    const MADE_CHUNKS: MadeChunks = {
        let OpenClose { open, close } = OpenClose::try_from_u8(OPEN_CLOSE).unwrap();
        if T::CHUNK.into_prev_state().is_init() {
            if T::CHUNK.into_next_state().is_eof() {
                // This chunk starts from init and ends with eof
                MadeChunks {
                    append_comma: open.is_comma(),
                    prepend_comma: close.is_comma(),
                    chunk: match (open, close) {
                        (GroupOrComma::Group, GroupOrComma::Group) => T::CHUNK.into_inner(),
                        (
                            GroupOrComma::Nothing | GroupOrComma::Comma,
                            GroupOrComma::Nothing | GroupOrComma::Comma,
                        ) => T::CHUNK.remove_surrounding_group().into_inner(),
                        (GroupOrComma::Nothing | GroupOrComma::Comma, GroupOrComma::Group) => {
                            T::CHUNK.remove_group_open().into_inner()
                        }
                        (GroupOrComma::Group, GroupOrComma::Nothing | GroupOrComma::Comma) => {
                            T::CHUNK.remove_group_close().into_inner()
                        }
                    },
                }
            } else {
                // only init

                let chunk;
                let prepend_comma;
                match open {
                    GroupOrComma::Nothing => {
                        chunk = T::CHUNK.remove_group_open().into_inner();
                        prepend_comma = false;
                    }
                    GroupOrComma::Group => {
                        chunk = T::CHUNK.into_inner();
                        prepend_comma = false;
                    }
                    GroupOrComma::Comma => {
                        chunk = T::CHUNK.remove_group_open().into_inner();
                        prepend_comma = true;
                    }
                }

                MadeChunks {
                    prepend_comma: prepend_comma,
                    chunk,
                    append_comma: false,
                }
            }
        } else {
            if const { T::CHUNK.into_next_state().is_eof() } {
                // only eof

                let chunk;
                let append_comma;
                match close {
                    GroupOrComma::Nothing => {
                        chunk = T::CHUNK.remove_group_close().into_inner();
                        append_comma = false;
                    }
                    GroupOrComma::Group => {
                        chunk = T::CHUNK.into_inner();
                        append_comma = false;
                    }
                    GroupOrComma::Comma => {
                        chunk = T::CHUNK.remove_group_close().into_inner();
                        append_comma = true;
                    }
                }

                MadeChunks {
                    prepend_comma: false,
                    chunk,
                    append_comma,
                }
            } else {
                // intermediate chunk
                MadeChunks {
                    prepend_comma: false,
                    chunk: T::CHUNK.into_inner(),
                    append_comma: true,
                }
            }
        }
    };
}

pub(crate) struct MadeChunks {
    pub prepend_comma: bool,
    pub chunk: &'static str,
    pub append_comma: bool,
}
