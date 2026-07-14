use core::{fmt, marker::PhantomData};

use super::{
    HasConstJsonValue, StatedChunkStr, array::NonEmptyArray, object::NonEmptyObject,
    string::JsonString, value::Value,
};

use self::IntermediateState::*;

#[derive(PartialEq, Eq)]
pub struct State(StateInner);

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! end_array_or_object {
    ($stack:ident . $end_group:ident (), $kind:expr $(,)?) => {
        match $stack.$end_group() {
            AfterEndArrayOrObject::Intermediate(intermediate) => {
                StateInner::Intermediate(intermediate)
            }
            AfterEndArrayOrObject::Eof => StateInner::Eof($kind),
        }
    };
}

impl State {
    #[cfg(remove)]
    pub const fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }

    #[cfg(remove)]
    pub(crate) const fn try_from_u128(v: u128) -> Option<Self> {
        crate::utils::option_map!(StateInner::try_from_u128(v), Self)
    }

    pub(crate) const fn assert_same(&self, other: &Self) {
        self.0.assert_same(&other.0)
    }

    pub(crate) const fn assert_init(&self) {
        match self.0 {
            StateInner::Init => {}
            _ => panic!("expect state to be Init"),
        }
    }

    pub(crate) const fn assert_eof(&self) {
        match self.0 {
            StateInner::Eof(..) => {}
            _ => panic!("expect state to be Eof"),
        }
    }

    pub(crate) const fn assert_eof_of_string(&self) {
        match self.0 {
            StateInner::Eof(kind) => match kind {
                ValueKind::String => {}
                _ => panic!("expect state to be Eof of json string"),
            },
            _ => panic!("expect state to be Eof"),
        }
    }

    pub(crate) const fn assert_eof_of_non_empty_array(self) {
        match self.0 {
            StateInner::Eof(kind) => match kind {
                ValueKind::ArrayWithNonEmptyContent => {}
                _ => panic!("expect state to be Eof of non empty array"),
            },
            _ => panic!("expect state to be Eof"),
        }
    }

    pub(crate) const fn assert_eof_of_non_empty_object(self) {
        match self.0 {
            StateInner::Eof(kind) => match kind {
                ValueKind::ObjectWithNonEmptyContent => {}
                _ => panic!("expect state to be Eof of non empty object"),
            },
            _ => panic!("expect state to be Eof"),
        }
    }

    pub(crate) const fn is_init(&self) -> bool {
        matches!(self, Self(StateInner::Init))
    }

    pub(crate) const fn is_eof(&self) -> bool {
        matches!(self, Self(StateInner::Eof(..)))
    }

    pub const INIT: Self = Self(StateInner::Init);
    pub(crate) const INIT_AFTER_ARRAY_START: Self = Self::INIT.left_bracket();
    pub(crate) const INIT_AFTER_ARRAY_ITEM: Self = Self::INIT_AFTER_ARRAY_START.json_value();
    pub(crate) const INIT_AFTER_OBJECT_START: Self = Self::INIT.left_brace();
    pub(crate) const INIT_AFTER_OBJECT_FIELD_VALUE: Self = Self::INIT_AFTER_OBJECT_START
        .double_quote()
        .double_quote()
        .colon()
        .json_value();
    pub(crate) const INIT_IN_STRING: Self = Self::INIT.double_quote();

    pub(crate) const fn assert_is_top_level_after_array_start(self) {
        self.assert_same(&Self::INIT_AFTER_ARRAY_START);
    }
    pub(crate) const fn assert_is_before_top_level_right_bracket(self) {
        self.right_bracket().assert_eof();
    }

    pub(crate) const fn assert_is_top_level_after_object_start(self) {
        self.assert_same(&Self::INIT_AFTER_OBJECT_START);
    }
    pub(crate) const fn assert_is_before_top_level_right_brace(self) {
        self.right_brace().assert_eof();
    }

    pub(crate) const fn assert_is_top_level_in_string(&self) {
        self.assert_same(&Self::INIT_IN_STRING);
    }

    pub const fn json_value(self) -> Self {
        Self(match self.0 {
            StateInner::Init => StateInner::Eof(ValueKind::Unknown),
            StateInner::Intermediate(Intermediate { stack, state }) => {
                let new_state = match state {
                    InString => panic!(),
                    AfterArrayStart | AfterArrayComma | AfterArrayStartOrComma => AfterArrayItem,
                    AfterArrayItem | AfterArrayStartOrItem => panic!(),
                    AfterObjectStart | AfterObjectComma | AfterObjectStartOrComma => panic!(),
                    InObjectFieldName => panic!(),
                    AfterObjectFieldName => panic!(),
                    AfterObjectFieldColon => AfterObjectFieldValue,
                    AfterObjectFieldValue => panic!(),
                    AfterObjectStartOrFieldValue => panic!(),
                };
                StateInner::Intermediate(Intermediate {
                    stack,
                    state: new_state,
                })
            }
            StateInner::Eof(..) => panic!(),
        })
    }

    /// Single `"`
    pub const fn double_quote(self) -> Self {
        Self(match self.0 {
            StateInner::Init => StateInner::Intermediate(Intermediate {
                stack: Stack::INIT,
                state: InString,
            }),
            StateInner::Intermediate(Intermediate { stack, state }) => {
                StateInner::Intermediate(Intermediate {
                    state: match state {
                        InString => match stack.is_in_array_or_object() {
                            Some(true) => AfterArrayItem,
                            Some(false) => AfterObjectFieldValue,
                            None => return Self(StateInner::Eof(ValueKind::String)),
                        },
                        AfterArrayStart | AfterArrayComma | AfterArrayStartOrComma => InString,
                        AfterArrayItem => panic!(),
                        AfterObjectStart | AfterObjectComma | AfterObjectStartOrComma => {
                            InObjectFieldName
                        }
                        InObjectFieldName => AfterObjectFieldName,
                        AfterObjectFieldName => panic!(),
                        AfterObjectFieldColon => InString,
                        AfterObjectFieldValue => panic!(),
                        AfterArrayStartOrItem => panic!(),
                        AfterObjectStartOrFieldValue => panic!(),
                    },
                    stack,
                })
            }
            StateInner::Eof(..) => panic!(),
        })
    }

    pub const fn json_string_fragment(self) -> Self {
        Self(match self.0 {
            StateInner::Init => {
                panic!()
            }
            StateInner::Intermediate(Intermediate { stack, state }) => {
                StateInner::Intermediate(Intermediate {
                    stack,
                    state: match state {
                        InString => InString,
                        InObjectFieldName => InObjectFieldName,
                        _ => panic!(),
                    },
                })
            }
            StateInner::Eof(..) => {
                panic!()
            }
        })
    }

    pub const fn comma(self) -> Self {
        Self(match self.0 {
            StateInner::Init => {
                panic!()
            }
            StateInner::Intermediate(Intermediate { stack, state }) => {
                StateInner::Intermediate(Intermediate {
                    stack,
                    state: match state {
                        InString => panic!(),
                        AfterArrayStart | AfterArrayComma | AfterArrayStartOrComma => panic!(),
                        AfterArrayStartOrItem => panic!(),
                        AfterArrayItem => AfterArrayComma,
                        AfterObjectStart | AfterObjectComma => panic!(),
                        InObjectFieldName => panic!(),
                        AfterObjectFieldName => panic!(),
                        AfterObjectFieldColon => panic!(),
                        AfterObjectFieldValue => AfterObjectComma,
                        AfterObjectStartOrComma => panic!(),
                        AfterObjectStartOrFieldValue => panic!(),
                    },
                })
            }
            StateInner::Eof(..) => {
                panic!()
            }
        })
    }

    pub const fn colon(self) -> Self {
        Self(match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                AfterObjectFieldName => StateInner::Intermediate(Intermediate {
                    stack,
                    state: AfterObjectFieldColon,
                }),
                _ => panic!(),
            },
            StateInner::Eof(..) => panic!(),
        })
    }

    pub const fn left_bracket(self) -> Self {
        Self(match self.0 {
            StateInner::Init => StateInner::Intermediate(Intermediate {
                stack: Stack::INIT.start_array(),
                state: AfterArrayStart,
            }),
            StateInner::Intermediate(Intermediate { stack, state }) => {
                state.assert_expecting_value();
                StateInner::Intermediate(Intermediate {
                    stack: stack.start_array(),
                    state: AfterArrayStart,
                })
            }
            StateInner::Eof(..) => panic!(),
        })
    }

    pub const fn right_bracket(self) -> Self {
        Self(match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                InString => panic!(),
                AfterArrayStart => {
                    end_array_or_object!(stack.end_array(), ValueKind::ArrayWithEmptyContent)
                }
                AfterArrayItem => {
                    end_array_or_object!(stack.end_array(), ValueKind::ArrayWithNonEmptyContent)
                }
                AfterArrayStartOrItem => {
                    end_array_or_object!(stack.end_array(), ValueKind::ArrayWithUnknownContent)
                }
                AfterArrayComma => panic!(),
                AfterArrayStartOrComma => panic!(),
                AfterObjectStart | AfterObjectComma => panic!(),
                InObjectFieldName => panic!(),
                AfterObjectFieldName => panic!(),
                AfterObjectFieldColon => panic!(),
                AfterObjectFieldValue => panic!(),
                AfterObjectStartOrComma => panic!(),
                AfterObjectStartOrFieldValue => panic!(),
            },
            StateInner::Eof(..) => panic!(),
        })
    }

    pub const fn left_brace(self) -> Self {
        Self(match self.0 {
            StateInner::Init => StateInner::Intermediate(Intermediate {
                stack: Stack::INIT.start_object(),
                state: AfterObjectStart,
            }),
            StateInner::Intermediate(Intermediate { stack, state }) => {
                state.assert_expecting_value();
                StateInner::Intermediate(Intermediate {
                    stack: stack.start_object(),
                    state: AfterObjectStart,
                })
            }
            StateInner::Eof(..) => panic!(),
        })
    }

    pub const fn right_brace(self) -> Self {
        Self(match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                AfterObjectStart => {
                    end_array_or_object!(stack.end_object(), ValueKind::ObjectWithEmptyContent)
                }
                AfterObjectFieldValue => {
                    end_array_or_object!(stack.end_object(), ValueKind::ObjectWithNonEmptyContent)
                }
                AfterObjectStartOrFieldValue => {
                    end_array_or_object!(stack.end_object(), ValueKind::ObjectWithUnknownContent)
                }
                InString => panic!(),
                AfterArrayStart => panic!(),
                AfterArrayItem => panic!(),
                AfterArrayComma => panic!(),
                AfterArrayStartOrComma => panic!(),
                AfterArrayStartOrItem => panic!(),
                InObjectFieldName => panic!(),
                AfterObjectFieldName => panic!(),
                AfterObjectFieldColon => panic!(),
                AfterObjectComma => panic!(),
                AfterObjectStartOrComma => panic!(),
            },
            StateInner::Eof(..) => panic!(),
        })
    }

    pub const fn json_items_after_item(self) -> State {
        match &self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack: _, state }) => match state {
                AfterArrayItem => self,
                InString => panic!(),
                AfterArrayStart => panic!(),
                AfterArrayComma => panic!(),
                AfterArrayStartOrComma => panic!(),
                AfterArrayStartOrItem => panic!(),
                AfterObjectStart => panic!(),
                InObjectFieldName => panic!(),
                AfterObjectFieldName => panic!(),
                AfterObjectFieldColon => panic!(),
                AfterObjectFieldValue => panic!(),
                AfterObjectComma => panic!(),
                AfterObjectStartOrComma => panic!(),
                AfterObjectStartOrFieldValue => panic!(),
            },
            StateInner::Eof(..) => panic!(),
        }
    }

    pub const fn json_items_after_array_start_before_item(self) -> State {
        match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                AfterArrayStart | AfterArrayStartOrComma => {
                    Self(StateInner::Intermediate(Intermediate {
                        stack,
                        state: AfterArrayStartOrComma,
                    }))
                }
                InString => panic!(),
                AfterArrayItem => panic!(),
                AfterArrayComma => panic!(),
                AfterArrayStartOrItem => panic!(),
                AfterObjectStart => panic!(),
                InObjectFieldName => panic!(),
                AfterObjectFieldName => panic!(),
                AfterObjectFieldColon => panic!(),
                AfterObjectFieldValue => panic!(),
                AfterObjectComma => panic!(),
                AfterObjectStartOrComma => panic!(),
                AfterObjectStartOrFieldValue => panic!(),
            },
            StateInner::Eof(..) => panic!(),
        }
    }

    pub const fn json_kvs_after_object_start_before_kv(self) -> State {
        match self.0 {
            StateInner::Init | StateInner::Eof(..) => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                AfterObjectStart => Self(StateInner::Intermediate(Intermediate {
                    stack,
                    state: AfterObjectStartOrComma,
                })),
                InString
                | AfterArrayStart
                | AfterArrayItem
                | AfterArrayComma
                | AfterArrayStartOrComma
                | AfterArrayStartOrItem
                | InObjectFieldName
                | AfterObjectFieldName
                | AfterObjectFieldColon
                | AfterObjectFieldValue
                | AfterObjectComma
                | AfterObjectStartOrComma
                | AfterObjectStartOrFieldValue => panic!(),
            },
        }
    }

    pub const fn json_kvs_after_field_value(self) -> State {
        match &self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack: _, state }) => match state {
                AfterObjectFieldValue => self,
                InString
                | AfterArrayStart
                | AfterArrayStartOrComma
                | AfterArrayStartOrItem
                | AfterArrayItem
                | AfterArrayComma
                | AfterObjectStart
                | AfterObjectStartOrComma
                | AfterObjectStartOrFieldValue
                | InObjectFieldName
                | AfterObjectFieldName
                | AfterObjectFieldColon
                | AfterObjectComma => panic!(),
            },
            StateInner::Eof(..) => panic!(),
        }
    }

    pub const fn json_items_between_brackets(self) -> State {
        match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                AfterArrayStart => Self(StateInner::Intermediate(Intermediate {
                    stack,
                    state: AfterArrayStartOrItem,
                })),
                InString => panic!(),
                AfterArrayItem => panic!(),
                AfterArrayComma => panic!(),
                AfterArrayStartOrComma => panic!(),
                AfterArrayStartOrItem => panic!(),
                AfterObjectStart => panic!(),
                InObjectFieldName => panic!(),
                AfterObjectFieldName => panic!(),
                AfterObjectFieldColon => panic!(),
                AfterObjectFieldValue => panic!(),
                AfterObjectComma => panic!(),
                AfterObjectStartOrComma => panic!(),
                AfterObjectStartOrFieldValue => panic!(),
            },
            StateInner::Eof(..) => panic!(),
        }
    }

    pub(crate) const fn copied(&self) -> Self {
        Self(match &self.0 {
            StateInner::Init => StateInner::Init,
            StateInner::Intermediate(intermediate) => {
                StateInner::Intermediate(intermediate.copied())
            }
            StateInner::Eof(kind) => StateInner::Eof(*kind),
        })
    }

    pub(crate) const fn remove_group_open(init: State, next: State) -> u8 {
        init.assert_init();

        let kind = match next.0 {
            StateInner::Init => panic!("cannot remove group because chunk is empty"),
            StateInner::Intermediate(intermediate) => intermediate.estimated_top_level_value_kind(),
            StateInner::Eof(kind) => kind,
        };

        match kind {
            ValueKind::ArrayWithUnknownContent
            | ValueKind::ArrayWithEmptyContent
            | ValueKind::ArrayWithNonEmptyContent => b'[',
            ValueKind::ObjectWithUnknownContent
            | ValueKind::ObjectWithEmptyContent
            | ValueKind::ObjectWithNonEmptyContent => b'{',
            ValueKind::String => b'"',
            ValueKind::Unknown => panic!("chunk is not known to start as a group"),
        }
    }

    pub(crate) const fn remove_group_close(prev: State, eof: State) -> u8 {
        let State(StateInner::Eof(kind)) = eof else {
            panic!("expect next state to be Eof")
        };

        let estimated_kind = match prev.0 {
            StateInner::Init => None,
            StateInner::Intermediate(intermediate) => {
                Some(intermediate.estimated_top_level_value_kind())
            }
            StateInner::Eof(_) => panic!("chunk is expected not to be empty"),
        };

        match kind {
            ValueKind::ArrayWithUnknownContent => {
                if let Some(estimated_kind) = estimated_kind {
                    assert!(matches!(
                        estimated_kind,
                        ValueKind::ArrayWithEmptyContent | ValueKind::ArrayWithUnknownContent
                    ));
                }
                b']'
            }
            ValueKind::ArrayWithEmptyContent => {
                if let Some(estimated_kind) = estimated_kind {
                    assert!(matches!(estimated_kind, ValueKind::ArrayWithEmptyContent));
                }
                b']'
            }
            ValueKind::ArrayWithNonEmptyContent => {
                if let Some(estimated_kind) = estimated_kind {
                    assert!(matches!(
                        estimated_kind,
                        ValueKind::ArrayWithEmptyContent
                            | ValueKind::ArrayWithNonEmptyContent
                            | ValueKind::ArrayWithUnknownContent
                    ));
                }
                b']'
            }
            ValueKind::ObjectWithUnknownContent => {
                if let Some(estimated_kind) = estimated_kind {
                    assert!(matches!(
                        estimated_kind,
                        ValueKind::ObjectWithEmptyContent | ValueKind::ObjectWithUnknownContent
                    ));
                }
                b'}'
            }
            ValueKind::ObjectWithEmptyContent => {
                if let Some(estimated_kind) = estimated_kind {
                    assert!(matches!(estimated_kind, ValueKind::ObjectWithEmptyContent));
                }
                b'}'
            }
            ValueKind::ObjectWithNonEmptyContent => {
                if let Some(estimated_kind) = estimated_kind {
                    assert!(matches!(
                        estimated_kind,
                        ValueKind::ObjectWithEmptyContent
                            | ValueKind::ObjectWithNonEmptyContent
                            | ValueKind::ObjectWithUnknownContent
                    ));
                }
                b'}'
            }
            ValueKind::String => {
                if let Some(estimated_kind) = estimated_kind {
                    assert!(matches!(estimated_kind, ValueKind::String));
                }
                b'"'
            }
            ValueKind::Unknown => panic!("chunk is not known to end as a group"),
        }
    }

    pub(crate) const fn remove_surrounding_group(init: State, eof: State) -> (u8, u8, bool) {
        init.assert_init();
        let State(StateInner::Eof(kind)) = eof else {
            panic!("expect next state to be Eof")
        };
        match kind {
            ValueKind::ArrayWithEmptyContent => (b'[', b']', true),
            ValueKind::ArrayWithUnknownContent | ValueKind::ArrayWithNonEmptyContent => {
                (b'[', b']', false)
            }
            ValueKind::ObjectWithEmptyContent => (b'{', b'}', true),
            ValueKind::ObjectWithUnknownContent | ValueKind::ObjectWithNonEmptyContent => {
                (b'{', b'}', false)
            }
            ValueKind::String => (b'"', b'"', false),
            ValueKind::Unknown => panic!("chunk is not known to be a group"),
        }
    }

    pub(crate) const fn assert_is_contentful_first_chunk_of_array(self) {
        let next = self;
        match next.0 {
            StateInner::Init | StateInner::Eof(..) => panic!("expect first chunk"),
            StateInner::Intermediate(intermediate) => {
                match intermediate.estimated_top_level_value_kind() {
                    ValueKind::ArrayWithNonEmptyContent => {
                        // ok
                    }
                    ValueKind::ArrayWithEmptyContent | ValueKind::ArrayWithUnknownContent => {
                        panic!("expect first chunk to be what a non-empty array would start with")
                    }
                    ValueKind::ObjectWithUnknownContent
                    | ValueKind::ObjectWithEmptyContent
                    | ValueKind::ObjectWithNonEmptyContent
                    | ValueKind::String
                    | ValueKind::Unknown => {
                        panic!("expect first chunk to be what an array would start with")
                    }
                }
            }
        }
    }

    pub(crate) const fn assert_is_contentful_first_chunk_of_object(self) {
        let next = self;
        match next.0 {
            StateInner::Init | StateInner::Eof(..) => panic!("expect first chunk"),
            StateInner::Intermediate(intermediate) => {
                match intermediate.estimated_top_level_value_kind() {
                    ValueKind::ObjectWithNonEmptyContent => {
                        // ok
                    }
                    ValueKind::ObjectWithEmptyContent | ValueKind::ObjectWithUnknownContent => {
                        panic!("expect first chunk to be what a non-empty object would start with")
                    }
                    ValueKind::ArrayWithUnknownContent
                    | ValueKind::ArrayWithEmptyContent
                    | ValueKind::ArrayWithNonEmptyContent
                    | ValueKind::String
                    | ValueKind::Unknown => {
                        panic!("expect first chunk to be what an object would start with")
                    }
                }
            }
        }
    }

    pub(crate) const fn assert_is_contentful_last_chunk_of_array(self) {
        let prev = self;
        match prev.0 {
            StateInner::Init | StateInner::Eof(..) => panic!("expect last chunk"),
            StateInner::Intermediate(intermediate) => {
                match intermediate.estimated_top_level_value_kind() {
                    ValueKind::ArrayWithNonEmptyContent | ValueKind::ArrayWithUnknownContent => {
                        // ok
                    }
                    ValueKind::ArrayWithEmptyContent => {
                        panic!("expect last chunk to be what a non-empty array would end with")
                    }
                    ValueKind::ObjectWithUnknownContent
                    | ValueKind::ObjectWithEmptyContent
                    | ValueKind::ObjectWithNonEmptyContent
                    | ValueKind::String
                    | ValueKind::Unknown => {
                        panic!("expect last chunk to be what a non-empty array would end with")
                    }
                }
            }
        }
    }

    pub(crate) const fn assert_is_contentful_last_chunk_of_object(self) {
        let prev = self;
        match prev.0 {
            StateInner::Init | StateInner::Eof(..) => panic!("expect last chunk"),
            StateInner::Intermediate(intermediate) => {
                match intermediate.estimated_top_level_value_kind() {
                    ValueKind::ObjectWithNonEmptyContent | ValueKind::ObjectWithUnknownContent => {
                        // ok
                    }
                    ValueKind::ObjectWithEmptyContent => {
                        panic!("expect last chunk to be what a non-empty object would end with")
                    }
                    ValueKind::ArrayWithUnknownContent
                    | ValueKind::ArrayWithEmptyContent
                    | ValueKind::ArrayWithNonEmptyContent
                    | ValueKind::String
                    | ValueKind::Unknown => {
                        panic!("expect last chunk to be what a non-empty object would end with")
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Intermediate {
    stack: Stack,
    state: IntermediateState,
}

impl Intermediate {
    const fn copied(&self) -> Self {
        Self {
            stack: self.stack.copied(),
            state: self.state.copied(),
        }
    }

    const fn estimated_top_level_value_kind(&self) -> ValueKind {
        match self.stack.len {
            0 => match self.state {
                InString => ValueKind::String,
                AfterArrayStart
                | AfterArrayStartOrComma
                | AfterArrayStartOrItem
                | AfterArrayItem
                | AfterArrayComma
                | AfterObjectStart
                | AfterObjectStartOrComma
                | AfterObjectStartOrFieldValue
                | InObjectFieldName
                | AfterObjectFieldName
                | AfterObjectFieldColon
                | AfterObjectFieldValue
                | AfterObjectComma => {
                    panic!("unexpected state: stack.len is 0 but state is not in string")
                }
            },
            1 => {
                if (self.stack.inner & 1) == 1 {
                    // array
                    match self.state {
                        InString | AfterArrayItem | AfterArrayComma => {
                            ValueKind::ArrayWithNonEmptyContent
                        }
                        AfterArrayStart => ValueKind::ArrayWithEmptyContent,
                        AfterArrayStartOrComma | AfterArrayStartOrItem => {
                            ValueKind::ArrayWithUnknownContent
                        }
                        AfterObjectStart
                        | AfterObjectStartOrComma
                        | AfterObjectStartOrFieldValue
                        | InObjectFieldName
                        | AfterObjectFieldName
                        | AfterObjectFieldColon
                        | AfterObjectFieldValue
                        | AfterObjectComma => {
                            panic!("unexpected state: expect state in top level array")
                        }
                    }
                } else {
                    // object
                    match self.state {
                        InString
                        | InObjectFieldName
                        | AfterObjectFieldName
                        | AfterObjectFieldColon
                        | AfterObjectFieldValue
                        | AfterObjectComma => ValueKind::ObjectWithNonEmptyContent,
                        AfterObjectStart => ValueKind::ObjectWithEmptyContent,
                        AfterObjectStartOrComma | AfterObjectStartOrFieldValue => {
                            ValueKind::ObjectWithUnknownContent
                        }
                        AfterArrayStart
                        | AfterArrayStartOrComma
                        | AfterArrayStartOrItem
                        | AfterArrayItem
                        | AfterArrayComma => {
                            panic!("unexpected state: expect state in top level object")
                        }
                    }
                }
            }
            len => {
                if ((self.stack.inner >> (len - 1)) & 1) == 1 {
                    // array
                    ValueKind::ArrayWithNonEmptyContent
                } else {
                    // object
                    ValueKind::ObjectWithNonEmptyContent
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum StateInner {
    Init,
    Intermediate(Intermediate),
    Eof(ValueKind),
}

macro_rules! define_value_kind {
    (
        $(#$attr:tt)*
        $vis:vis enum $ValueKind:ident {
            $($Var:ident),+ $(,)?
        }

        #[assert_same]
        fn $assert_same:ident();
    ) => {
        $(#$attr)*
        $vis enum $ValueKind {
            $($Var),+
        }

        impl $ValueKind {
            const fn $assert_same(&self, other_state: &Self) {
                match (self, other_state) {
                    $((Self::$Var, Self::$Var) => {})+
                    _ => {
                        panic!("state mismatch")
                    }
                }
            }
        }
    };
}

define_value_kind!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ValueKind {
        // The content is not known to be empty or non-empty.
        ArrayWithUnknownContent,
        ArrayWithEmptyContent,
        ArrayWithNonEmptyContent,
        ObjectWithUnknownContent,
        ObjectWithEmptyContent,
        ObjectWithNonEmptyContent,
        String,
        Unknown,
    }

    #[assert_same]
    fn assert_same();
);

impl StateInner {
    const fn as_u128(&self) -> u128 {
        match self {
            StateInner::Init => 0,
            // 10XXXXXX XXXXXXXX XXXXXXXX XXXXXXXX
            // XXXXXXXX XXXXXXXX state    stack_2
            // stack_1 (8 bytes)
            StateInner::Intermediate(Intermediate { stack, state }) => {
                let (stack_1, stack_2) = stack.as_u64_u8();
                let state = state.copied().into_u8();

                let mut bytes = [0u8; 16];

                {
                    #[rustfmt::skip]
                    let [
                        tag_mut, _, _, _,
                        _, _, state_mut, stack_2_mut,
                        stack_1_mut @ ..
                    ] = &mut bytes;

                    *tag_mut = Self::INTERMEDIATE_TAG;
                    *state_mut = state;
                    *stack_2_mut = stack_2;
                    *stack_1_mut = stack_1.to_le_bytes();
                }

                u128::from_le_bytes(bytes)
            }
            StateInner::Eof(..) => Self::EOF_AS_U128,
        }
    }

    const INTERMEDIATE_TAG: u8 = 0b10_000_000;
    const EOF_AS_U128: u128 = !0;

    #[cfg(remove)]
    const fn try_from_u128(v: u128) -> Option<Self> {
        Some(match v {
            0 => Self::Init,
            Self::EOF_AS_U128 => Self::Eof,
            v => {
                #[rustfmt::skip]
                let [
                    Self::INTERMEDIATE_TAG, 0, 0, 0,
                    0, 0, state, stack_2,
                    stack_1 @ ..
                ] = v.to_le_bytes() else {
                    return None;
                };

                let Some(state) = IntermediateState::try_from_u8(state) else {
                    return None;
                };
                let Some(stack) = Stack::try_from_u64_u8(u64::from_le_bytes(stack_1), stack_2)
                else {
                    return None;
                };

                Self::Intermediate(Intermediate { stack, state })
            }
        })
    }

    const fn assert_same(&self, other: &Self) {
        match (self, other) {
            (StateInner::Init, StateInner::Init) => {}
            (
                StateInner::Intermediate(Intermediate { stack, state }),
                StateInner::Intermediate(Intermediate {
                    stack: other_stack,
                    state: other_state,
                }),
            ) => {
                stack.assert_same(&other_stack);
                state.assert_same(&other_state);
            }
            (StateInner::Eof(this_kind), StateInner::Eof(other_kind)) => {
                this_kind.assert_same(other_kind)
            }
            _ => panic!("State mismatch"),
        }
    }
}

type StackInner = u64;

#[derive(PartialEq, Eq)]
struct Stack {
    // bit 1 means in array
    // bit 0 means in object
    inner: StackInner,
    len: usize,
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut this = self.copied();

        while let Some(array_or_object) = this.pop() {
            if array_or_object {
                write!(f, "[")?;
            } else {
                write!(f, "{{")?;
            }
        }

        Ok(())
    }
}

impl Stack {
    const fn as_u64_u8(&self) -> (u64, u8) {
        (self.inner, {
            assert!(self.len <= u8::MAX as usize);
            self.len as u8
        })
    }

    const fn try_from_u64_u8(inner: u64, len: u8) -> Option<Self> {
        if (len as u32) > u64::BITS {
            return None;
        }

        if (inner >> (len as usize)) != 0 {
            return None;
        }

        Some(Self {
            inner,
            len: len as usize,
        })
    }

    const INIT: Self = Self { inner: 0, len: 0 };

    const fn is_in_array_or_object(&self) -> Option<bool> {
        if self.len == 0 {
            None
        } else {
            Some((self.inner & 1) != 0)
        }
    }

    const fn is_in_top_level_array(&self) -> bool {
        self.len == 1 && ((self.inner & 1) == 1)
    }

    const fn is_in_top_level_object(&self) -> bool {
        self.len == 1 && ((self.inner & 1) == 0)
    }

    const fn pop(&mut self) -> Option<bool> {
        if self.len == 0 {
            None
        } else {
            Some({
                self.len -= 1;
                let last = self.inner & 1;
                self.inner >>= 1;
                last != 0
            })
        }
    }

    const fn start_array(mut self) -> Stack {
        assert!(
            self.len < (StackInner::BITS as usize),
            "too many nested array or object"
        );
        self.inner <<= 1;
        self.inner |= 1;
        self.len += 1;

        self
    }

    const fn start_object(mut self) -> Stack {
        assert!(
            self.len < (StackInner::BITS as usize),
            "too many nested array or object"
        );
        self.inner <<= 1;
        self.inner &= !1;
        self.len += 1;

        self
    }

    const fn end_array(mut self) -> AfterEndArrayOrObject {
        let popped = self.pop();
        assert!(popped.expect("in array"), "in array not in object");

        self.current_state_after_array_or_object()
    }

    const fn end_object(mut self) -> AfterEndArrayOrObject {
        let popped = self.pop();
        assert!(!popped.expect("in object"), "in object not in array");

        self.current_state_after_array_or_object()
    }

    const fn current_state_after_array_or_object(self) -> AfterEndArrayOrObject {
        match self.is_in_array_or_object() {
            Some(true) => {
                // after value in array
                AfterEndArrayOrObject::Intermediate(Intermediate {
                    stack: self,
                    state: AfterArrayItem,
                })
            }
            Some(false) => {
                // after value in object
                AfterEndArrayOrObject::Intermediate(Intermediate {
                    stack: self,
                    state: AfterObjectFieldValue,
                })
            }
            None => AfterEndArrayOrObject::Eof,
        }
    }

    const fn assert_same(&self, other: &Stack) {
        if self.len == other.len && self.inner == other.inner {
            return;
        }

        panic!("state stack mismatch")
    }

    const fn copied(&self) -> Self {
        Self {
            inner: self.inner,
            len: self.len,
        }
    }
}

enum AfterEndArrayOrObject {
    Intermediate(Intermediate),
    Eof,
}

macro_rules! define_inter_state {
    (
        $(#$attr:tt)*
        $vis:vis enum $IntermediateState:ident {
            $($Var:ident = $discriminant:expr),+ $(,)?
        }

        #[assert_same]
        fn $assert_same:ident();

        #[copied]
        fn $copied:ident();

        #[try_from_u8]
        fn $try_from_u8:ident();

        #[into_u8]
        fn $into_u8:ident();
    ) => {
        $(#$attr)*
        $vis enum $IntermediateState {
            $($Var,)+
        }

        impl $IntermediateState {
            const fn $assert_same(&self, other_state: &Self) {
                match (self, other_state) {
                    $((Self::$Var, Self::$Var) => {})+
                    _ => {
                        panic!("state mismatch")
                    }
                }
            }

            const fn $copied(&self) -> Self {
                match self {
                    $(Self::$Var => Self::$Var,)+
                }
            }

            const fn $into_u8(self) -> u8 {
                match self {
                    $(Self::$Var => $discriminant,)+
                }
            }

            const fn $try_from_u8(v: u8) -> Option<Self> {
                match v {
                    $($discriminant => Some(Self::$Var),)+
                    _ => None,
                }
            }
        }
    };
}

define_inter_state!(
    #[derive(Debug, PartialEq, Eq)]
    enum IntermediateState {
        // Note the discriminants are not stable across versions
        InString = 0,
        AfterArrayStart = 1,
        AfterArrayStartOrComma = 2,
        AfterArrayStartOrItem = 3,
        AfterArrayItem = 4,
        AfterArrayComma = 5,
        AfterObjectStart = 6,
        AfterObjectStartOrComma = 7,
        AfterObjectStartOrFieldValue = 8,
        InObjectFieldName = 9,
        AfterObjectFieldName = 10,
        AfterObjectFieldColon = 11,
        AfterObjectFieldValue = 12,
        AfterObjectComma = 13,
    }

    #[assert_same]
    fn assert_same();

    #[copied]
    fn copied();

    #[try_from_u8]
    fn try_from_u8();
    #[into_u8]
    fn into_u8();
);

impl IntermediateState {
    /// Assert the state is expecting
    /// json value except object field name
    const fn assert_expecting_value(&self) {
        match self {
            AfterArrayStart | AfterArrayComma | AfterArrayStartOrComma | AfterObjectFieldColon => {}
            InString => panic!(),
            AfterArrayStartOrItem | AfterArrayItem => panic!(),
            AfterObjectStart | AfterObjectComma | AfterObjectStartOrComma => panic!(),
            InObjectFieldName => panic!(),
            AfterObjectFieldName => panic!(),
            AfterObjectFieldValue => panic!(),
            AfterObjectStartOrFieldValue => panic!(),
        }
    }
}

pub(crate) const fn check(prev_state: State, next_state: State, s: &str) {
    let s = deserializer::Deserializer::new(s);
    let expected_next_state = match s.parse_till_eof_with_state(prev_state.0) {
        Ok(v) => v,
        Err(msg) => panic!("{}", msg),
    };

    next_state.0.assert_same(&expected_next_state);
}

/// Panics if `s` is not a json value or `s` contains json whitespaces.
pub(crate) const fn assert_json_value<'a>(s: &'a str) {
    let s = deserializer::Deserializer::new(s);

    let next_state = match s.parse_till_eof_with_state(StateInner::Init) {
        Ok(v) => v,
        Err(msg) => panic!("{}", msg),
    };

    match next_state {
        StateInner::Eof(_) => {}
        _ => panic!("invalid json value"),
    }
}

pub trait HasConstCompileTimeChunk {
    const CHUNK: super::StatedChunkStr<'static>;
}

pub struct CompileTimeChunk<T: ?Sized + HasConstCompileTimeChunk>(PhantomData<T>);

impl<T: ?Sized + HasConstCompileTimeChunk> fmt::Debug for CompileTimeChunk<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompileTimeChunk")
            .field("CHUNK", &T::CHUNK)
            .finish()
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> Copy for CompileTimeChunk<T> {}
impl<T: ?Sized + HasConstCompileTimeChunk> Clone for CompileTimeChunk<T> {
    fn clone(&self) -> Self {
        *self
    }
}

mod ser {
    use core::{iter, marker::PhantomData};

    use crate::ser::{
        iter_text_chunk::{HasConstChunk, IterNonLending},
        texts::ConstChunk,
        traits::{IntoTextChunks, proxy_IntoTextChunks},
    };

    use super::{CompileTimeChunk, HasConstCompileTimeChunk};

    pub struct Chunk<T: ?Sized + HasConstCompileTimeChunk>(PhantomData<T>);

    impl<T: ?Sized + HasConstCompileTimeChunk> HasConstChunk for Chunk<T> {
        const CHUNK: &'static str = T::CHUNK.into_inner();
    }

    impl<T: ?Sized + HasConstCompileTimeChunk> IntoTextChunks for CompileTimeChunk<T> {
        proxy_IntoTextChunks!(|self| -> ConstChunk<Chunk<T>> { ConstChunk::DEFAULT });
    }
}

enum Never {}

pub struct CompileTimeChunkIsJsonValue<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);

impl<T: ?Sized + HasConstCompileTimeChunk> HasConstJsonValue for CompileTimeChunkIsJsonValue<T> {
    const JSON_VALUE: crate::ser::texts::Value<&'static str> = {
        _ = CompileTimeChunk::<T>::JSON_VALUE;
        crate::ser::texts::Value::new_without_validation(T::CHUNK.into_inner())
    };
}

impl<T: ?Sized + HasConstCompileTimeChunk> CompileTimeChunk<T> {
    pub const DEFAULT: Self = {
        _ = T::CHUNK;
        Self(PhantomData)
    };

    pub const JSON_VALUE: Value<Self> = Value::new(Self::DEFAULT);

    pub const JSON_STRING: JsonString<Self> = JsonString::new(Self::JSON_VALUE);

    pub const JSON_ARRAY_NON_EMPTY: NonEmptyArray<Self> = NonEmptyArray::new(Self::JSON_VALUE);

    pub const JSON_OBJECT_NON_EMPTY: NonEmptyObject<Self> = NonEmptyObject::new(Self::JSON_VALUE);
}

mod deserializer;
