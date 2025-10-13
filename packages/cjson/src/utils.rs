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
