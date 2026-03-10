use crate::r#const::array_string::ArrayString;
use crate::r#const::json_value_array_str::JsonValueArrayStr;
use crate::utils::impl_many;
use crate::{
    r#const::{ConstIntoJson, ConstIntoJsonValueString},
    ser::texts,
};

use super::Finite;

const SIZE: usize = core::mem::size_of::<const_ryu::Buffer>();

struct FiniteNew<T>(T);

impl FiniteNew<f32> {
    const fn call_once(self) -> Option<Finite<f32>> {
        Finite::new_f32(self.0)
    }
}

impl FiniteNew<f64> {
    const fn call_once(self) -> Option<Finite<f64>> {
        Finite::new_f64(self.0)
    }
}

impl_many!(
    type Float = each_of![f32, f64];

    const _: () = {
        impl ConstIntoJson<Finite<Float>> {
            pub const fn const_into_json(self) -> Finite<Float> {
                self.0
            }
        }
        impl ConstIntoJson<Float> {
            pub const fn const_into_json(self) -> Finite<Float> {
                match FiniteNew(self.0).call_once() {
                    Some(f) => f,
                    None => panic!("non-finite float cannot be serialized as json"),
                }
            }
        }

        impl ConstIntoJsonValueString<Finite<Float>> {
            pub const fn const_into_json_value_string_len(self) -> usize {
                self.const_into_json_value_string_without_const_len()
                    .inner()
                    .len()
            }

            pub const fn const_into_json_value_string<const LEN: usize>(
                self,
            ) -> JsonValueArrayStr<LEN> {
                let s = self.const_into_json_value_string_without_const_len();
                assert!(s.inner().len() == LEN);
                let mut bytes = [0u8; LEN];
                bytes.copy_from_slice(s.inner().as_bytes());
                JsonValueArrayStr::new_without_validation(bytes)
            }

            pub const fn const_into_json_value_string_without_const_len(
                self,
            ) -> texts::Number<ArrayString<u8, SIZE>> {
                let mut buf = const_ryu::Buffer::new();
                let s = const_ryu::FormatFinite(&mut buf, self.0.0).call_once();

                let res = ArrayString::from_str(s);

                texts::Number::new_without_validation(res)
            }

            pub const fn const_concat_after_stated_chunk_buf<const CAP: usize>(
                self,
                chunk_buf: crate::r#const::StatedChunkBuf<CAP>,
            ) -> crate::r#const::StatedChunkBuf<CAP> {
                let mut buf = const_ryu::Buffer::new();
                let s = const_ryu::FormatFinite(&mut buf, self.0.0).call_once();

                chunk_buf.json_value(texts::Value::new_without_validation(s))
            }
        }

        assert!(SIZE <= u8::MAX as usize);
    };
);
