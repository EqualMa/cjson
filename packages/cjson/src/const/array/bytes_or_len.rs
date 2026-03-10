pub(super) struct BytesOrLen<const LEN: usize> {
    bytes: [u8; LEN],
    len: usize,
}

impl<const LEN: usize> BytesOrLen<LEN> {
    pub(super) const fn push_str(&mut self, s: &str) {
        if LEN > 0 {
            self.bytes
                .split_at_mut(self.len)
                .1
                .split_at_mut(s.len())
                .0
                .copy_from_slice(s.as_bytes());
        }
        self.len += s.len();
    }

    pub(super) const fn len(&self) -> usize {
        self.len
    }

    pub(super) const fn as_str(&self) -> &str {
        // this fn is only expected to be called in const eval so safety is preferred
        match str::from_utf8(self.as_bytes()) {
            Ok(v) => v,
            Err(_) => panic!(),
        }
    }

    pub(super) const fn as_bytes(&self) -> &[u8] {
        const { Self::REQUIRE_BYTES }
        self.bytes.split_at(self.len).0
    }

    const REQUIRE_BYTES: () = const {
        if LEN == 0 {
            panic!("bytes is not available")
        }
    };
}
