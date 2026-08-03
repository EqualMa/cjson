pub mod base {
    pub(crate) use crate::ser::consumers::help_ancestor::HelpAncestorToConsumeChild as HELP_ANCESTOR_TO_CONSUME_CHILD;

    pub use crate::ser::{
        //
        consumers::{
            //
            ConsumeChained as CONSUME_CHAINED,
            ConsumeJson as CONSUME_JSON,
            ConsumeJsonChunks as CONSUME_JSON_CHUNKS,
            ConsumeJsonChunksFromInit as CONSUME_JSON_CHUNKS_FROM_INIT,
            chunks::{
                ReadyToConsumeJsonChunksOfNonEmptyArray as READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY,
                ReadyToConsumeJsonChunksOfNonEmptyObject as READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT,
            },
            json_string_chunks::{
                //
                ConsumeFragmentInString as CONSUME_FRAGMENT_IN_STRING,
                ConsumeInJsonString as CONSUME_IN_JSON_STRING,
                EndJsonString as END_JSON_STRING,
            },
        },
        traits::ConsumeTextChunk as CONSUME_TEXT_CHUNK,
    };
}

pub mod try_ {
    pub(crate) use crate::ser::consumers::help_ancestor::HelpAncestorToTryConsumeChild as HELP_ANCESTOR_TO_CONSUME_CHILD;

    pub use crate::ser::{
        //
        consumers::{
            //
            TryConsumeChained as CONSUME_CHAINED,
            TryConsumeJson as CONSUME_JSON,
            TryConsumeJsonChunks as CONSUME_JSON_CHUNKS,
            TryConsumeJsonChunksFromInit as CONSUME_JSON_CHUNKS_FROM_INIT,
            chunks::{
                ReadyToTryConsumeJsonChunksOfNonEmptyArray as READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY,
                ReadyToTryConsumeJsonChunksOfNonEmptyObject as READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT,
            },
            json_string_chunks::{
                //
                TryConsumeFragmentInString as CONSUME_FRAGMENT_IN_STRING,
                TryConsumeInJsonString as CONSUME_IN_JSON_STRING,
                TryEndJsonString as END_JSON_STRING,
            },
        },
        traits::TryConsumeTextChunk as CONSUME_TEXT_CHUNK,
    };
}

pub mod async_try {
    pub(crate) use crate::ser::consumers::help_ancestor::HelpAncestorToAsyncTryConsumeChild as HELP_ANCESTOR_TO_CONSUME_CHILD;

    pub use crate::ser::{
        consumers::{
            //
            AsyncTryConsumeChained as CONSUME_CHAINED,
            AsyncTryConsumeJson as CONSUME_JSON,
            AsyncTryConsumeJsonChunks as CONSUME_JSON_CHUNKS,
            AsyncTryConsumeJsonChunksFromInit as CONSUME_JSON_CHUNKS_FROM_INIT,
            chunks::{
                ReadyToAsyncTryConsumeJsonChunksOfNonEmptyArray as READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY,
                ReadyToAsyncTryConsumeJsonChunksOfNonEmptyObject as READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT,
            },
            json_string_chunks::{
                //
                AsyncTryConsumeFragmentInString as CONSUME_FRAGMENT_IN_STRING,
                AsyncTryConsumeInJsonString as CONSUME_IN_JSON_STRING,
                AsyncTryEndJsonString as END_JSON_STRING,
            },
        },
        traits::AsyncTryConsumeTextChunk as CONSUME_TEXT_CHUNK,
    };
}
