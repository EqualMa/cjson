use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::ser::ToJson;

#[derive(Debug, Clone, Copy)]
pub enum Never {}

mod never;

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

crate::utils::impl_many!(
    impl<__> ToJson for each_of![Null, False, True] {
        type ToJson<'a>
            = Self
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            *self
        }
    }
);

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
