use core::{fmt, marker::PhantomData};

use crate::r#const::HasConstJsonValue;

use super::StatedChunkStr;

use self::IntermediateState::*;

pub struct State(StateInner);

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl State {
    pub(crate) const fn assert_same(self, other: Self) {
        self.0.assert_same(other.0);
    }

    pub const INIT: Self = Self(StateInner::Init);
    pub(crate) const EOF: Self = Self(StateInner::Eof);

    pub const fn json_value(self) -> Self {
        Self(match self.0 {
            StateInner::Init => StateInner::Eof,
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
            StateInner::Eof => {
                panic!()
            }
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
                            None => return Self::EOF,
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
            StateInner::Eof => panic!(),
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
            StateInner::Eof => {
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
            StateInner::Eof => {
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
            StateInner::Eof => panic!(),
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
            StateInner::Eof => panic!(),
        })
    }

    pub const fn right_bracket(self) -> Self {
        Self(match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                InString => panic!(),
                AfterArrayStart | AfterArrayItem | AfterArrayStartOrItem => {
                    stack.end_array().into_state_inner()
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
            StateInner::Eof => panic!(),
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
            StateInner::Eof => panic!(),
        })
    }

    pub const fn right_brace(self) -> Self {
        Self(match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                AfterObjectStart | AfterObjectFieldValue | AfterObjectStartOrFieldValue => {
                    stack.end_object().into_state_inner()
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
            StateInner::Eof => panic!(),
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
            StateInner::Eof => panic!(),
        }
    }

    pub const fn json_items_after_array_start_before_item(self) -> State {
        match self.0 {
            StateInner::Init => panic!(),
            StateInner::Intermediate(Intermediate { stack, state }) => match state {
                AfterArrayStart => Self(StateInner::Intermediate(Intermediate {
                    stack,
                    state: AfterArrayStartOrComma,
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
            StateInner::Eof => panic!(),
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
            StateInner::Eof => panic!(),
        }
    }

    pub(crate) const fn copied(&self) -> Self {
        Self(match &self.0 {
            StateInner::Init => StateInner::Init,
            StateInner::Intermediate(intermediate) => {
                StateInner::Intermediate(intermediate.copied())
            }
            StateInner::Eof => StateInner::Eof,
        })
    }
}

#[derive(Debug)]
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
}

#[derive(Debug)]
enum StateInner {
    Init,
    Intermediate(Intermediate),
    Eof,
}

impl StateInner {
    const fn assert_same(self, other: Self) {
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
            (StateInner::Eof, StateInner::Eof) => {}
            _ => panic!("State mismatch"),
        }
    }
}

type StackInner = u64;

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
    const INIT: Self = Self { inner: 0, len: 0 };

    const fn is_in_array_or_object(&self) -> Option<bool> {
        if self.len == 0 {
            None
        } else {
            Some((self.inner & 1) != 0)
        }
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

impl AfterEndArrayOrObject {
    const fn into_state_inner(self) -> StateInner {
        match self {
            Self::Intermediate(intermediate) => StateInner::Intermediate(intermediate),
            Self::Eof => StateInner::Eof,
        }
    }
}

macro_rules! define_inter_state {
    (
        $(#$attr:tt)*
        $vis:vis enum $IntermediateState:ident {
            $($Var:ident),+ $(,)?
        }

        #[assert_same]
        fn $assert_same:ident();

        #[copied]
        fn $copied:ident();
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
        }
    };
}

define_inter_state!(
    #[derive(Debug)]
    enum IntermediateState {
        InString,
        AfterArrayStart,
        AfterArrayStartOrComma,
        AfterArrayStartOrItem,
        AfterArrayItem,
        AfterArrayComma,
        AfterObjectStart,
        AfterObjectStartOrComma,
        AfterObjectStartOrFieldValue,
        InObjectFieldName,
        AfterObjectFieldName,
        AfterObjectFieldColon,
        AfterObjectFieldValue,
        AfterObjectComma,
    }

    #[assert_same]
    fn assert_same();

    #[copied]
    fn copied();
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

impl<'a> StatedChunkStr<'a> {
    pub(crate) const fn assert(self) {
        let s = deserializer::Deserializer::new(self.chunk);
        let next_state = match s.parse_till_eof_with_state(self.prev_state.0) {
            Ok(v) => v,
            Err(msg) => panic!("{}", msg),
        };

        self.next_state.0.assert_same(next_state);
    }
}

/// Panics if `s` is not a json value or `s` contains json whitespaces.
pub(crate) const fn assert_json_value<'a>(s: &'a str) {
    StatedChunkStr {
        prev_state: State::INIT,
        next_state: State(StateInner::Eof),
        chunk: s,
    }
    .assert();
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

    use crate::ser::{iter_text_chunk::IterNonLending, traits::IntoTextChunks};

    use super::{CompileTimeChunk, HasConstCompileTimeChunk};

    pub struct Chunk<T: ?Sized + HasConstCompileTimeChunk>(PhantomData<T>);

    impl<T: ?Sized + HasConstCompileTimeChunk> AsRef<[u8]> for Chunk<T> {
        fn as_ref(&self) -> &[u8] {
            const { T::CHUNK.chunk.as_bytes() }
        }
    }

    impl<T: ?Sized + HasConstCompileTimeChunk> IntoTextChunks for CompileTimeChunk<T> {
        type IntoTextChunks = IterNonLending<iter::Once<Chunk<T>>>;

        fn into_text_chunks(self) -> Self::IntoTextChunks {
            IterNonLending(iter::once(Chunk(PhantomData)))
        }

        #[cfg(feature = "alloc")]
        fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
            const { T::CHUNK.chunk }.into()
        }
    }
}

enum Never {}

pub struct CompileTimeChunkIsJsonValue<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);

impl<T: ?Sized + HasConstCompileTimeChunk> HasConstJsonValue for CompileTimeChunkIsJsonValue<T> {
    const JSON_VALUE: crate::ser::texts::Value<&'static str> = {
        () = CompileTimeChunk::<T>::ASSERT_JSON_VALUE;
        crate::ser::texts::Value::new_without_validation(T::CHUNK.chunk)
    };
}

impl<T: ?Sized + HasConstCompileTimeChunk> CompileTimeChunk<T> {
    pub const DEFAULT: Self = Self(PhantomData);

    pub(crate) const ASSERT: () = T::CHUNK.assert();

    const ASSERT_JSON_VALUE: () = {
        assert!(matches!(T::CHUNK.prev_state.0, StateInner::Init));
        assert!(matches!(T::CHUNK.next_state.0, StateInner::Eof));
    };

    pub const JSON_VALUE: super::ConstJsonValue<CompileTimeChunkIsJsonValue<T>> = {
        () = Self::ASSERT_JSON_VALUE;
        super::ConstJsonValue::new()
    };
}

mod deserializer;
