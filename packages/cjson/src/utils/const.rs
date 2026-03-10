pub(crate) struct Eq<A, B>(pub A, pub B);

impl Eq<&[u8], &[u8]> {
    pub(crate) const fn call_once(self) -> bool {
        let Self(a, b) = self;
        a.len() == b.len()
            && 'ret: {
                let mut i = 0;
                while i < a.len() {
                    if a[i] != b[i] {
                        break 'ret false;
                    }
                    i += 1;
                }

                true
            }
    }
}
