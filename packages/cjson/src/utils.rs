pub(crate) mod iter_map;
pub(crate) mod size_hint;

macro_rules! impl_many {
    (impl$(<__>)? $Trait:ident for each_of![$($Ty:ty),+ $(,)?] $body:tt) => {
        $(
            impl $Trait for $Ty $body
        )+
    };
}

pub(crate) use impl_many;

macro_rules! option_if_some_or {
    ($option:expr, |$some:pat_param| $some_out:expr, $none:expr $(,)?) => {
        if let ::core::option::Option::Some($some) = $option {
            $some_out
        } else {
            $none
        }
    };
    ($option:expr, $map_some:expr, $none:expr $(,)?) => {
        $crate::utils::option_if_some_or!($option, |v| $map_some(v), $none)
    };
}

macro_rules! option_map {
    ($v:expr, $f:expr $(,)?) => {
        if let ::core::option::Option::Some(v) = $v {
            ::core::option::Option::Some($f(v))
        } else {
            ::core::option::Option::None
        }
    };
}

macro_rules! option_map_or {
    ($option:expr, $default:expr, |$some:pat_param| $mapped:expr $(,)?) => {
        $crate::utils::option_if_some_or!($option, |$some| $mapped, $default)
    };
    ($option:expr, $default:expr, $f:expr $(,)?) => {
        $crate::utils::option_if_some_or!($option, $f, $default)
    };
}

macro_rules! option_unwrap_or {
    ($v:expr, $default:expr $(,)?) => {
        if let ::core::option::Option::Some(v) = $v {
            v
        } else {
            $default
        }
    };
}

pub(crate) use {option_if_some_or, option_map, option_map_or, option_unwrap_or};
