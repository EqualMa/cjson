#[macro_export]
macro_rules! __private_json_x {
    (
        [$json_x:tt $(())?]
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            // json_x parsed options
            $json_x
            {
                macro($json_x)
            }
            $($($on_parsed_append)*)?
        }
    };
    (
        [$json_x:tt (macro($($json_x_macro_name:tt)?))]
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            // json_x parsed options
            $json_x
            {
                macro($($json_x_macro_name)?)
            }
            $($($on_parsed_append)*)?
        }
    };
}
