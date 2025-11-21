use crate::{r#const::array_string::ArrayString, utils::impl_many};

use super::IterTextChunk;

pub struct Chunk<Len, const CAP: usize>(ArrayString<Len, CAP>);

impl_many!(
    type Len = each_of![u8];

    impl<const CAP: usize> AsRef<[u8]> for Chunk<Len, CAP> {
        fn as_ref(&self) -> &[u8] {
            self.0.as_bytes()
        }
    }

    impl<const CAP: usize> IterTextChunk for ArrayString<Len, CAP> {
        type Chunk<'a>
            = Chunk<Len, CAP>
        where
            Self: 'a;

        fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
            if self.is_empty() {
                None
            } else {
                let chunk = *self;
                self.clear();
                Some(Chunk(chunk))
            }
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
