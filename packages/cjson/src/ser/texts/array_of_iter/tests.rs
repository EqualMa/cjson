use crate::ser::{
    texts::Value,
    traits::{Array, EmptyOrCommaSeparatedElements, IntoTextChunks as _},
};

use super::ArrayOfIter;

#[test]
fn json_array_of_iter() {
    assert_eq!(
        ArrayOfIter(([] as [Value<&'static str>; 0]).into_iter())._private_into_text_chunks_vec(),
        b"[]",
    );
    assert_eq!(
        ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
            ._private_into_text_chunks_vec(),
        b"[[],{}]",
    );
}

#[test]
fn json_items_of_iter() {
    const EMPTY: [Value<&'static str>; 0] = [];
    assert_eq!(
        ArrayOfIter(EMPTY.into_iter())
            .into_comma_separated_elements()
            ._private_into_text_chunks_vec(),
        b"",
    );

    assert_eq!(
        ArrayOfIter(EMPTY.into_iter())
            .into_comma_separated_elements()
            .prepend_leading_comma_if_not_empty()
            ._private_into_text_chunks_vec(),
        b"",
    );

    assert_eq!(
        ArrayOfIter(EMPTY.into_iter())
            .into_comma_separated_elements()
            .append_trailing_comma_if_not_empty()
            ._private_into_text_chunks_vec(),
        b"",
    );

    assert_eq!(
        ArrayOfIter(EMPTY.into_iter())
            .into_comma_separated_elements()
            .chain_with_comma(ArrayOfIter(EMPTY.into_iter()).into_comma_separated_elements())
            ._private_into_text_chunks_vec(),
        b"",
    );
    assert_eq!(
        ArrayOfIter(EMPTY.into_iter())
            .into_comma_separated_elements()
            .chain_with_comma(
                ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
                    .into_comma_separated_elements()
            )
            ._private_into_text_chunks_vec(),
        b"[],{}",
    );

    assert_eq!(
        ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
            .into_comma_separated_elements()
            ._private_into_text_chunks_vec(),
        b"[],{}",
    );
    assert_eq!(
        ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
            .into_comma_separated_elements()
            .prepend_leading_comma_if_not_empty()
            ._private_into_text_chunks_vec(),
        b",[],{}",
    );
    assert_eq!(
        ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
            .into_comma_separated_elements()
            .append_trailing_comma_if_not_empty()
            ._private_into_text_chunks_vec(),
        b"[],{},",
    );

    assert_eq!(
        ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
            .into_comma_separated_elements()
            .chain_with_comma(ArrayOfIter(EMPTY.into_iter()).into_comma_separated_elements())
            ._private_into_text_chunks_vec(),
        b"[],{}",
    );
    assert_eq!(
        ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
            .into_comma_separated_elements()
            .chain_with_comma(
                ArrayOfIter([Value::EMPTY_ARRAY, Value::EMPTY_OBJECT].into_iter())
                    .into_comma_separated_elements()
            )
            ._private_into_text_chunks_vec(),
        b"[],{},[],{}",
    );
}
