use core::fmt;

/// only store first N bytes
struct Buf<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Buf<N> {
    fn write_bytes(&mut self, s: &[u8]) {
        if s.is_empty() {
            return;
        }

        'push: {
            let rest = match self.buf.split_at_mut_checked(self.len) {
                Some((_, rest)) if !rest.is_empty() => rest,
                _ => {
                    break 'push;
                }
            };

            let (push_to, push_from) = if rest.len() > s.len() {
                let (push_to, _) = rest.split_at_mut(s.len());
                (push_to, s)
            } else {
                let (push_from, _) = s.split_at(rest.len());
                (rest, push_from)
            };

            push_to.copy_from_slice(push_from);
        }

        self.len += s.len();
    }
}

impl<const N: usize> fmt::Write for Buf<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        () = self.write_bytes(s.as_bytes());
        Ok(())
    }
}

pub trait ByteArrayMatches<const N: usize> {
    fn byte_array_matches(ba: [u8; N]) -> bool;
}

pub trait MatchBytes<const CAP: usize> {
    type Output;
    fn match_bytes(ba: &[u8]) -> Self::Output;
}

fn display_into_buf<const N: usize>(v: impl fmt::Display) -> Buf<N> {
    let mut buf = const {
        Buf {
            buf: [0u8; N],
            len: 0,
        }
    };

    {
        use core::fmt::Write as _;
        write!(buf, "{}", v).unwrap();
    }

    buf
}

pub struct IdentToBuf<const N: usize>(Buf<N>);

impl<const N: usize> IdentToBuf<N> {
    pub fn as_slice(&self) -> &[u8] {
        self.0.buf.split_at(self.0.len).0
    }
}

pub fn _ident_to_buf<const N: usize>(ident: &impl fmt::Display) -> IdentToBuf<N> {
    let buf = display_into_buf(ident);

    assert!(buf.len <= N, "N is too small");

    IdentToBuf(buf)
}

pub fn _ident_matches_byte_array<T: ?Sized + ByteArrayMatches<N>, const N: usize>(
    v: impl fmt::Display,
) -> bool {
    let buf = display_into_buf(v);
    buf.len == N && T::byte_array_matches(buf.buf)
}

macro_rules! ident_matches {
    ($id:expr, $matches:expr) => {{
        enum ByteArray {}
        const BYTE_ARRAY_LEN: usize = $matches.len();
        const BYTE_ARRAY: [u8; BYTE_ARRAY_LEN] = *$matches;
        impl $crate::syn_generic::ident_eq::ByteArrayMatches<BYTE_ARRAY_LEN> for ByteArray {
            fn byte_array_matches(ba: [u8; BYTE_ARRAY_LEN]) -> bool {
                matches!(ba, BYTE_ARRAY)
            }
        }
        $crate::syn_generic::ident_eq::_ident_matches_byte_array::<ByteArray, BYTE_ARRAY_LEN>(&$id)
    }};
}

macro_rules! ident_match {
    (match $id:ident $match_body:tt) => {
        $crate::__ident_match_parse_pats! {
            (
                match ($id) $match_body
            )
            // literals
            ()
            // parsed
            []
            // rest
            $match_body
        }
    };
}

macro_rules! __ident_match_parse_pats {
    (
        (
            match ($id:expr)
            $match_body:tt
        )
        ($(($literals:literal))+)
        $parsed:tt
        {} // EOF
    ) => {
        match $crate::ident_eq::_ident_to_buf::<{
            $crate::ident_eq::max([
                $($literals.len(),)+
            ])
        }>(&$id).as_slice()
        $match_body
    };
    (
        $data:tt
        ($($literals:tt)*)
        $parsed:tt
        {
            $(|)? $($lit:literal)|+ =>
            $($after_fat_arrow:tt)*
        }
    ) => {
        $crate::__ident_match_after_fat_arrow! {
            {
                $data
                (
                    $($literals)*
                    $( ($lit) )+
                )
            }
            $parsed
            {
                pat[$($lit)|+]
                if[]
            }
            { $($after_fat_arrow)* }
        }
    };
    (
        $data:tt
        $literals:tt
        $parsed:tt
        {
            $p:pat $(if $condition:expr)? =>
            $($after_fat_arrow:tt)*
        }
    ) => {
        $crate::__ident_match_after_fat_arrow! {
            {
                $data
                $literals
            }
            $parsed
            {
                pat[$p]
                if[$($condition)?]
            }
            { $($after_fat_arrow)* }
        }
    };
}

macro_rules! __ident_match_after_fat_arrow {
    (
        {$($d:tt)*}
        [$($parsed:tt)*]
        { $($cur:tt)+ }
        { $v:expr $(, $($rest:tt)*)? }
    ) => {
        $crate::__ident_match_parse_pats! {
            $($d)*
            [
                $($parsed)*
                {
                    $($cur)+
                    v($v)
                }
            ]
            { $($($rest)*)? }
        }
    };
    (
        {$($d:tt)*}
        [$($parsed:tt)*]
        { $($cur:tt)+ }
        { $v:block $($rest:tt)* }
    ) => {
        $crate::__ident_match_parse_pats! {
            $($d)*
            [
                $($parsed)*
                {
                    $($cur)+
                    v($v)
                }
            ]
            { $($rest)* }
        }
    };
}

pub(crate) use {
    __ident_match_after_fat_arrow, __ident_match_parse_pats, ident_match, ident_matches,
};

pub const fn max<const N: usize>(nums: [usize; N]) -> usize {
    const { assert!(N > 0) }

    let mut i = 0;
    let mut max = 0;

    while i < N {
        if nums[i] > max {
            max = nums[i];
        }
        i += 1;
    }

    max
}
