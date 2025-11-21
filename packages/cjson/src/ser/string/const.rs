use crate::r#const::json_value_array_str::JsonValueArrayStr;
use crate::r#const::{ConstIntoJson, ConstIntoJsonValueString};
use crate::ser::texts::StrToJsonStringFragment;

impl<'a> ConstIntoJson<&'a str> {
    pub const fn const_into_json(self) -> &'a str {
        self.0
    }
}

impl ConstIntoJsonValueString<&str> {
    pub const fn const_into_json_value_string_len(self) -> usize {
        let mut len = 0usize;

        let mut chunks = StrToJsonStringFragment(self.0).const_into_text_chunks();

        while let Some(chunk) = chunks.next_text_chunk() {
            len += chunk.len();
        }

        len + 2 // two double-quotes
    }

    pub const fn const_into_json_value_string<const LEN: usize>(self) -> JsonValueArrayStr<LEN> {
        let mut bytes = [0u8; LEN];

        {
            bytes[0] = b'"';
            let mut bytes = bytes.split_at_mut(1).1;

            let mut chunks = StrToJsonStringFragment(self.0).const_into_text_chunks();

            while let Some(chunk) = chunks.next_text_chunk() {
                let cur;
                (cur, bytes) = bytes.split_at_mut(chunk.len());
                cur.copy_from_slice(chunk);
            }

            assert!(bytes.len() == 1);

            bytes[0] = b'"';
        }

        JsonValueArrayStr::new_without_validation(bytes)
    }
}

#[cfg(test)]
const _: () = {
    // %x22 /          ; "    quotation mark  U+0022
    // %x5C /          ; \    reverse solidus U+005C
    // %x2F /          ; /    solidus         U+002F
    // %x62 /          ; b    backspace       U+0008
    // %x66 /          ; f    form feed       U+000C
    // %x6E /          ; n    line feed       U+000A
    // %x72 /          ; r    carriage return U+000D
    // %x74 /          ; t    tab             U+0009
    // %x75 4HEXDIG )  ; uXXXX                U+XXXX
    const S: &str = "\x22\x5C\x2F\x08\x0C\x0A\x0D\x09\x00";
    const LEN: usize = ConstIntoJsonValueString(S).const_into_json_value_string_len();

    assert!(LEN == 23);

    const JSON: &[u8] = ConstIntoJsonValueString(S)
        .const_into_json_value_string::<LEN>()
        .as_bytes();

    assert!(matches!(JSON, br#""\"\\/\b\f\n\r\t\u0000""#));
};
