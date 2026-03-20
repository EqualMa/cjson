#![no_std]
#![deny(clippy::missing_safety_doc)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod ser;
pub mod values;

pub use ::cjson_proc_macro::ToJson;
pub use ser::ToJson;

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
mod macros_impl_to_json;
#[cfg(feature = "proc-macro")]
mod macros_proc_macro_to_json;

#[doc(hidden)]
pub mod __private;

#[cfg(feature = "proc-macro")]
pub mod proc_macro;
