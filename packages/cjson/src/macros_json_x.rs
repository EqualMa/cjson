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
            $json_x
            {} // json_x parsed options
            $($($on_parsed_append)*)?
        }
    };
}
