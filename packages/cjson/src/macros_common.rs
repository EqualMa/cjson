#[macro_export]
#[doc(hidden)]
macro_rules! __expand_or {
    ([         ][$($or:tt)*]) => ($($or)*);
    ([$($e:tt)+][$($or:tt)*]) => ($($e )+);
}
