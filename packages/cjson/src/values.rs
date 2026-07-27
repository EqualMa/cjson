use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::ser::{
    IntoJson, IntoJsonKeyColonValue, ToJsonByCopyIntoJson, helpers::json_fns, json_kinds,
};

#[derive(Debug, Clone, Copy)]
pub enum Never {}

mod never;

mod option;

#[derive(Debug, Clone, Copy)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

mod either;

#[derive(Debug, Clone, Copy)]
pub struct Null;

mod null_const;

#[derive(Debug, Clone, Copy)]
pub struct False;

#[derive(Debug, Clone, Copy)]
pub struct True;

crate::utils::impl_many!({
    {
        {
            use Null as JsonLiteral;
        }
        {
            use False as JsonLiteral;
        }
        {
            use True as JsonLiteral;
        }
    }

    impl IntoJson for JsonLiteral {
        type JsonKind = json_kinds::AnyValue;
        json_fns!({
            json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
            |self, w| w.consume_any_value(Self::JSON_VALUE_STR, ())
        });

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
    }

    impl ToJsonByCopyIntoJson for JsonLiteral {}
});

#[derive(Debug, Clone, Copy, RefCastCustom)]
#[repr(transparent)]
pub struct Number<T: ?Sized>(T);

impl<T> Number<T> {
    pub(crate) const fn new_without_validation(s: T) -> Self {
        Self(s)
    }
}
impl<T: ?Sized> Number<T> {
    #[ref_cast_custom]
    pub(crate) const fn ref_cast_without_validation(s: &T) -> &Self;
}

mod number;

#[derive(Debug, Clone, Copy)]
pub struct Finite<T>(T);

impl Finite<f64> {
    pub const fn new_f64(v: f64) -> Option<Self> {
        if v.is_finite() { Some(Self(v)) } else { None }
    }
}

impl Finite<f32> {
    pub const fn new_f32(v: f32) -> Option<Self> {
        if v.is_finite() { Some(Self(v)) } else { None }
    }
}

mod float;

#[derive(Debug, Clone, Copy)]
pub struct ChainString<A, B>(pub A, pub B);
#[derive(Debug, Clone, Copy)]
pub struct ChainArray<A, B>(pub A, pub B);
#[derive(Debug, Clone, Copy)]
pub struct ChainObject<A, B>(pub A, pub B);

mod chain;

#[derive(Debug, Clone, Copy)]
pub struct ArrayOfIter<I: IntoIterator<Item: IntoJson>>(pub I);
#[derive(Debug, Clone, Copy)]
pub struct ObjectOfIter<I: IntoIterator<Item: IntoJsonKeyColonValue>>(pub I);

impl<I: IntoIterator<Item: IntoJson>> IntoJson for ArrayOfIter<I> {
    type JsonKind = json_kinds::Array;

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        |self, w| w.consume_array_of_items(self.0, ())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

impl<I: IntoIterator<Item: IntoJsonKeyColonValue>> IntoJson for ObjectOfIter<I> {
    type JsonKind = json_kinds::Object;

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        |self, w| w.consume_object_of_iter(self.0, ())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}
