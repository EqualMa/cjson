//! Also see the compile_fail doc test in
//! [`::cjson::macro_helpers::impl_json_auto_ref::to_type::auto_ref`].

use std::marker::PhantomData;

use cjson::{json_fns, json_items};

enum Never {}

struct Val<'a>(PhantomData<&'a ()>, Never);

impl cjson::ser::IntoJson for Never {
    json_items!(|self| match self {});
}

impl cjson::ser::ToJson for Val<'_> {
    type ToJsonKind = cjson::ser::json_kinds::AnyValue;

    json_fns!(|&self| match (self.1) {});

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = true;
}

enum Never2 {}

impl cjson::ser::IntoJson for Never2 {
    type JsonKind = cjson::ser::json_kinds::AnyValue;

    json_fns!(|self| match self {});

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
        <&Val<'_> as ::cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}

struct ExpectBool<const B: bool>;

impl<const B: bool> ExpectBool<B> {
    const DEFAULT: Self = Self;
}

type ExpectTrue = ExpectBool<{ <&Val<'_> as cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY }>;

const _: () = {
    let self::ExpectBool::<true> = ExpectTrue::DEFAULT;
    assert!(!<Never as cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY);
    assert!(<&Val<'_> as cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY);
    assert!(<Never2 as cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY);
};

#[test]
fn compile_only() {}
