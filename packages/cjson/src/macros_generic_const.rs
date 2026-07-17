///
/// ```no_compile
/// json_value_generic_const!($into_json:expr, $capacity:expr $(,)?)
/// json_value_generic_const!($into_json:expr                 $(,)?)
/// ```
#[macro_export]
macro_rules! __private_json_well_known_macro_json_value_generic_const {
    (
        $json_value_generic_const_body:tt
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime $prev
                last_compile_time[
                    $($current_compile_time)*
                    json_value_generic_const $json_value_generic_const_body
                ]
            ]
            after_value $after_value
        }
    };
}
