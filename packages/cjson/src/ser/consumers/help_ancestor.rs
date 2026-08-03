use crate::ser::IntoJson;

use super::{
    Consumed,
    json_kinds::{self, JsonKind},
};

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_JSON, Output, XHelpers as _};

        impl<KChild: JsonKind> HELP_ANCESTOR_TO_CONSUME_CHILD<KChild> for json_kinds::AnyValue {
            fn help_ancestor_to_consume_child<
                W: CONSUME_JSON<ConsumeJsonKind: JsonKind<Contains<Self> = ()>>,
            >(
                consumer: W,
                value: impl IntoJson<JsonKind = KChild>,
            ) -> Output![Consumed<Self, W>, W::Writer] {
                <W as CONSUME_JSON>::consume_any_value_of_any_kind(consumer, value, ())
            }
        }

        impl<K: HelpAncestorToXConsumeSelf> HELP_ANCESTOR_TO_CONSUME_CHILD<K> for K {
            fn help_ancestor_to_consume_child<
                W: CONSUME_JSON<ConsumeJsonKind: JsonKind<Contains<Self> = ()>>,
            >(
                consumer: W,
                value: impl crate::ser::IntoJson<JsonKind = K>,
            ) -> Output![Consumed<Self, W>, W::Writer] {
                value.json_provide_into_x(consumer)
            }
        }
    }

    mod base {
        pub trait HelpAncestorToConsumeChild<KChild: JsonKind>:
            JsonKind<Contains<KChild> = ()>
        {
        }

        use HelpAncestorToConsumeChild as HELP_ANCESTOR_TO_CONSUME_CHILD;
    }
    mod try_ {
        pub trait HelpAncestorToTryConsumeChild<KChild: JsonKind>:
            JsonKind<Contains<KChild> = ()>
        {
        }

        use HelpAncestorToTryConsumeChild as HELP_ANCESTOR_TO_CONSUME_CHILD;
    }
    mod async_try {
        pub trait HelpAncestorToAsyncTryConsumeChild<KChild: JsonKind>:
            JsonKind<Contains<KChild> = ()>
        {
        }

        use HelpAncestorToAsyncTryConsumeChild as HELP_ANCESTOR_TO_CONSUME_CHILD;
    }

    fn help_ancestor_to_consume_child<
        W: CONSUME_JSON<ConsumeJsonKind: JsonKind<Contains<Self> = ()>>,
    >(
        consumer: W,
        value: impl crate::ser::IntoJson<JsonKind = KChild>,
    ) -> Output![Consumed<Self, W>, W::Writer];
});

pub trait HelpAncestorToXConsumeSelf: JsonKind {}

impl HelpAncestorToXConsumeSelf for json_kinds::JsonString {}
impl HelpAncestorToXConsumeSelf for json_kinds::Array {}
impl HelpAncestorToXConsumeSelf for json_kinds::Object {}

pub trait HelpAncestorToXConsumeChild<KChild: JsonKind>:
    HelpAncestorToConsumeChild<KChild>
    + HelpAncestorToTryConsumeChild<KChild>
    + HelpAncestorToAsyncTryConsumeChild<KChild>
{
}

impl<
    K: HelpAncestorToConsumeChild<KChild>
        + HelpAncestorToTryConsumeChild<KChild>
        + HelpAncestorToAsyncTryConsumeChild<KChild>,
    KChild: JsonKind,
> HelpAncestorToXConsumeChild<KChild> for K
{
}
