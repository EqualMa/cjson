use alloc::{string::String, vec::Vec};

use crate::{ser::iter_text_chunk::IterTextChunk, utils::impl_many};

impl_many!(
    impl<__> IterTextChunk for each_of![String, Vec<u8>] {
        type Chunk<'a>
            = Self
        where
            Self: 'a;

        fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
            if self.is_empty() {
                None
            } else {
                let s = core::mem::take(self);
                Some(s)
            }
        }
    }
);
