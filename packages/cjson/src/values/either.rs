use crate::{
    ser::{IntoJson, ToJson2 as ToJson, helpers::json_fns, json_kinds::JsonKind},
    utils::impl_many,
};

impl_many!({
    {
        {
            use super::Either;
            macro_rules! EitherA { [$($t:tt)*] => [Either::A($($t)*)] }
            macro_rules! EitherB { [$($t:tt)*] => [Either::B($($t)*)] }
        }
        #[cfg(feature = "either")]
        {
            use ::either::Either;
            macro_rules! EitherA { [$($t:tt)*] => [Either::Left ($($t)*)] }
            macro_rules! EitherB { [$($t:tt)*] => [Either::Right($($t)*)] }
        }
    }

    impl<A: IntoJson, B: IntoJson> IntoJson for Either<A, B> {
        type JsonKind = <A::JsonKind as JsonKind>::Union<B::JsonKind>;
        json_fns!({
            json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
            use trait_mod;
            async |self, w| {
                use trait_mod::{HELP_ANCESTOR_TO_CONSUME_CHILD, await_};
                match self {
                    EitherA!(this) => await_!(<Self::JsonKind as HELP_ANCESTOR_TO_CONSUME_CHILD<
                        A::JsonKind,
                    >>::help_ancestor_to_consume_child(
                        w, this
                    )),
                    EitherB!(this) => await_!(<Self::JsonKind as HELP_ANCESTOR_TO_CONSUME_CHILD<
                        B::JsonKind,
                    >>::help_ancestor_to_consume_child(
                        w, this
                    )),
                }
            }
        });
        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
            A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
    }

    impl<A: ToJson, B: ToJson> ToJson for Either<A, B> {
        type ToJsonKind = <A::ToJsonKind as JsonKind>::Union<B::ToJsonKind>;
        json_fns!({
            json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
            use trait_mod;
            async |&self, w| {
                use trait_mod::{HELP_ANCESTOR_TO_CONSUME_CHILD, await_};
                match self {
                    EitherA!(this) => {
                        await_!(<Self::ToJsonKind as HELP_ANCESTOR_TO_CONSUME_CHILD<
                            A::ToJsonKind,
                        >>::help_ancestor_to_consume_child(
                            w, this
                        ))
                    }
                    EitherB!(this) => {
                        await_!(<Self::ToJsonKind as HELP_ANCESTOR_TO_CONSUME_CHILD<
                            B::ToJsonKind,
                        >>::help_ancestor_to_consume_child(
                            w, this
                        ))
                    }
                }
            }
        });

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
            A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
    }
});
