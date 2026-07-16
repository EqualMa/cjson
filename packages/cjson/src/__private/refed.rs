use crate::ser::{IntoJson, ToJson2};

pub mod prelude_refed {}

enum Never {}
pub struct Refed<T>(Never, ::core::marker::PhantomData<T>);

impl<T: ToJson2> IntoJson for Refed<T> {
    type JsonKind = T::ToJsonKind;

    fn json_provide_into<
        W: crate::ser::ConsumeJson<
                ConsumeJsonKind: crate::ser::json_kinds::JsonKind<Contains<Self::JsonKind> = ()>,
            >,
    >(
        self,
        _: W,
    ) -> crate::ser::Consumed<Self::JsonKind, W> {
        match self.0 {}
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = <T as ToJson2>::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}

macro_rules! __private_json_refed {
        ($Ty:ty) => {
            Refed<$Ty>
        };
    }
