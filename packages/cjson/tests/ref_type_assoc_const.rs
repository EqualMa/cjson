use std::marker::PhantomData;

enum Never {}

struct Val<'a>(PhantomData<&'a ()>, Never);

impl cjson::ser::IntoJson for Never {
    type JsonKind = cjson::ser::json_kinds::AnyValue;
    fn json_provide_into<
        W: cjson::ser::ConsumeJson<
                ConsumeJsonKind: cjson::ser::json_kinds::JsonKind<Contains<Self::JsonKind> = ()>,
            >,
    >(
        self,
        _: W,
    ) -> cjson::ser::Consumed<Self::JsonKind, W> {
        match self {}
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
        <&str as ::cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}

impl cjson::ser::ToJson2 for Val<'_> {
    type ToJsonKind = cjson::ser::json_kinds::AnyValue;
    fn json_provide_to<
        W: cjson::ser::ConsumeJson<
                ConsumeJsonKind: cjson::ser::json_kinds::JsonKind<Contains<Self::ToJsonKind> = ()>,
            >,
    >(
        &self,
        _: W,
    ) -> cjson::ser::Consumed<Self::ToJsonKind, W> {
        match self.1 {}
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = true;
}

enum Never2 {}

impl cjson::ser::IntoJson for Never2 {
    type JsonKind = cjson::ser::json_kinds::AnyValue;
    fn json_provide_into<
        W: cjson::ser::ConsumeJson<
                ConsumeJsonKind: cjson::ser::json_kinds::JsonKind<Contains<Self::JsonKind> = ()>,
            >,
    >(
        self,
        _: W,
    ) -> cjson::ser::Consumed<Self::JsonKind, W> {
        match self {}
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
        <&Val<'_> as ::cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}

const _: () = {
    assert!(!<Never as cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY);
    assert!(<&Val<'_> as cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY);
    assert!(<Never2 as cjson::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY);
};

#[test]
fn compile_only() {}
