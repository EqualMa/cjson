use crate::ser::{
    texts::Value,
    traits::{Array, IntoTextChunks},
};

use super::{super::ArrayOfIter, Bracketed};

#[test]
fn bracketed() {
    assert_eq!(
        Bracketed(
            ArrayOfIter(core::iter::empty::<Value<&'static str>>()).into_comma_separated_elements(),
        )
        ._private_into_text_chunks_vec(),
        b"[]",
    );
}
