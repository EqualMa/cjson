//! [`base`] : sync, not blocking, not fallible
//! [`try_`] : sync, blocking, fallible
//! [`async_try`] : async, not blocking, fallible
//!
//! `blocking, not fallible` and `async, not fallible` are not considered in this crate.

macro_rules! verbatim {
    [ $($t:tt)* ] => [ $($t)* ];
}

pub mod base {
    pub use crate::ser::consumers::{trait_helpers::base::*, trait_items::base::*};

    macro_rules! Output {
        ($T:ty) => {
            crate::ser::consumers::define_traits::base::Output![$T, Self::Writer]
        };
        ($T:ty, $W:ty) => {
            $T
        };
    }

    macro_rules! select {
        ({ $t:tt $try:tt $async_try:tt }) => ( $t );
        (
            {$($t:tt)*}
            $($rest:tt)*
        ) => {
            $($t)*
        };
        (
            base[$($t:tt)*]
            $($rest:tt)*
        ) => {
            $($t)*
        };
    }

    macro_rules! select_expr {
        [ $e:expr, $try:expr, $async_try:expr $(,)? ] => ( $e );
    }

    macro_rules! select_type {
        [ $e:ty, $try:ty, $async_try:ty $(,)? ] => ( $e );
    }

    macro_rules! select_method {
        (($e:expr).$method:ident $args:tt . $try_method:ident . $async_try_method:ident $(.await $(?)?)?) => {
            $e.$method $args
        };
        ($e:tt    .$method:ident $args:tt . $try_method:ident . $async_try_method:ident $(.await $(?)?)?) => {
            $e.$method $args
        };
    }

    macro_rules! de_async {
        (async $($rest:tt)*) => { $($rest)* };
    }

    macro_rules! de_async_move {
        (async move $($rest:tt)*) => { $($rest)* };
    }

    pub(crate) use {
        Output, Output as AwaitedOutput, de_async, de_async_move, last_expr as only_expr, select,
        select_expr, select_method, select_type, verbatim as await_, verbatim as async_block,
        verbatim as async_move_block, verbatim as last_expr, verbatim as await_try,
        verbatim as never_future, verbatim as async_,
    };
}
pub mod try_ {
    pub use crate::ser::consumers::{trait_helpers::try_::*, trait_items::try_::*};

    macro_rules! Output {
        ($T:ty) => {
            crate::ser::consumers::define_traits::try_::Output![$T, Self::Writer]
        };
        ($T:ty, $W:ty) => {
            Result::<$T, <$W as crate::ser::traits::TryConsumeTextChunk>::Err>
        };
    }

    macro_rules! select {
        ({ $base:tt $t:tt $async_try:tt }) => ( $t );
        (
            $base:tt,
            {$($t:tt)*}
            $($rest:tt)*
        ) => {
            $($t)*
        };
        (
            base $base:tt
            try [$($t:tt)*]
            $($rest:tt)*
        ) => {
            $($t)*
        };
    }

    macro_rules! select_expr {
        [ $base:expr, $e:expr, $async_try:expr $(,)? ] => ( $e );
    }

    macro_rules! select_type {
        [ $base:ty, $e:ty, $async_try:ty $(,)? ] => ( $e );
    }

    macro_rules! select_method {
        (($e:expr).$method:ident $args:tt . $try_method:ident . $async_try_method:ident $(.await)?) => {
            $e.$try_method $args
        };
        ($e:tt    .$method:ident $args:tt . $try_method:ident . $async_try_method:ident $(.await)?) => {
            $e.$try_method $args
        };
        (($e:expr).$method:ident $args:tt . $try_method:ident . $async_try_method:ident .await?) => {
            $e.$try_method $args ?
        };
        ($e:tt    .$method:ident $args:tt . $try_method:ident . $async_try_method:ident .await?) => {
            $e.$try_method $args ?
        };
    }

    macro_rules! await_try {
        ($e:expr) => {
            $e?
        };
    }

    macro_rules! last_expr {
        ($e:expr) => {
            Ok($e)
        };
    }

    pub(crate) use super::base::{
        async_, async_block, async_move_block, await_, de_async, de_async_move, never_future,
    };
    pub(crate) use {
        Output, Output as AwaitedOutput, await_try, last_expr, last_expr as only_expr, select,
        select_expr, select_method, select_type,
    };
}
pub mod async_try {
    pub use crate::ser::consumers::{trait_helpers::async_try::*, trait_items::async_try::*};

    macro_rules! Output {
        ($T:ty) => {
            crate::ser::consumers::define_traits::async_try::Output![$T, Self::Writer]
        };
        ($T:ty, $W:ty) => {
            impl Future<Output = Result::<$T, <$W as crate::ser::traits::AsyncTryConsumeTextChunk>::Err>>
        };
    }

    macro_rules! AwaitedOutput {
        ($T:ty) => {
            crate::ser::consumers::define_traits::async_try::AwaitedOutput![$T, Self::Writer]
        };
        ($T:ty, $W:ty) => {
            Result::<$T, <$W as crate::ser::traits::AsyncTryConsumeTextChunk>::Err>
        };
    }

    macro_rules! select {
        ({ $base:tt $try:tt $t:tt }) => ( $t );
        (
            $base:tt,
            $try_:tt,
            {$($t:tt)*}
            $(,)?
        ) => {
            $($t)*
        };
        (
            base $base:tt
            try $try:tt
            async_try[$($t:tt)*]
        ) => {
            $($t)*
        };
    }

    macro_rules! select_expr {
        [ $base:expr, $try:expr, $e:expr $(,)? ] => ( $e );
    }

    macro_rules! select_type {
        [ $base:ty, $try:ty, $e:ty $(,)? ] => ( $e );
    }

    macro_rules! select_method {
        (($e:expr).$method:ident $args:tt . $try_method:ident . $async_try_method:ident $($postfix:tt)*) => {
            $e.$async_try_method $args $($postfix)*
        };
        ($e:tt    .$method:ident $args:tt . $try_method:ident . $async_try_method:ident $($postfix:tt)*) => {
            $e.$async_try_method $args $($postfix)*
        };
    }

    macro_rules! await_try {
        ($e:expr) => {
            $e.await?
        };
    }

    macro_rules! async_block {
        ($b:tt) => {
            async $b
        };
    }

    macro_rules! async_move_block {
        ($b:tt) => {
            async move $b
        };
    }

    macro_rules! await_ {
        ($e:expr) => {
            $e.await
        };
    }

    macro_rules! only_expr {
        ($e:expr) => {
            ::core::future::ready(Ok($e))
        };
    }

    macro_rules! never_future {
        ($e:expr) => {
            $e as crate::never_future::NeverFuture<_>
        };
    }

    macro_rules! async_ {
        ($($t:tt)*) => {
            async $($t)*
        };
    }

    pub(crate) use super::try_::last_expr;
    pub(crate) use {
        AwaitedOutput, Output, async_, async_block, async_move_block, await_, await_try,
        never_future, only_expr, select, select_expr, select_method, select_type,
        verbatim as de_async_move, verbatim as de_async,
    };
}
