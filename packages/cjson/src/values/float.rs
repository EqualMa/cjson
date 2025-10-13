use arrayvec::ArrayString;

use crate::{
    ser::{ToJson, texts},
    utils::impl_many,
};

use super::Finite;

const SIZE: usize = core::mem::size_of::<ryu::Buffer>();

impl_many!(
    impl<__> ToJson for each_of![Finite<f64>, Finite<f32>] {
        type ToJson<'a>
            = texts::Number<ArrayString<SIZE>>
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            let mut buf = ryu::Buffer::new();
            let s = buf.format_finite(self.0);

            let ret = ArrayString::from(s).unwrap();
            texts::Number::new_without_validation(ret)
        }
    }
);
