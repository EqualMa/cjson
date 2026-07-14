use core::marker::PhantomData;

use super::{HasConstCompileTimeChunk, HasConstState, State};

pub enum Init {}

impl HasConstState for Init {
    const STATE: State = State::INIT;
}

enum Never {}
pub struct NextStateOf<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);

impl<T: ?Sized + HasConstCompileTimeChunk> HasConstState for NextStateOf<T> {
    const STATE: State = T::CHUNK.into_next_state();
}

macro_rules! define_then {
    (
        $($Then:ident :: $then:ident ),+ $(,)?
    ) => {
        $(
            pub struct $Then<T: ?Sized + HasConstState>(Never, PhantomData<T>);

            impl<T: ?Sized + HasConstState> HasConstState for $Then<T> {
                const STATE: State = T::STATE.$then();
            }
        )+
    };
}

define_then!(
    ThenValue::json_value,
    ThenItemsAfterItem::json_items_after_item,
    ThenItemsAfterArrayStartBeforeItem::json_items_after_array_start_before_item,
    ThenKvsAfterFieldValue::json_kvs_after_field_value,
    ThenKvsAfterObjectStartBeforeKv::json_kvs_after_object_start_before_kv,
    ThenLeftBracket::left_bracket,
    ThenLeftBrace::left_brace,
    ThenDoubleQuote::double_quote,
    ThenStringFragment::json_string_fragment,
);

type LeftBracket = ThenLeftBracket<Init>;

pub type LeftBracketValue = ThenValue<LeftBracket>;
pub type LeftBracketItemsBeforeItem = ThenItemsAfterArrayStartBeforeItem<LeftBracket>;

type LeftBrace = ThenLeftBrace<Init>;

pub type LeftBraceKvsBeforeKv = ThenKvsAfterObjectStartBeforeKv<LeftBrace>;

type TopLevelInString = ThenDoubleQuote<Init>;
pub const TOP_LEVEL_IN_STRING: State = TopLevelInString::STATE;
