use std::marker::PhantomData;

use cjson::into_json;

enum Never {}

struct Val<'a>(PhantomData<&'a ()>, Never);

impl cjson::ser::IntoJson for Never {
    into_json!(|self| match self {});
}

impl cjson::ser::ToJson2 for Val<'_> {
    type ToJsonKind = cjson::ser::json_kinds::AnyValue;

    // TODO: refactor with json_provide_to!(|self| match self.1 {});
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
    fn json_provide_to_try<
        W: cjson::ser::TryConsumeJson<
                ConsumeJsonKind: cjson::ser::json_kinds::JsonKind<Contains<Self::ToJsonKind> = ()>,
            >,
    >(
        &self,
        _: W,
    ) -> Result<
        cjson::ser::Consumed<Self::ToJsonKind, W>,
        <W::Writer as cjson::ser::traits::TryConsumeTextChunk>::Err,
    > {
        match self.1 {}
    }
    async fn json_provide_to_async_try<
        W: cjson::ser::AsyncTryConsumeJson<
                ConsumeJsonKind: cjson::ser::json_kinds::JsonKind<Contains<Self::ToJsonKind> = ()>,
            >,
    >(
        &self,
        _: W,
    ) -> Result<
        cjson::ser::Consumed<Self::ToJsonKind, W>,
        <W::Writer as cjson::ser::traits::AsyncTryConsumeTextChunk>::Err,
    > {
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
    fn json_provide_into_try<
        W: cjson::ser::TryConsumeJson<
                ConsumeJsonKind: cjson::ser::json_kinds::JsonKind<Contains<Self::JsonKind> = ()>,
            >,
    >(
        self,
        _: W,
    ) -> Result<
        cjson::ser::Consumed<Self::JsonKind, W>,
        <W::Writer as cjson::ser::traits::TryConsumeTextChunk>::Err,
    > {
        match self {}
    }
    async fn json_provide_into_async_try<
        W: cjson::ser::AsyncTryConsumeJson<
                ConsumeJsonKind: cjson::ser::json_kinds::JsonKind<Contains<Self::JsonKind> = ()>,
            >,
    >(
        self,
        _: W,
    ) -> Result<
        cjson::ser::Consumed<Self::JsonKind, W>,
        <W::Writer as cjson::ser::traits::AsyncTryConsumeTextChunk>::Err,
    > {
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
