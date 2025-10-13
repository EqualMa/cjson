use crate::ser::{ToJson, texts};

use super::Number;

impl ToJson for Number<str> {
    type ToJson<'a>
        = texts::Number<&'a str>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        texts::Number::new_without_validation(&self.0)
    }
}

impl Number<str> {
    pub const fn new_str_checked(s: &str) -> Option<&Self> {
        if validate_json_number(s.as_bytes()) {
            Some(Number::ref_cast_without_validation(s))
        } else {
            None
        }
    }
}
impl Number<[u8]> {
    pub const fn new_bytes_checked(s: &[u8]) -> Option<&Self> {
        if validate_json_number(s) {
            Some(Number::ref_cast_without_validation(s))
        } else {
            None
        }
    }
}

const fn validate_json_number(bytes: &[u8]) -> bool {
    let after_minus = match bytes.split_first() {
        Some((&b'-', rest)) => rest.split_first(),
        v => v,
    };

    let Some(after_minus) = after_minus else {
        return false;
    };

    let after_int = match after_minus {
        (&b'0', after_int) => after_int.split_first(),
        (&(b'1'..=b'9'), rest) => consume_any_digits(rest),
        _ => return false,
    };

    let after_frac = match after_int {
        Some((&b'.', rest)) => match consume_one_or_more_digits(rest) {
            Ok(v) => v,
            Err(_) => return false,
        },
        Some(v) => return validate_exp(v),
        None => return true,
    };

    match after_frac {
        Some(v) => validate_exp(v),
        None => return true,
    }
}

const fn validate_exp(input: (&u8, &[u8])) -> bool {
    match input {
        (&(b'e' | b'E'), rest) => {
            let after_optional_sign = match rest.split_first() {
                Some((&(b'-' | b'+'), after_sign)) => after_sign.split_first(),
                v => v,
            };

            let Some(after_optional_sign) = after_optional_sign else {
                return false;
            };

            return match consume_one_or_more_digits_from_split(after_optional_sign) {
                Ok(None) => true,
                _ => false,
            };
        }
        _ => return false,
    }
}

const fn consume_one_or_more_digits(input: &[u8]) -> Result<Option<(&u8, &[u8])>, ()> {
    match input.split_first() {
        Some((&(b'0'..=b'9'), rest)) => Ok(consume_any_digits(rest)),
        _ => Err(()),
    }
}

const fn consume_one_or_more_digits_from_split<'a>(
    input: (&u8, &'a [u8]),
) -> Result<Option<(&'a u8, &'a [u8])>, ()> {
    match input {
        (&(b'0'..=b'9'), rest) => Ok(consume_any_digits(rest)),
        _ => Err(()),
    }
}

const fn consume_any_digits(mut input: &[u8]) -> Option<(&u8, &[u8])> {
    loop {
        match input.split_first() {
            Some((&(b'0'..=b'9'), rest)) => {
                input = rest;
            }
            input => return input,
        }
    }
}
