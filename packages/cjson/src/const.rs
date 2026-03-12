use core::marker::PhantomData;

use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::ser::{ToJson, iter_text_chunk::IterNonLending, texts, traits::IntoTextChunks};

pub struct ConstIntoJson<T>(pub T);

pub struct ConstIntoJsonValueString<T>(pub T);
pub struct ConstAsJsonValueStr<T>(pub T);

pub struct ConstIntoTextChunks<T: ?Sized>(pub T);
pub struct ConstIterTextChunk<T: ?Sized>(pub T);

pub struct BooleanTextChunks(Option<bool>);

impl BooleanTextChunks {
    const fn next_text_chunk() {}
}

#[derive(Debug, RefCastCustom)]
#[repr(transparent)]
pub struct AsRefU8Slice<T>(pub T);

impl<T> AsRefU8Slice<T> {
    #[ref_cast_custom]
    pub(crate) const fn new_ref(s: &T) -> &Self;
}

pub trait HasConstJsonValue {
    const JSON_VALUE: texts::Value<&'static str>;
}

pub struct ConstJsonValue<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> ConstJsonValue<T> {
    pub const DEFAULT: Self = Self(PhantomData);
    pub const fn new() -> Self {
        Self::DEFAULT
    }
}

impl<T: ?Sized> Copy for ConstJsonValue<T> {}
impl<T: ?Sized> Clone for ConstJsonValue<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Default for ConstJsonValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct JsonValueString<const LEN: usize>([u8; LEN]);

impl<T: ?Sized + HasConstJsonValue> ConstJsonValue<T> {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        T::JSON_VALUE
    }
}

mod ser {
    use core::marker::PhantomData;

    use crate::ser::{
        ToJson,
        iter_text_chunk::IterNonLending,
        traits::{self, IntoTextChunks},
    };

    use super::{ConstJsonValue, HasConstJsonValue};

    pub struct Chunk<T: ?Sized + HasConstJsonValue>(PhantomData<T>);

    impl<T: ?Sized + HasConstJsonValue> AsRef<[u8]> for Chunk<T> {
        fn as_ref(&self) -> &[u8] {
            T::JSON_VALUE.inner().as_bytes()
        }
    }

    impl<T: ?Sized + HasConstJsonValue> ToJson for ConstJsonValue<T> {
        type ToJson<'a>
            = Self
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            *self
        }
    }

    impl<T: ?Sized + HasConstJsonValue> IntoTextChunks for ConstJsonValue<T> {
        type IntoTextChunks = IterNonLending<core::iter::Once<Chunk<T>>>;

        fn into_text_chunks(self) -> Self::IntoTextChunks {
            IterNonLending(core::iter::once(Chunk(PhantomData)))
        }
    }

    impl<T: ?Sized + HasConstJsonValue> traits::sealed::Text for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::Text for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::sealed::Value for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::Value for ConstJsonValue<T> {}

    mod r#const {}
}

#[cfg(test)]
mod const_tests {
    use super::{ConstIntoJson, ConstIntoTextChunks, ConstIterTextChunk};

    const fn assert_ser_bool(v: bool) {
        let mut chunks = ConstIterTextChunk(
            ConstIntoTextChunks(ConstIntoJson(v).const_into_json()).const_into_text_chunks(),
        );

        let Some(chunk) = chunks.const_next_text_chunk() else {
            panic!()
        };
        let chunk = chunk.as_ref_u8_slice();

        if v {
            assert!(matches!(chunk, b"true"));
        } else {
            assert!(matches!(chunk, b"false"));
        }

        assert!(chunks.const_next_text_chunk().is_none());
    }

    const _: () = {
        assert_ser_bool(true);
        assert_ser_bool(false);
    };
}

pub(crate) mod array_string;
pub(crate) mod json_value_array_str;

mod state;

pub use self::state::{CompileTimeChunk, HasConstCompileTimeChunk, State};

pub(crate) use self::state::assert_json_value;

pub mod array;

#[derive(Debug)]
pub struct StatedChunkStr<'a> {
    prev_state: State,
    next_state: State,
    chunk: &'a str,
}

impl<'a> StatedChunkStr<'a> {
    pub const fn next_state(self) -> State {
        self.next_state
    }
}

pub struct StatedChunkString<const LEN: usize> {
    prev_state: State,
    next_state: State,
    chunk: [u8; LEN],
}

impl<const LEN: usize> StatedChunkString<LEN> {
    pub const fn as_str(&self) -> StatedChunkStr<'_> {
        StatedChunkStr {
            prev_state: self.prev_state.copied(),
            next_state: self.next_state.copied(),
            chunk: unsafe { str::from_utf8_unchecked(&self.chunk) },
        }
    }
}

pub struct StatedChunkBuf<const CAP: usize> {
    prev_state: State,
    cur_state: State,
    buf: ChunkBuf<CAP>,
}

impl<const CAP: usize> StatedChunkBuf<CAP> {
    pub const fn new(prev_state: State) -> Self {
        Self {
            prev_state: prev_state.copied(),
            cur_state: prev_state,
            buf: ChunkBuf::DEFAULT,
        }
    }
}

pub struct ChunkLen(usize);
struct ChunkBuf<const CAP: usize> {
    buf: [u8; CAP],
    len: usize,
}

impl ChunkLen {
    pub const DEFAULT: Self = Self(0);

    pub const fn len(self) -> usize {
        self.0
    }

    pub const fn left_bracket(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn right_bracket(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn comma(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn json_value(mut self, len: usize) -> Self {
        assert!(len > 0);
        self.0 += len;
        self
    }
}

impl<const CAP: usize> ChunkBuf<CAP> {
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

    pub const fn comma(self) -> Self {
        self.with_byte(b',')
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

    const fn json_value(self, value: texts::Value<&'_ str>) -> Self {
        self.with_str(value.inner())
    }

    pub const fn double_quote(self) -> Self {
        self.with_byte(b'"')
    }

    const fn json_string_fragments(self, chunk: &[u8]) -> Self {
        self.with_bytes(chunk)
    }

    const fn assert(self) -> [u8; CAP] {
        assert!(self.len == CAP);
        debug_assert!(str::from_utf8(&self.buf).is_ok());
        self.buf
    }
}

impl<const CAP: usize> StatedChunkBuf<CAP> {
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

    pub const fn comma(self) -> Self {
        Self {
            prev_state: self.prev_state,
            cur_state: self.cur_state.comma(),
            buf: self.buf.comma(),
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
            cur_state: self.cur_state.json_string_fragments(),
            buf: self.buf.json_string_fragments(chunk),
        }
    }

    pub const fn assert(self) -> StatedChunkString<CAP> {
        StatedChunkString {
            prev_state: self.prev_state,
            next_state: self.cur_state,
            chunk: self.buf.assert(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkConcatJsonValue<C: RuntimeChunk, V: ToJson>(pub C, pub V);

impl<C: RuntimeChunk, V: ToJson> RuntimeChunk for ChunkConcatJsonValue<C, V> {
    const PREV_STATE: State = C::PREV_STATE;
    const NEXT_STATE: State = C::NEXT_STATE.json_value();

    type ToIntoTextChunks<'a>
        = crate::ser::iter_text_chunk::Chain<
        <C::ToIntoTextChunks<'a> as IntoTextChunks>::IntoTextChunks,
        <V::ToJson<'a> as IntoTextChunks>::IntoTextChunks,
    >
    where
        Self: 'a;

    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_> {
        const { _ = Self::NEXT_STATE }
        crate::ser::iter_text_chunk::Chain::new(
            self.0.to_into_text_chunks().into_text_chunks(),
            self.1.to_json().into_text_chunks(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkConcat<A: RuntimeChunk, B: RuntimeChunk>(pub A, pub B);

impl<A: RuntimeChunk, B: RuntimeChunk> ChunkConcat<A, B> {
    const ASSERT: () = {
        A::NEXT_STATE.assert_same(B::PREV_STATE);
    };
}

impl<A: RuntimeChunk, B: RuntimeChunk> RuntimeChunk for ChunkConcat<A, B> {
    const PREV_STATE: State = {
        Self::ASSERT;
        A::PREV_STATE
    };
    const NEXT_STATE: State = {
        Self::ASSERT;
        B::NEXT_STATE
    };

    type ToIntoTextChunks<'a>
        = crate::ser::iter_text_chunk::Chain<
        <A::ToIntoTextChunks<'a> as IntoTextChunks>::IntoTextChunks,
        <B::ToIntoTextChunks<'a> as IntoTextChunks>::IntoTextChunks,
    >
    where
        Self: 'a;

    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_> {
        const { () = Self::ASSERT }

        crate::ser::iter_text_chunk::Chain::new(
            self.0.to_into_text_chunks().into_text_chunks(),
            self.1.to_into_text_chunks().into_text_chunks(),
        )
    }
}

// TODO: sealed
pub trait RuntimeChunk {
    const PREV_STATE: State;
    const NEXT_STATE: State;

    type ToIntoTextChunks<'a>: IntoTextChunks
    where
        Self: 'a;
    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_>;
}

impl<T: ?Sized + HasConstCompileTimeChunk> RuntimeChunk for CompileTimeChunk<T> {
    const PREV_STATE: State = T::CHUNK.prev_state;
    const NEXT_STATE: State = T::CHUNK.next_state;

    type ToIntoTextChunks<'a>
        = Self
    where
        Self: 'a;

    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_> {
        *self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AssertJsonValueChunks<C: RuntimeChunk>(pub C);

impl<C: RuntimeChunk> AssertJsonValueChunks<C> {
    const ASSERT: () = {
        C::PREV_STATE.assert_same(State::INIT);
        C::NEXT_STATE.assert_same(State::EOF);
    };
}

mod ser_chunks {
    use crate::ser::{
        ToJson,
        traits::{self, IntoTextChunks},
    };

    use super::{AssertJsonValueChunks, RuntimeChunk};

    pub struct AssertJsonValueChunksToJson<'a, C: RuntimeChunk>(&'a C);

    impl<C: RuntimeChunk> ToJson for AssertJsonValueChunks<C> {
        type ToJson<'a>
            = AssertJsonValueChunksToJson<'a, C>
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            const { () = Self::ASSERT }

            AssertJsonValueChunksToJson(&self.0)
        }
    }

    impl<C: RuntimeChunk> traits::sealed::Text for AssertJsonValueChunksToJson<'_, C> {}
    impl<C: RuntimeChunk> traits::Text for AssertJsonValueChunksToJson<'_, C> {}

    impl<'a, C: RuntimeChunk> IntoTextChunks for AssertJsonValueChunksToJson<'a, C> {
        type IntoTextChunks = <C::ToIntoTextChunks<'a> as IntoTextChunks>::IntoTextChunks;

        fn into_text_chunks(self) -> Self::IntoTextChunks {
            const { AssertJsonValueChunks::<C>::ASSERT }
            C::to_into_text_chunks(self.0).into_text_chunks()
        }
    }
}
