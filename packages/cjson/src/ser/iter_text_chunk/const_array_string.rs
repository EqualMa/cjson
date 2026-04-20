use crate::{r#const::array_string::ArrayString, utils::impl_many};

use super::IterTextChunk;

impl_many!(
    type Len = each_of![u8];

    impl<const CAP: usize> IterTextChunk for ArrayString<Len, CAP> {
        type Chunk<'a>
            = &'a str
        where
            Self: 'a;

        fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
            self.take_non_empty_str()
        }

        fn bytes_len_hint(&self) -> (usize, Option<usize>) {
            let len = self.len();
            (len, Some(len))
        }

        #[doc(hidden)]
        #[cfg(feature = "alloc")]
        fn _private_collect_into_vec(self) -> ::alloc::vec::Vec<u8> {
            self.as_bytes().into()
        }
    }
);
