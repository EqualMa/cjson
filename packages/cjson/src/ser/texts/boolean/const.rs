use crate::{
    r#const::{AsRefU8Slice, ConstIntoJsonValueString, ConstIntoTextChunks, ConstIterTextChunk},
    ser::texts,
};

use super::{Boolean, Chunk};

impl ConstIntoJsonValueString<Boolean> {
    pub const fn const_into_json_value_string_len(self) -> usize {
        Chunk(self.0.0).as_ref_str().len()
    }

    pub const fn const_into_json_value_string<const LEN: usize>(
        self,
    ) -> texts::Value<&'static str> {
        let s = self.const_into_json_value_string_without_const_len();
        assert!(s.inner().len() == LEN);
        s
    }

    pub const fn const_into_json_value_string_without_const_len(
        self,
    ) -> texts::Value<&'static str> {
        let s = Chunk(self.0.0).as_ref_str();
        texts::Value::new_without_validation(s)
    }

    pub const fn const_concat_after_stated_chunk_buf<const CAP: usize>(
        self,
        chunk_buf: crate::r#const::StatedChunkBuf<CAP>,
    ) -> crate::r#const::StatedChunkBuf<CAP> {
        chunk_buf.json_value(self.const_into_json_value_string_without_const_len())
    }
}

impl AsRefU8Slice<Chunk> {
    pub const fn as_ref_u8_slice(&self) -> &[u8] {
        self.0.as_ref_u8_slice()
    }
}

pub struct Chunks(Option<Chunk>);

impl ConstIntoTextChunks<Boolean> {
    pub const fn const_into_text_chunks(self) -> Chunks {
        Chunks(Some(Chunk(self.0.0)))
    }
}

impl ConstIterTextChunk<Chunks> {
    pub const fn const_next_text_chunk(&mut self) -> Option<Chunk> {
        self.0.0.take()
    }
}
