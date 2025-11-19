#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
use crate::ser::texts;
use crate::ser::traits;

pub trait TextExt: traits::Text {
    #[cfg(feature = "alloc")]
    fn into_string(self) -> texts::Text<String>
    where
        Self: Sized,
    {
        let bytes = self._private_into_text_chunks_vec();
        // SAFETY: traits::Text promised the emitted chunks are valid utf8 bytes.
        let s = unsafe { String::from_utf8_unchecked(bytes) };
        texts::Text::new_without_validation(s)
    }
}

impl<T: ?Sized + traits::Text> TextExt for T {}
