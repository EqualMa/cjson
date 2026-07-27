#![no_std]
#![deny(clippy::missing_safety_doc)]
#![cfg_attr(feature = "write_all_vectored", feature(write_all_vectored))]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod ser;
pub mod values;

pub use ::cjson_proc_macro::ToJson;
pub use ser::ToJson;

#[macro_export]
macro_rules! json_to {
    ($($json:tt)+) => {{
        let mut s = $crate::__private::Default::default();
        let w = $crate::ser::ConsumeJsonText(
            <_ as $crate::ser::traits::ConsumeTextChunk>::as_mut_consume_text_chunk(&mut s)
        );
        let $crate::ser::Consumed { .. } = $crate::json_write! { w, $($json)+ };
        s
    }};
}

#[macro_export]
macro_rules! json_to_try {
    ($($json:tt)+) => {{
        let mut s = $crate::__private::Default::default();
        let w = $crate::ser::ConsumeJsonText(
            <_ as $crate::ser::traits::TryConsumeTextChunk>::as_mut_try_consume_text_chunk(&mut s)
        );
        let $crate::ser::Consumed { .. } = $crate::json_write_try! { w, $($json)+ };
        s
    }};
}

#[macro_export]
macro_rules! json_to_async_try {
    ($($json:tt)+) => {{
        let mut s = $crate::__private::Default::default();
        let w = $crate::ser::ConsumeJsonText(
            <_ as $crate::ser::traits::AsyncTryConsumeTextChunk>::as_mut_async_try_consume_text_chunk(&mut s)
        );
        let $crate::ser::Consumed { .. } = $crate::json_write_async_try! { w, $($json)+ };
        s
    }};
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! json_to_string {
    ($($json:tt)+) => {{
        let s: $crate::__private::String = $crate::json_to!($($json)+);
        s
    }};
}

mod never_future;
mod utils;
/*
macro_rules! json_string {
    () => {};
}

const _: () = {
    json_string!(runtime!(String::from("")));

    json_string!("" + const {} + runtime! { String::from("") });
};

macro_rules! json {
    (null) => {
        $crate::values::Null
    };
    // (false) => {
    //     $crate::values::False
    // };
    // (true) => {
    //     $crate::values::True
    // };
    ($lit:literal) => {
        const { RustLiteral::into_json::<{ RustLiteral::json_len($lit) }>($lit) }
    };
    (const $const_block:block) => {
        const { RustConst($const_block).into_json() }
    };
}

json! {false}
 */
// mod macros;

// mod const_json;

pub mod r#const;
mod macros;
mod macros_generic_const;
mod macros_impl_json;
mod macros_impl_to_json;
mod macros_write;

pub mod macro_helpers;

#[doc(hidden)]
pub mod __private;

#[cfg(feature = "proc-macro")]
pub mod proc_macro;
