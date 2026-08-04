#[macro_export]
macro_rules! __private_json_x {
    (
        [$json_x:tt $(())?]
        $data:tt
    ) => {
        $crate::__private_json_x_expand! {
            {
                macro($json_x)
                any_value()
            }
            {$}
            $data
        }
    };
    (
        [$json_x:tt $json_x_options:tt]
        $data:tt
    ) => {
        $crate::__private_json_x_parse_options! {
            $json_x_options $json_x_options
            {
                macro {}
                any_value()
            }
            {
                $json_x
                $data
            }
        }
    };
}

#[macro_export]
macro_rules! __private_json_x_parse_options {
    (
        ()()
        {
            macro {}
            any_value $any_value:tt
        }
        {
            $json_x:tt
            $data:tt
        }
    ) => {
        $crate::__private_json_x_expand! {
            {
                macro($json_x)
                any_value $any_value
            }
            {$}
            $data
        }
    };
    (
        ()()
        {
            macro { $macro:tt }
            any_value $any_value:tt
        }
        {
            $json_x:tt
            $data:tt
        }
    ) => {
        $crate::__private_json_x_expand! {
            {
                macro $macro
                any_value $any_value
            }
            {$}
            $data
        }
    };
    (
        ( macro ($($json_x_macro_name:ident)?)        $(, $($_rest:tt)*)? )
        ( macro $macro:tt                             $(, $( $rest:tt)*)? )
        {
            macro {}
            any_value $any_value:tt
        }
        $data:tt
    ) => {
        $crate::__private_json_x_parse_options! {
            ($($($_rest)*)?)
            ($($( $rest)*)?)
            {
                macro { $macro }
                any_value $any_value
            }
            $data
        }
    };
    (
        ( any_value     $(, $($_rest:tt)*)? )
        ( $any_value:tt $(, $( $rest:tt)*)? )
        {
            macro $macro:tt
            any_value()
        }
        $data:tt
    ) => {
        $crate::__private_json_x_parse_options! {
            ($($($_rest)*)?)
            ($($( $rest)*)?)
            {
                macro $macro
                any_value($any_value)
            }
            $data
        }
    };
}

#[macro_export]
macro_rules! __private_json_x_expand {
    (
        // parsed options
        {
            macro($($json_x_macro_name:tt)?)
            any_value()
        }
        {$_:tt} // $
        {
            $maybe_try:tt
            ($consumer:expr)
            ($attr_expr:expr)
        }
    ) => {{
        let __cjson_consumer = $consumer;
        $(macro_rules! $json_x_macro_name {
            ($_($json_comma:tt)+) => {
                $crate::__private_json_write! {
                    $maybe_try
                    (__cjson_consumer)
                    $_($json_comma)+
                }
            };
        })?
        $attr_expr
    }};
    (
        // parsed options
        {
            macro($json_x_macro_name:tt)
            any_value($any_value:ident)
        }
        {$_:tt} // $
        {
            {$async_try_mod:ident $($async_try_postfix:tt)*}
            ($consumer:expr)
            ($attr_expr:expr)
        }
    ) => {{
        use $crate::macro_helpers::json_x::$any_value as _;
        let __cjson_consumer = <_ as $crate::__private::macro_used_names::$async_try_mod::CONSUME_JSON>::into_consume_json_text($consumer, ());
        macro_rules! $json_x_macro_name {
            ($_($json_comma:tt)+) => {
                $crate::macro_helpers::json_x::$any_value(
                    $crate::__private_json_write! {
                        {$async_try_mod $($async_try_postfix)*}
                        (__cjson_consumer)
                        $_($json_comma)+
                    }
                )
            };
        }
        $attr_expr
    }};
}
