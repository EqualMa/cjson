use core::marker::PhantomData;

use crate::{
    r#const::State,
    ser::traits::{ConsumeTextChunk, TryConsumeTextChunk},
    utils::impl_many,
};

use super::{Consumed, HasConstState, IntoJson, json_kinds};

// TODO: sealed
pub trait RuntimeChunks {
    type NextState<Prev: ?Sized + HasConstState>: ?Sized + HasConstState;

    fn runtime_chunks_write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W);
    fn runtime_chunks_try_write_into<W: ?Sized + TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err>;
}

pub struct JsonValue<T: IntoJson>(pub T);

pub struct JsonStringFragment<T: IntoJson<JsonKind = json_kinds::JsonString>>(pub T);

pub struct JsonItemsAfterItem<T: IntoJson<JsonKind = json_kinds::Array>>(pub T);
pub struct JsonItemsAfterArrayStartBeforeItem<T: IntoJson<JsonKind = json_kinds::Array>>(pub T);
pub struct JsonItemsBetweenBrackets<T: IntoJson<JsonKind = json_kinds::Array>>(pub T);

enum Never {}
pub struct StateThenJsonValue<Prev: ?Sized + HasConstState>(Never, PhantomData<Prev>);

impl<Prev: ?Sized + HasConstState> HasConstState for StateThenJsonValue<Prev> {
    const STATE: State = Prev::STATE.json_value();
}

impl<T: IntoJson> RuntimeChunks for JsonValue<T> {
    type NextState<Prev: ?Sized + HasConstState> = StateThenJsonValue<Prev>;

    fn runtime_chunks_write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W) {
        todo!()
        // let Consumed(_, PhantomData) = self
        //     .0
        //     .json_provide_into(super::ConsumeJsonText(w.as_mut_consume_text_chunk()));
    }

    fn runtime_chunks_try_write_into<W: ?Sized + TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        todo!()
    }
}

#[cfg(todo)]
impl_many!({
    {
        {
            use ChunkConcatJsonValue as CR;
            use ToJson as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> = <V as ToJson>::ToJson<'a>;
            fn runtime_chunk_to_text_chunk<V: ToJson>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json(v)
            }
        }
        {
            use ChunkConcatJsonStringFragment as CR;
            use ToJsonString as ToTrait;
            type FragmentsOf<S> = <S as traits::JsonString>::IntoJsonStringFragments;
            type RuntimeChunkToTextChunk<'a, V> =
                FragmentsOf<<V as ToJsonString>::ToJsonString<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonString>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_string(v).into_json_string_fragments()
            }
        }
        {
            use ChunkConcatJsonItemsAfterItem as CR;
            use ToJsonArray as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> =
                JsonItemsAfterItem<<V as ToJsonArray>::ToJsonArray<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonArray>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_array(v)
                    .into_comma_separated_elements()
                    .prepend_leading_comma_if_not_empty()
            }
        }
        {
            use ChunkConcatJsonItemsAfterArrayStartBeforeItem as CR;
            use ToJsonArray as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> =
                JsonItemsAfterArrayStartBeforeItem<<V as ToJsonArray>::ToJsonArray<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonArray>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_array(v)
                    .into_comma_separated_elements()
                    .append_trailing_comma_if_not_empty()
            }
        }
        {
            use ChunkConcatJsonItemsBetweenBrackets as CR;
            use ToJsonArray as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> =
                JsonItemsBetweenBrackets<<V as ToJsonArray>::ToJsonArray<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonArray>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_array(v).into_comma_separated_elements()
            }
        }
    }
});
