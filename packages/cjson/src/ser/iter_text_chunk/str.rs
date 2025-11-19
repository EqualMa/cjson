use super::IterTextChunk;

impl IterTextChunk for &str {
    type Chunk<'a>
        = Self
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        if self.is_empty() {
            None
        } else {
            let this = *self;
            *self = "";
            Some(this)
        }
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    fn _private_collect_into_vec(self) -> ::alloc::vec::Vec<u8> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::IterTextChunk;

    #[test]
    fn empty() {
        assert!("".next_text_chunk().is_none());
    }

    #[test]
    fn hello() {
        let mut s = "hello";
        assert_eq!(s.next_text_chunk(), Some("hello"));
        assert!(s.next_text_chunk().is_none());
    }
}
