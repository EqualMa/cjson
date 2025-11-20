use crate::{ser::iter_text_chunk::IterTextChunk, utils::option_map_or};

use super::escape;

#[derive(Debug)]
pub struct Chunks<'a> {
    iter_bytes: Iter<'a>,
    escaped: Option<&'static [u8]>,
}

#[derive(Debug)]
struct Iter<'a>(&'a [u8]);

impl<'a> Iter<'a> {
    const fn len(&self) -> usize {
        self.0.len()
    }
    const fn as_slice(&self) -> &'a [u8] {
        self.0
    }
    const fn position_needs_escape(&mut self) -> Option<usize> {
        let mut i = 0usize;

        while i < self.0.len() {
            if escape::needs_escape(&self.0[i]) {
                self.0 = self.0.split_at(i + 1).1;

                return Some(i);
            }
            i += 1;
        }

        self.0 = b"";

        None
    }
}

trait PositionNeedsEscape {
    fn position_needs_escape(&mut self) -> Option<usize>;
}

impl PositionNeedsEscape for core::slice::Iter<'_, u8> {
    #[inline(always)]
    fn position_needs_escape(&mut self) -> Option<usize> {
        self.position(escape::needs_escape)
    }
}

impl<'a> Chunks<'a> {
    pub(super) const fn new(s: &'a str) -> Self {
        Self {
            iter_bytes: Iter(s.as_bytes()),
            escaped: None,
        }
    }
}

macro_rules! concat {
    ({$($a:tt)*}{$($b:tt)*}) => {
        $($a)*$($b)*
    };
}

macro_rules! impl_for {
    (
        impl<$lt:lifetime> each_of![$( $(#[$const:ident])? $Ty:path),+ $(,)?] $imp:tt
    ) => {
        $(
            impl<$lt> $Ty {
                concat! { {$($const)?} $imp }
            }
        )+
    };
}

impl_for!(
    impl<'a>
        each_of![
            //
            #[const]
            Chunks<'a>,
            super::Chunks<'a>,
        ]
    {
        fn next(&mut self) -> Option<&'a [u8]> {
            if let Some(ch) = self.escaped.take() {
                return Some(ch);
            }

            if self.iter_bytes.len() == 0 {
                return None;
            }

            let bytes = self.iter_bytes.as_slice();

            match self.iter_bytes.position_needs_escape() {
                Some(i) => {
                    let byte = bytes[i];

                    let escaped = unsafe { escape::escape_to_bytes_unchecked(byte) };

                    if i == 0 {
                        Some(escaped)
                    } else {
                        self.escaped = Some(escaped);

                        Some(bytes.split_at(i).0)
                    }
                }
                None => Some(bytes),
            }
        }
    }
);

impl<'this> Chunks<'this> {
    pub const fn next_text_chunk(&mut self) -> Option<&'this [u8]> {
        self.next()
    }

    pub const fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        bytes_len_hint(self.escaped, self.iter_bytes.len())
    }
}

impl<'this> IterTextChunk for super::Chunks<'this> {
    type Chunk<'a>
        = &'this [u8]
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.next()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        bytes_len_hint(self.escaped, self.iter_bytes.len())
    }
}

const fn bytes_len_hint(escaped: Option<&[u8]>, bytes_len: usize) -> (usize, Option<usize>) {
    bytes_len_hint_impl(option_map_or!(escaped, 0, <[u8]>::len), bytes_len)
}

const fn bytes_len_hint_impl(escaped_len: usize, bytes_len: usize) -> (usize, Option<usize>) {
    (
        escaped_len.saturating_add(bytes_len),
        match bytes_len.checked_mul(6) {
            Some(v) => escaped_len.checked_add(v),
            None => None,
        },
    )
}
