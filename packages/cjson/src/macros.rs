#[macro_export]
#[doc(hidden)]
macro_rules! __expand_or {
    ([         ][$($or:tt)*]) => ($($or)*);
    ([$($e:tt)+][$($or:tt)*]) => ($($e )+);
}

#[macro_export]
macro_rules! json {
    ($lit:literal) => {
        $crate::json!(const { $lit })
    };
    (const $const_block:block) => {
        const {
            enum HasConstJsonValue {}

            impl $crate::r#const::HasConstJsonValue for HasConstJsonValue {
                const JSON_VALUE: $crate::ser::texts::Value<&'static $crate::__private::str> = {
                    $crate::r#const::ConstAsJsonValueStr(
                        $crate::r#const::ConstIntoJsonValueString(
                            $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                        )
                        .const_into_json_value_string::<{
                            $crate::r#const::ConstIntoJsonValueString(
                                $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                            )
                            .const_into_json_value_string_len()
                        }>(),
                    )
                    .const_as_json_value_str()
                };
            }

            $crate::r#const::ConstJsonValue::<HasConstJsonValue>::new()
        }
    };
    (
        $(const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ; $(,)?)+
        const $const_block:block
    ) => {
        const {
            enum HasConstJsonValue
                <$(const $CONST: $ConstTy),+>
                {}

            impl
                <$(const $CONST: $ConstTy),+>
                $crate::r#const::HasConstJsonValue for HasConstJsonValue
                <$(      $CONST          ),+>
            {
                const JSON_VALUE: $crate::ser::texts::Value<&'static $crate::__private::str> = {
                    $crate::r#const::ConstAsJsonValueStr(
                        $crate::r#const::ConstIntoJsonValueString(
                            $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                        )
                        .const_into_json_value_string_without_const_len()
                        // TODO: rust limitation: generic parameters may not be used in const operations
                        // .const_into_json_value_string::<LEN>()
                    )
                    .const_as_json_value_str()
                };
            }

            $crate::r#const::ConstJsonValue::<HasConstJsonValue::
                <$({$crate::__private::__expand_or!([$($const_value)?][$CONST])}),+>
            >::new()
        }
    };
    ($well_known_ident:ident) => {
        $crate::json!(const { $crate::__private::well_known_ident::$well_known_ident })
    };
}

#[cfg(test)]
mod tests;
