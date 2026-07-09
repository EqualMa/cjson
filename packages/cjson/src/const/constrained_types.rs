use core::{hint::unreachable_unchecked, marker::PhantomData};

use crate::utils::impl_many;

use super::{
    HasConstState, State, stated_str::StatedChunkStr, stated_str_as_array_vec::StatedChunkBuf,
    str_as_array::StrAsArray, str_as_array_vec::StrAsArrayVec,
};

macro_rules! PhantomData {
    ($($T:ty),* $(,)?) => {
        PhantomData::<($(PhantomData<$T>,)*)>
    };
}

macro_rules! define {
    (
        $AsArray:ident,
        $AsArrayVec:ident,
        $AsStr:ident,
        $(
            generics![$($generics:tt)*],
            generic_params![$($generic_params:tt)*],
        )?
        |$assert_v:pat_param| $assert:block,
    ) => {
        pub struct $AsArray<const LEN: usize, $($($generics)*)?>(StrAsArray<LEN> $(, PhantomData![$($generic_params)*])?);
        pub struct $AsArrayVec<const CAP: usize, $($($generics)*)?>(StrAsArrayVec<CAP> $(, PhantomData![$($generic_params)*])?);
        pub struct $AsStr<'a, $($($generics)*)?>(&'a str $(, PhantomData![$($generic_params)*])?);

        impl<const LEN: usize, $($($generics)*)?> $AsArray<LEN, $($($generic_params)*)?> {
            pub const fn from_array_vec(v: StatedChunkBuf<LEN>) -> Self {
                let v = v.assert();
                $AsStr::<$($($generic_params)*)?>::assert_valid(v.as_str());
                Self(v.into_inner() $(, PhantomData![$($generic_params)*])?)
            }
            pub const fn as_str(&self) -> $AsStr<'_, $($($generic_params)*)?> {
                $AsStr(self.0.as_str() $(, PhantomData![$($generic_params)*])?)
            }
        }

        impl<const CAP: usize, $($($generics)*)?> $AsArrayVec<CAP, $($($generic_params)*)?> {
            pub const fn from_array_vec(v: StatedChunkBuf<CAP>) -> Self {
                $AsStr::<$($($generic_params)*)?>::assert_valid(v.as_str());
                Self(v.into_inner() $(, PhantomData![$($generic_params)*])?)
            }
            pub const fn as_str(&self) -> $AsStr<'_, $($($generic_params)*)?> {
                $AsStr(self.0.as_str() $(, PhantomData![$($generic_params)*])?)
            }
        }

        impl<'a, $($($generics)*)?>
            $AsStr<'a, $($($generic_params)*)?> {
            pub const fn as_str(self) -> &'a str {
                self.0
            }

            const fn assert_valid($assert_v: StatedChunkStr<'a>)
                $assert
        }
    };
}

define!(
    // The chunk represents an non-empty json array;
    // The chunk is represented as a byte array.
    NonEmptyArrayAsArray,
    NonEmptyArrayAsArrayVec,
    NonEmptyArrayAsStr,
    |v| {
        let chunk = v.remove_surrounding_group();
        chunk
            .prev_state()
            .copied()
            .assert_is_top_level_after_array_start();
        chunk
            .next_state()
            .assert_same(&State::INIT_AFTER_ARRAY_ITEM);
    },
);

impl<'a> NonEmptyArrayAsStr<'a> {
    const fn debug_assert(&self) {
        debug_assert!(matches!(
            self.0.as_bytes(),
            [b'[', items @ .., b']'] if items.len() > 0
        ));
    }

    pub const fn items(&self) -> &'a str {
        self.debug_assert();
        let [b'[', items @ .., b']'] = self.0.as_bytes() else {
            // SAFETY: self.0 is json array
            unsafe { unreachable_unchecked() }
        };

        // SAFETY: self.0 is json array
        unsafe { str::from_utf8_unchecked(items) }
    }

    pub const fn items_close(&self) -> &'a str {
        self.debug_assert();
        let [b'[', items_close @ ..] = self.0.as_bytes() else {
            // SAFETY: self.0 is json array
            unsafe { unreachable_unchecked() }
        };

        // SAFETY: self.0 is json array
        unsafe { str::from_utf8_unchecked(items_close) }
    }

    pub const fn open_items(&self) -> &'a str {
        self.debug_assert();
        let [open_items @ .., b']'] = self.0.as_bytes() else {
            // SAFETY: self.0 is json array
            unsafe { unreachable_unchecked() }
        };

        // SAFETY: self.0 is json array
        unsafe { str::from_utf8_unchecked(open_items) }
    }
}

define!(
    NonEmptyObjectAsArray,
    NonEmptyObjectAsArrayVec,
    NonEmptyObjectAsStr,
    |v| {
        let chunk = v.remove_surrounding_group();
        chunk
            .prev_state()
            .copied()
            .assert_is_top_level_after_array_start();
        chunk
            .next_state()
            .assert_same(&State::INIT_AFTER_ARRAY_ITEM);
    },
);

define!(
    ContentfulFirstChunkOfArrayAsArray,
    ContentfulFirstChunkOfArrayAsArrayVec,
    ContentfulFirstChunkOfArrayAsStr,
    generics![
        Next: ?Sized + HasConstState,
    ],
    generic_params![Next,],
    |v| {
        v.prev_state().assert_init();
        Next::STATE.assert_same(v.next_state());
        assert!(!Next::STATE.is_eof());
        match v.inner().as_bytes() {
            [b'[', content @ ..] => {
                if content.is_empty() {
                    panic!("expect first chunk of array to be contentful")
                }
            }
            _ => panic!("expect first chunk of array"),
        }
    },
);

define!(
    ContentfulFirstChunkOfObjectAsArray,
    ContentfulFirstChunkOfObjectAsArrayVec,
    ContentfulFirstChunkOfObjectAsStr,
    generics![
        Next: ?Sized + HasConstState,
    ],
    generic_params![Next,],
    |v| {
        v.prev_state().assert_init();
        Next::STATE.assert_same(v.next_state());
        assert!(!Next::STATE.is_eof());
        match v.inner().as_bytes() {
            [b'{', content @ ..] => {
                if content.is_empty() {
                    panic!("expect first chunk of object to be contentful")
                }
            }
            _ => panic!("expect first chunk of object"),
        }
    },
);

define!(
    ContentfulLastChunkOfArrayAsArray,
    ContentfulLastChunkOfArrayAsArrayVec,
    ContentfulLastChunkOfArrayAsStr,
    generics![
        Prev: ?Sized + HasConstState,
    ],
    generic_params![Prev,],
    |v| {
        assert!(!Prev::STATE.is_init());
        Prev::STATE.assert_same(v.prev_state());
        v.next_state().assert_eof();
        match v.inner().as_bytes() {
            [content @ .., b']'] => {
                if content.is_empty() {
                    panic!("expect last chunk of array to be contentful")
                }
            }
            _ => panic!("expect last chunk of array"),
        }
    },
);

define!(
    ContentfulLastChunkOfObjectAsArray,
    ContentfulLastChunkOfObjectAsArrayVec,
    ContentfulLastChunkOfObjectAsStr,
    generics![
        Prev: ?Sized + HasConstState,
    ],
    generic_params![Prev,],
    |v| {
        assert!(!Prev::STATE.is_init());
        Prev::STATE.assert_same(v.prev_state());
        v.next_state().assert_eof();
        match v.inner().as_bytes() {
            [content @ .., b'}'] => {
                if content.is_empty() {
                    panic!("expect last chunk of object to be contentful")
                }
            }
            _ => panic!("expect last chunk of object"),
        }
    },
);

define!(
    IntermediateChunkAsArray,
    IntermediateChunkAsArrayVec,
    IntermediateChunkAsStr,
    generics![
        Prev: ?Sized + HasConstState,
        Next: ?Sized + HasConstState,
    ],
    generic_params![Prev, Next,],
    |v| {
        Prev::STATE.assert_same(v.prev_state());
        Next::STATE.assert_same(v.next_state());
        assert!(!Prev::STATE.is_init());
        assert!(!Next::STATE.is_eof());
    },
);

impl_many!({
    {
        {
            use ContentfulFirstChunkOfArrayAsStr as ContentfulFirstChunkAsStr;
            use ContentfulLastChunkOfArrayAsStr as ContentfulLastChunkAsStr;
            use NonEmptyArrayAsStr as NonEmptyAsStr;

            const OPEN: u8 = b'[';
            const CLOSE: u8 = b']';
            const fn assert_is_contentful_first_chunk(next: State) {
                next.assert_is_contentful_first_chunk_of_array()
            }
        }
        {
            use ContentfulFirstChunkOfObjectAsStr as ContentfulFirstChunkAsStr;
            use ContentfulLastChunkOfObjectAsStr as ContentfulLastChunkAsStr;
            use NonEmptyObjectAsStr as NonEmptyAsStr;

            const OPEN: u8 = b'{';
            const CLOSE: u8 = b'}';
            const fn assert_is_contentful_first_chunk(next: State) {
                next.assert_is_contentful_first_chunk_of_object()
            }
        }
    }

    impl<'a, Next: ?Sized + HasConstState> ContentfulFirstChunkAsStr<'a, Next> {
        pub(crate) const fn remove_group_open(self) -> &'a str {
            const { assert_is_contentful_first_chunk(Next::STATE) }
            match self.0.as_bytes() {
                [OPEN, content @ ..] => {
                    debug_assert!(!content.is_empty());
                    // SAFETY: the chunk is valid utf8
                    unsafe { str::from_utf8_unchecked(content) }
                }
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("invalid ContentfulFirstChunkAsStr")
                    }

                    // SAFETY: the chunk is valid
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
        }
    }

    impl<'a, Prev: ?Sized + HasConstState> ContentfulLastChunkAsStr<'a, Prev> {
        pub(crate) const fn remove_group_close(self) -> &'a str {
            const { assert_is_contentful_first_chunk(Prev::STATE) }
            match self.0.as_bytes() {
                [content @ .., CLOSE] => {
                    debug_assert!(!content.is_empty());
                    // SAFETY: the chunk is valid utf8
                    unsafe { str::from_utf8_unchecked(content) }
                }
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("invalid ContentfulLastChunkAsStr")
                    }

                    // SAFETY: the chunk is valid
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
        }
    }

    impl<'a> NonEmptyAsStr<'a> {
        pub(crate) const fn remove_group_open(self) -> &'a str {
            match self.0.as_bytes() {
                [OPEN, content_close @ ..] => {
                    debug_assert!(matches!(
                        content_close,
                        [content @ .., CLOSE] if !content.is_empty()
                    ));
                    // SAFETY: the chunk is valid utf8
                    unsafe { str::from_utf8_unchecked(content_close) }
                }
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("invalid NonEmptyAsStr")
                    }

                    // SAFETY: the chunk is valid
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
        }

        pub(crate) const fn remove_group_close(self) -> &'a str {
            match self.0.as_bytes() {
                [open_content @ .., CLOSE] => {
                    debug_assert!(matches!(
                        open_content,
                        [OPEN, content @ ..] if !content.is_empty()
                    ));
                    // SAFETY: the chunk is valid utf8
                    unsafe { str::from_utf8_unchecked(open_content) }
                }
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("invalid NonEmptyAsStr")
                    }

                    // SAFETY: the chunk is valid
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
        }

        pub(crate) const fn remove_surrounding_group(self) -> &'a str {
            match self.0.as_bytes() {
                [OPEN, content @ .., CLOSE] => {
                    debug_assert!(!content.is_empty());
                    // SAFETY: the chunk is valid utf8
                    unsafe { str::from_utf8_unchecked(content) }
                }
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("invalid NonEmptyAsStr")
                    }

                    // SAFETY: the chunk is valid
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
        }
    }
});
