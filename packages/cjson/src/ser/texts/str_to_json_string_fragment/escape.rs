//! https://docs.rs/serde_json/1.0.145/src/serde_json/ser.rs.html#2079-2165

pub(super) const fn needs_escape(x: &u8) -> bool {
    match x {
        ..=0x1F | 0x22 | 0x5C => true,
        _ => false,
    }
}

/// # Safety
///
/// `byte <= 0x5C` must be true.
///
/// [`needs_escape(byte)`](needs_escape) is sufficient.
pub unsafe fn escape_to_bytes_unchecked(byte: u8) -> &'static [u8] {
    debug_assert!((byte as usize) < ESCAPE_BYTES.len());

    let ret = *(
        // SAFETY: byte <= 0x5C
        unsafe { ESCAPE_BYTES.get_unchecked(byte as usize) }
    );

    debug_assert!(!ret.is_empty());

    ret
}

const MAX_ESCAPE_CHAR_PLUS_1: u8 = 0x5C + 1;

const ESCAPE_BYTES: [&[u8]; MAX_ESCAPE_CHAR_PLUS_1 as usize] = {
    const EMPTY: &[u8] = &[];
    let mut ret = [EMPTY; MAX_ESCAPE_CHAR_PLUS_1 as usize];

    let mut i = 0;

    while i < MAX_ESCAPE_CHAR_PLUS_1 {
        let escaped: &[u8] = match escape(i) {
            MaybeEscaped::Special(v) => v,
            MaybeEscaped::U(v) => v,
            MaybeEscaped::NotEscaped => EMPTY,
        };
        debug_assert!(core::str::from_utf8(escaped).is_ok());
        ret[i as usize] = escaped;
        i += 1;
    }

    ret
};

enum MaybeEscaped {
    Special(&'static [u8; 2]),
    U(&'static [u8; 6]),
    NotEscaped,
}

const fn escape(byte: u8) -> MaybeEscaped {
    MaybeEscaped::Special(match byte {
        0x08 => b"\\b",
        0x09 => b"\\t",
        0x0A => b"\\n",
        0x0C => b"\\f",
        0x0D => b"\\r",
        0x22 => b"\\\"",
        0x5C => b"\\\\",
        ..=0x1F => {
            return MaybeEscaped::U(&ESCAPE_U[byte as usize]);
        }
        _ => return MaybeEscaped::NotEscaped,
    })
}

const MAX_U_PLUS_1: u8 = 0x1F + 1;
const ESCAPE_U: [[u8; 6]; MAX_U_PLUS_1 as usize] = {
    let mut ret = [[0u8; 6]; MAX_U_PLUS_1 as usize];
    let mut byte = 0u8;

    while byte < MAX_U_PLUS_1 {
        const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
        ret[byte as usize] = [
            b'\\',
            b'u',
            b'0',
            b'0',
            HEX_DIGITS[(byte >> 4) as usize],
            HEX_DIGITS[(byte & 0xF) as usize],
        ];

        byte += 1;
    }

    ret
};
