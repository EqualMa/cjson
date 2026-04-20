use core::mem::MaybeUninit;

use itoa::Buffer;

pub(super) use itoa::Integer;

type ArrayString<const CAP: usize> = crate::r#const::array_string::ArrayString<u8, CAP>;

pub(super) fn int_to_string<const CAP: usize>(v: impl Integer) -> ArrayString<CAP> {
    int_to_string_impl(v)
}

fn int_to_string_impl<const CAP: usize>(v: impl Integer) -> ArrayString<CAP> {
    let mut buf = Buffer::new();
    let s = buf.format(v);

    let ret = ArrayString::from_str(s);

    ret
}

pub(super) fn format<const CAP: usize>(buf: &mut MaybeUninit<Buffer>, i: impl Integer) -> &str {
    buf.write(Buffer::new()).format(i)
}
