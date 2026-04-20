use crate::utils::impl_many;

#[derive(Debug, Clone, Copy)]
pub struct ArrayString<Len, const CAP: usize> {
    len: Len,
    /// &bytes[..len] must be valid UTF-8
    bytes: [u8; CAP],
}

impl_many!(
    type Len = each_of![u8];

    impl<const CAP: usize> ArrayString<Len, CAP> {
        const ASSERT_LEN_SIZE: () = {
            assert!(core::mem::size_of::<Len>() <= core::mem::size_of::<usize>());
            assert!(CAP <= Len::MAX as usize);
        };

        pub(crate) const NEW: Self = Self {
            len: 0,
            bytes: [0; CAP],
        };

        pub(crate) const fn len(&self) -> usize {
            const { Self::ASSERT_LEN_SIZE };
            self.len as usize
        }

        pub(crate) const fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub(crate) const fn as_str(&self) -> &str {
            let bytes = self.as_bytes();
            // SAFETY: self.as_bytes() is valid UTF-8
            unsafe { str::from_utf8_unchecked(bytes) }
        }

        pub(crate) const fn take_non_empty_str(&mut self) -> Option<&str> {
            if self.len == 0 {
                return None;
            }
            let bytes = self.bytes.split_at(self.len()).0; // inline self.as_bytes()
            self.len = 0;
            // SAFETY: self.as_bytes() is valid UTF-8
            let s = unsafe { str::from_utf8_unchecked(bytes) };
            Some(s)
        }

        pub(crate) const fn as_bytes(&self) -> &[u8] {
            self.bytes.split_at(self.len()).0
        }

        pub(crate) const fn from_str(v: &str) -> Self {
            const { Self::ASSERT_LEN_SIZE };
            assert!(v.len() <= CAP);
            Self {
                len: v.len() as Len,
                bytes: {
                    let mut bytes = [0u8; CAP];
                    bytes.split_at_mut(v.len()).0.copy_from_slice(v.as_bytes());
                    bytes
                },
            }
        }

        pub(crate) const fn clear(&mut self) {
            self.len = 0
        }
    }

    impl<const CAP: usize> core::fmt::Write for ArrayString<Len, CAP> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let len = self.len();
            let (_, tail) = self.bytes.split_at_mut(len);
            let Some((write_to, _)) = tail.split_at_mut_checked(s.len()) else {
                return Err(core::fmt::Error);
            };

            write_to.copy_from_slice(s.as_bytes());

            self.len += s.len() as u8; // we have checked s is smaller than CAP

            Ok(())
        }
    }
);

// TODO: is this needed?
impl<Len, const CAP: usize> crate::ser::traits::IntoTextChunks for ArrayString<Len, CAP> {
    type IntoTextChunks = crate::ser::iter_text_chunk::NeverTextChunk;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        todo!()
    }

    fn write_into<W: ?Sized + crate::ser::traits::ConsumeTextChunk>(self, w: &mut W) {
        todo!()
    }

    fn try_write_into<W: ?Sized + crate::ser::traits::TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        todo!()
    }
}
