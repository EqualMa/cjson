use core::{marker::PhantomData, mem::MaybeUninit};

use crate::{ser::texts::StrToJsonStringFragment, values};

macro_rules! expand_or {
    ([         ][$($or:tt)*]) => {
        $($or)*
    };
    ([$($e:tt)+][$($or:tt)*]) => {
        $($e)+
    };
}

macro_rules! expand_if_else {
    ([            ][$($if:tt)*][$($else:tt)*]) => {
        $($else)*
    };
    ([$($cond:tt)+][$($if:tt)*][$($else:tt)*]) => {
        $($if)+
    };
}

macro_rules! define_literal_kind {
    (
        enum $Kind:ident {}
        $vis:vis enum $KindWithInfo:ident {
            $(
                $(#[map_info(|$v:pat_param| $info:expr)])?
                $Var:ident($Info:ty)
            ),+ $(,)?
        }
    ) => {
        #[repr(u8)]
        enum $Kind {$(
            $Var,
        )+}

        $vis enum $KindWithInfo {$(
            $Var($Info),
        )+}

        impl $KindWithInfo {
            pub const fn into_kind_with_info(self) -> (u8, usize) {
                match self {$(
                    Self::$Var(expand_or![[$($v)?][info]]) => (
                        $Kind::$Var as u8,
                        expand_or![[$($info)?][info]]
                    ),
                )+}
            }
        }
    };
}

define_literal_kind!(
    enum RustLiteralKind {}
    pub enum RustLiteralKindWithInfo {
        #[map_info(|v| if v { 1 } else { 0 })]
        Bool(bool),
        F32(usize),
        F64(usize),
        I8(usize),
        I16(usize),
        I32(usize),
        I64(usize),
        Isize(usize),
        I128(usize),
        U8(usize),
        U16(usize),
        U32(usize),
        U64(usize),
        Usize(usize),
        U128(usize),
        Str(usize),
    }
);

macro_rules! define_macro_ConstKindAndData {
    ([$_:tt] $ConstKindAndData:ident {
        [KIND,]
        $([$param:ident, $($default:expr)?])*
    }) => {
        macro_rules! $ConstKindAndData {
            (
                $Kind:ty,
                $_(
                    $_ name:ident = $_ value:expr
                ),*
            ) => {
                $ConstKindAndData! {$Kind, $_(
                    $_ name = $_ value,
                )*}
            };
            (
                $Kind:ty,
                $(
                    $_($param = $_ $param:expr,)?
                )+
            ) => {
                ConstKindAndData::<
                    { GetKindAndData::<$Kind>::KIND },
                    $(
                        {expand_or!(
                            [$_($_ $param)?]
                            [expand_or!(
                                [$($default)?]
                                [compile_error!(concat!(stringify!($param), " must be specified"))]
                            )]
                        )},
                    )+
                >
            };
        }
    };
}

macro_rules! define_ConstKindAndData {
    (
        $vis:vis enum $ConstKindAndData:ident<$(
            const $NAME:ident: $Ty:ty $(= $default:literal)?
        ),+ $(,)?> {}
    ) => {
        $vis enum $ConstKindAndData<$(
            const $NAME: $Ty $(= $default)?
        ),+> {}

        define_macro_ConstKindAndData! {
            [$]
            $ConstKindAndData
            {
                $([$NAME, $($default)?])+
            }
        }
    };
}

define_ConstKindAndData!(
    pub enum ConstKindAndData<
        const KIND: u8,
        const LEN: usize = 0,
        const I8: i8 = 0,
        const I16: i16 = 0,
        const I32: i32 = 0,
        const I64: i64 = 0,
        const Isize: isize = 0,
        const I128: i128 = 0,
        const U8: u8 = 0,
        const U16: u16 = 0,
        const U32: u32 = 0,
        const U64: u64 = 0,
        const Usize: usize = 0,
        const U128: u128 = 0,
    > {}
);

pub struct RuntimeKindAndData {
    pub kind: u8,
    pub data: RuntimeData,
}

pub struct RuntimeData {
    pub len: usize,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub isize: isize,
    pub i128: i128,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub usize: usize,
    pub u128: u128,
}

impl RuntimeData {
    const DUMMY: Self = Self {
        len: 0,
        i8: 0,
        i16: 0,
        i32: 0,
        i64: 0,
        isize: 0,
        i128: 0,
        u8: 0,
        u16: 0,
        u32: 0,
        u64: 0,
        usize: 0,
        u128: 0,
    };
}

pub struct GetKindAndData<T>(T);

macro_rules! define_const_kind {
    (
        match $v:ident as $Kind:ident {$(
            $Var:ident => $(& $($lt:lifetime)?)? $Ty:ident $({
                $($field:ident: $field_value:expr),* $(,)?
            })?
        ),+ $(,)?}
    ) => {
        #[repr(u8)]
        enum $Kind {$(
            $Var,
        )+}

        $(impl GetKindAndData< $(& $($lt)?)? $Ty> {
            const KIND: u8 = $Kind::$Var as u8;
            pub const fn call_once(self) -> RuntimeKindAndData {
                let Self($v) = self;
                RuntimeKindAndData {
                    kind: Self::KIND,
                    data: expand_if_else!([
                        $({ $($field)* })?
                    ][
                        RuntimeData {
                            $($($field: $field_value,)*)?
                            ..RuntimeData::DUMMY
                        }
                    ][
                        // for integers
                        RuntimeData {
                            $Ty: $v,
                            len: {
                                let mut buf = [MaybeUninit::<_>::uninit(); <$Ty as const_itoa::Integer>::MAX_STR_LEN];
                                let s = const_itoa::Format(&mut buf, $v).call_once();
                                s.len()
                            },
                            ..RuntimeData::DUMMY
                        }
                    ]),
                }
            }
        })+

    };
}

define_const_kind!(match v as ConstValueKind {
    Bool => bool {
        u8: if v { 1 } else { 0 },
    },
    F32 => f32 {
        u32: v.to_bits(),
        len: {
            if !v.is_finite() {
                panic!("f32 is no finite");
            }
            let mut buf = const_ryu::Buffer::new();
            let string = const_ryu::FormatFinite(&mut buf, v).call_once();
            string.len()
        }
    },
    F64 => f64 {
        u64: v.to_bits(),
        len: {
            if !v.is_finite() {
                panic!("f64 is no finite");
            }
            let mut buf = const_ryu::Buffer::new();
            let string = const_ryu::FormatFinite(&mut buf, v).call_once();
            string.len()
        }
    },
    I8 => i8,
    I16 => i16,
    I32 => i32,
    I64 => i64,
    Isize => isize,
    I128 => i128,
    U8 => u8,
    U16 => u16,
    U32 => u32,
    U64 => u64,
    Usize => usize,
    U128 => u128,
    Str => &str {
        len: str_to_json_len(v)
    },
});

const fn str_to_json_len(s: &str) -> usize {
    let mut text_chunks = StrToJsonStringFragment(s).const_into_text_chunks();
    let mut len = 2usize; // two quotes
    while let Some(chunk) = text_chunks.next_text_chunk() {
        len += chunk.len();
    }

    len
}

pub trait HasConstValue {
    type ConstValue;
    const CONST_VALUE: Self::ConstValue;
}

pub trait ConstValueToJson {
    type TypeOfConstValue;
    type TypeOfConstValueToJson<T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>>;
    type MapConstValueToJson<T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>>: HasConstValue<
        ConstValue = Self::TypeOfConstValueToJson<T>,
    >;
}

enum Never {}

// region: bool
const _: () = {
    struct MapFalseToJson<T: ?Sized + HasConstValue<ConstValue = bool>>(Never, PhantomData<T>);

    impl<T: ?Sized + HasConstValue<ConstValue = bool>> HasConstValue for MapFalseToJson<T> {
        type ConstValue = values::False;

        const CONST_VALUE: Self::ConstValue = {
            assert!(!T::CONST_VALUE);
            values::False
        };
    }

    struct MapTrueToJson<T: ?Sized + HasConstValue<ConstValue = bool>>(Never, PhantomData<T>);

    impl<T: ?Sized + HasConstValue<ConstValue = bool>> HasConstValue for MapTrueToJson<T> {
        type ConstValue = values::True;

        const CONST_VALUE: Self::ConstValue = {
            assert!(T::CONST_VALUE);
            values::True
        };
    }

    impl ConstValueToJson for ConstKindAndData![bool, LEN = 0] {
        type TypeOfConstValue = bool;
        type TypeOfConstValueToJson<
            T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>,
        > = values::False;
        type MapConstValueToJson<T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>> =
            MapFalseToJson<T>;
    }

    impl ConstValueToJson for ConstKindAndData![bool, LEN = 1] {
        type TypeOfConstValue = bool;
        type TypeOfConstValueToJson<
            T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>,
        > = values::True;
        type MapConstValueToJson<T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>> =
            MapTrueToJson<T>;
    }
};
// endregion
// region: int
macro_rules! impl_integer {
    (
        $Var:ident as $Ty:ty
    ) => {
        const _: () = {
            struct ConstIntToJson<const $Var: $Ty, const LEN: usize>;
            struct MapConstIntToJson<
                T: ?Sized + HasConstValue<ConstValue = $Ty>,
                const $Var: $Ty,
                const LEN: usize,
            >(Never, PhantomData<T>);

            impl<const $Var: $Ty, const LEN: usize> ConstValueToJson
                for ConstKindAndData![$Ty, LEN = LEN, $Var = $Var,]
            {
                type TypeOfConstValue = $Ty;

                type TypeOfConstValueToJson<
                    T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>,
                > = ConstIntToJson<$Var, LEN>;

                type MapConstValueToJson<
                    T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>,
                > = MapConstIntToJson<T, $Var, LEN>;
            }

            impl<T: ?Sized + HasConstValue<ConstValue = $Ty>, const $Var: $Ty, const LEN: usize>
                HasConstValue for MapConstIntToJson<T, $Var, LEN>
            {
                type ConstValue = ConstIntToJson<$Var, LEN>;
                const CONST_VALUE: Self::ConstValue = {
                    assert!(T::CONST_VALUE == $Var);
                    ConstIntToJson
                };
            }
        };
    };
}

macro_rules! impl_integers {
    ($($Var:ident($Ty:ty)),+ $(,)?) => {
        $(impl_integer!{$Var as $Ty})+
    };
}

impl_integers!(
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Isize(isize),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    U128(u128),
);
// endregion
// region: str
const _: () = {
    pub struct ConstStrToJson<'a, T: ?Sized + HasConstValue<ConstValue = &'a str>, const LEN: usize>(
        PhantomData<T>,
    );

    impl<'a, T: ?Sized + HasConstValue<ConstValue = &'a str>, const LEN: usize>
        ConstStrToJson<'a, T, LEN>
    {
        const TO_STRING: [u8; LEN] = {
            let s = T::CONST_VALUE;
            let mut text_chunks = StrToJsonStringFragment(s).const_into_text_chunks();
            let mut ret = [0u8; LEN];

            let mut i = 0;
            ret[i] = b'"';

            while let Some(chunk) = text_chunks.next_text_chunk() {
                ret.split_at_mut(i)
                    .0
                    .split_at_mut(chunk.len())
                    .0
                    .copy_from_slice(chunk);
                i += chunk.len();
            }

            ret[i] = b'"';

            assert!(i == LEN);

            ret
        };
    }

    struct MapStrToJson<'a, T: ?Sized + HasConstValue<ConstValue = &'a str>, const LEN: usize>(
        Never,
        PhantomData<T>,
    );

    impl<'a, T: ?Sized + HasConstValue<ConstValue = &'a str>, const LEN: usize> HasConstValue
        for MapStrToJson<'a, T, LEN>
    {
        type ConstValue = ConstStrToJson<'a, T, LEN>;

        const CONST_VALUE: Self::ConstValue = {
            _ = ConstStrToJson::<'a, T, LEN>::TO_STRING;

            ConstStrToJson(PhantomData)
        };
    }

    impl<const LEN: usize> ConstValueToJson for ConstKindAndData![&str, LEN = LEN] {
        type TypeOfConstValue = &'static str;
        type TypeOfConstValueToJson<
            T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>,
        > = ConstStrToJson<'static, T, LEN>;
        type MapConstValueToJson<T: ?Sized + HasConstValue<ConstValue = Self::TypeOfConstValue>> =
            MapStrToJson<'static, T, LEN>;
    }
};
// endregion
// region: float
// endregion
