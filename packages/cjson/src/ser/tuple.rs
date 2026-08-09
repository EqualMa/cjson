use super::{IntoJson, ToJson2 as ToJson, helpers::json_fns, json_kinds};

macro_rules! impl_tuple {
    ($pre:tt) => {};
    (
        {$($pre:tt)*}
        $i:tt
        $($rest:tt)*
    ) => {
        impl_tuple_one! {
            $($pre)*
            $i
        }
        impl_tuple! {
            {
                $($pre)*
                $i
            }
            $($rest)*
        }
    };
}

macro_rules! unwrap_first {
    ({$($first:tt)*} $($rest:tt)*) => { $($first)* };
}

macro_rules! unwrap_non_first {
    ($first:tt $({$($rest:tt)*})*) => { $($($rest)*)* };
}

macro_rules! impl_tuple_one {
    () => {};
    ($(( $TN:tt $tn:tt ))+) => {
        impl<$($TN: ToJson,)+> ToJson for ($($TN,)+) {
            type ToJsonKind = json_kinds::Array;
            json_fns!({
                json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
                use trait_mod;
                async |&self, w| {
                    use trait_mod::{
                        await_try, await_,
                        READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY as _,
                        CONSUME_JSON_CHUNKS as _
                    };
                    let ($($tn,)+) = self;
                    let w = w.start_to_consume_chunks_of_non_empty_array(());
                    let w = await_try!(w.left_bracket_value(unwrap_first!($({$tn})+)));
                    unwrap_non_first! {$({
                        let w = await_try!(w.comma_json_value($tn));
                    })+}
                    await_!(w.end_with_right_bracket(()))
                }
            });
            const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
        }
        impl<$($TN: IntoJson,)+> IntoJson for ($($TN,)+) {
            type JsonKind = json_kinds::Array;
            json_fns!({
                json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
                use trait_mod;
                async |self, w| {
                    use trait_mod::{
                        await_try, await_,
                        READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY as _,
                        CONSUME_JSON_CHUNKS as _
                    };
                    let ($($tn,)+) = self;
                    let w = w.start_to_consume_chunks_of_non_empty_array(());
                    let w = await_try!(w.left_bracket_value(unwrap_first!($({$tn})+)));
                    unwrap_non_first! {$({
                        let w = await_try!(w.comma_json_value($tn));
                    })+}
                    await_!(w.end_with_right_bracket(()))
                }
            });
            const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
        }
    };
}

impl_tuple! {
    {}
    (T0 t0)
    (T1 t1)
    (T2 t2)
    (T3 t3)
    (T4 t4)
    (T5 t5)
    (T6 t6)
    (T7 t7)
    (T8 t8)
    (T9 t9)
    (T10 t10)
    (T11 t11)
}
