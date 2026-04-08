use core::marker::PhantomData;

use alloc::vec::Vec;

use super::{ConstChunk, HasConstChunk, IterTextChunk};

pub(super) enum Inner<T: ?Sized> {
    Init(PhantomData<T>),
    End,
}

impl<T: ?Sized> Inner<T> {
    pub const DEFAULT: Self = Self::Init(PhantomData);
}

pub struct Chunk<T: ?Sized + HasConstChunk>(PhantomData<T>);

impl<T: ?Sized + HasConstChunk> AsRef<[u8]> for Chunk<T> {
    fn as_ref(&self) -> &[u8] {
        const { T::CHUNK.as_bytes() }
    }
}

impl<T: ?Sized + HasConstChunk> IterTextChunk for ConstChunk<T> {
    type Chunk<'a>
        = Chunk<T>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        match self.0 {
            Inner::Init(PhantomData::<_>) => {
                self.0 = Inner::End;
                Some(Chunk(PhantomData))
            }
            Inner::End => None,
        }
    }

    fn _private_collect_into_vec(self) -> ::alloc::vec::Vec<u8> {
        match self.0 {
            Inner::Init(PhantomData::<_>) => const { T::CHUNK.as_bytes() }.to_vec(),
            Inner::End => Vec::new(),
        }
    }
}
