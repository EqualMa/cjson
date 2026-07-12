pub trait ArrayOrObjectOpenClose: Sized {}
pub trait StringOpenClose: ArrayOrObjectOpenClose {}

macro_rules! define {
    (
        $(
            $OpenClose:ident
        ),+ $(,)?
    ) => {$(
        pub struct $OpenClose;

        impl ArrayOrObjectOpenClose for $OpenClose {}
    )+};
}

define!(
    NothingNothing,
    NothingComma,
    NothingGroup,
    CommaNothing,
    CommaComma,
    CommaGroup,
    GroupNothing,
    GroupComma,
    GroupGroup,
);

impl StringOpenClose for NothingNothing {}
impl StringOpenClose for NothingGroup {}
impl StringOpenClose for GroupNothing {}
impl StringOpenClose for GroupGroup {}
