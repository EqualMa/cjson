use crate::r#const::ConstAsJsonValueStr;
use crate::r#const::array_string::ArrayString;

use crate::r#const::json_value_array_str::JsonValueArrayStr;
use crate::utils::impl_many;
use crate::{
    r#const::{ConstIntoJson, ConstIntoJsonValueString},
    ser::texts,
};

impl_many!(
    type Int = each_of![
        i8, i16, i32, i64, isize, i128, //
        u8, u16, u32, u64, usize, u128,
    ];

    const _: () = {
        impl ConstIntoJson<Int> {
            pub const fn const_into_json(self) -> Int {
                self.0
            }
        }

        impl ConstIntoJsonValueString<Int> {
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
            ) -> texts::Number<ArrayString<u8, { <Int as const_itoa::Integer>::MAX_STR_LEN }>>
            {
                let mut buf = const_itoa::Buffer::new();
                let s = { const_itoa::Format(&mut buf, self.0).call_once() };

                let res = ArrayString::from_str(s);

                texts::Number::new_without_validation(res)
            }

            pub const fn const_concat_after_stated_chunk_buf<const CAP: usize>(
                self,
                chunk_buf: crate::r#const::StatedChunkBuf<CAP>,
            ) -> crate::r#const::StatedChunkBuf<CAP> {
                let mut buf = const_itoa::Buffer::new();
                let s = { const_itoa::Format(&mut buf, self.0).call_once() };

                let v = texts::Value::new_without_validation(s);

                chunk_buf.json_value(v)
            }
        }
    };
);
