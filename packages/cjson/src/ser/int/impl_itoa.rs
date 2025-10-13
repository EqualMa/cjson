use arrayvec::ArrayString;

pub(super) use itoa::Integer;

pub(super) fn int_to_string<const CAP: usize>(v: &impl Integer) -> ArrayString<CAP> {
    int_to_string_impl(*v)
}

fn int_to_string_impl<const CAP: usize>(v: impl Integer) -> ArrayString<CAP> {
    let mut buf = itoa::Buffer::new();
    let s = buf.format(v);

    let ret = ArrayString::from(s).unwrap();

    ret
}
