use core::fmt;
use proc_macro::{Ident, Literal};

macro_rules! impl_many {
    (impl$(<__>)? $Trait:ident for each_of![$($Ty:ty),+ $(,)?] $body:tt) => {
        $(
            impl $Trait for $Ty $body
        )+
    };
    (
        type $T:ident = each_of! $each_of:tt;

        $($rest:tt)*
    ) => {
        $crate::utils::impl_many! {
            @impl_type $T $each_of
            {$($rest)*}
        }
    };
    (@impl_type $T:ident [$($Ty:ty),+ $(,)?] $rest:tt ) => {$(
        const _: () = {
            type $T = $Ty;
            const _: () = $rest;
        };
    )+};
    ({
        $defs:tt
        $($imps:tt)*
    }) => {
        crate::utils::impl_many! {
            @__defs
            $defs
            {$($imps)*}
        }
    };
    (@__defs { $($(#$def_attr:tt)* {$($defs:tt)*})+ } $imps:tt) => {
        $(
            $(#$def_attr)*
            const _: () = {
                $($defs)*

                crate::utils::impl_many! {
                    @__unwrap $imps
                }
            };
        )+
    };
    (@__unwrap {$($t:tt)*}) => {
        $($t)*
    }
}

pub(crate) use impl_many;

pub fn ident_to_literal_string(ident: &Ident) -> Literal {
    let mut lit = Literal::string(&ident.to_string());
    lit.set_span(ident.span());
    lit
}
#[cfg(todo)]
/// When [display](fmt::Display), `Ident` is either `["r#", ident]` or `[ident]`
pub fn ident_to_literal_string(ident: &Ident) -> Literal {
    enum Buf {
        Init,
        One(Literal),
        Raw(String),
        Unexpected,
    }

    impl Buf {
        fn push_str(&mut self, s: &str) {
            match self {
                Buf::Init => match s {
                    "r#" => *self = Self::Raw(String::new()),
                    s => {
                        if s.is_empty() {
                            return;
                        }
                        *self = Self::One(Literal::string(s))
                    }
                },
                Buf::One(_) => {
                    if s.is_empty() {
                        return;
                    }
                    *self = Self::Unexpected
                }
                Buf::Unexpected => {}
                Buf::Raw(this) => this.push_str(s),
            }
        }
    }

    impl fmt::Write for Buf {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            Ok(self.push_str(s))
        }
    }

    let mut buf = Buf::Init;
    {
        use core::fmt::Write as _;
        write!(buf, "{}", ident).unwrap();
    }
    let mut lit = 'lit: {
        let ident = match buf {
            Buf::One(literal) => break 'lit literal,
            Buf::Raw(raw) => raw,
            _ => ident.to_string(),
        };

        Literal::string(&ident)
    };

    lit.set_span(ident.span());

    lit
}
