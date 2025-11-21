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

        pub(crate) const fn as_bytes(&self) -> &[u8] {
            self.bytes.split_at(self.len()).0
        }

        pub(crate) const fn from_str(v: &str) -> Self {
            const { Self::ASSERT_LEN_SIZE };
            assert!(v.len() <= CAP);
            assert!(v.len() <= (Len::MAX as usize));
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
);
