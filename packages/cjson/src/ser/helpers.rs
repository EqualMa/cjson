macro_rules! json_fns {
    ({
        $json_provide_into:ident::
        $json_provide_into_try:ident::
        $json_provide_into_async_try:ident::
        $JsonKind:ident;
        $($rest:tt)*
    }) => {
        crate::ser::helpers::json_fns! { match_use {
            $json_provide_into
            $json_provide_into_try
            $json_provide_into_async_try
            $JsonKind
        } {$($rest)*} {$($rest)*} }
    };
    (
        match_use $idents:tt
        { use    $_use_alias:ident ;            $($_rest:tt)*}
        {$use:tt  $use_alias:tt    $use_semi:tt $( $rest:tt)*}
    ) => {
        crate::ser::helpers::json_fns! {
            match_async {
                $idents
                use { $use $use_alias $use_semi }
            }
            {$($_rest)*} {$($rest)*}
        }
    };
    (
        match_use $idents:tt
        $_rest:tt
        $rest:tt
    ) => {
        crate::ser::helpers::json_fns! {
            match_async {
                $idents
                use {}
            }
            $_rest $rest
        }
    };
    (
        match_async $data:tt
        { async    $($_rest:tt)*}
        {$async:tt $( $rest:tt)*}
    ) => {
        crate::ser::helpers::json_fns! {
            @$data
            async { $async }
            {$($rest)*}
        }
    };
    (
        match_async $data:tt
        $_rest:tt
        $rest:tt
    ) => {
        crate::ser::helpers::json_fns! {
            @$data
            async {}
            $rest
        }
    };
    (
        @{
            {
                $json_provide_into:ident
                $json_provide_into_try:ident
                $json_provide_into_async_try:ident
                $JsonKind:ident
            }
            use { $( $use:tt $use_alias:tt $use_semi:tt)?}
        }
        async {$($async:tt)?}
        {
            |$(&$($self_lt:lifetime)?)? $self1:ident $($self2:ident)?, $w:pat_param $(,)?| $imp:expr
        }
    ) => {
        fn $json_provide_into<
            W: crate::ser::ConsumeJson<
                    ConsumeJsonKind: crate::ser::json_kinds::JsonKind<
                        Contains<Self::$JsonKind> = (),
                    >,
                >,
        >(
            $(&$($self_lt)?)? $self1 $($self2)?,
            $w: W,
        ) -> crate::ser::Consumed<Self::$JsonKind, W> {
            $( $use crate::ser::define_traits::base as $use_alias $use_semi )?
            $imp
        }

        fn $json_provide_into_try<
            W: crate::ser::TryConsumeJson<
                    ConsumeJsonKind: crate::ser::json_kinds::JsonKind<
                        Contains<Self::$JsonKind> = (),
                    >,
                >,
        >(
            $(&$($self_lt)?)? $self1 $($self2)?,
            $w: W,
        ) -> Result<
            crate::ser::Consumed<Self::$JsonKind, W>,
            <W::Writer as crate::ser::traits::TryConsumeTextChunk>::Err,
        > {
            $( $use crate::ser::define_traits::try_ as $use_alias $use_semi )?
            $imp
        }

        $($async)? fn $json_provide_into_async_try<
            W: crate::ser::AsyncTryConsumeJson<
                    ConsumeJsonKind: crate::ser::json_kinds::JsonKind<
                        Contains<Self::$JsonKind> = (),
                    >,
                >,
        >(
            $(&$($self_lt)?)? $self1 $($self2)?,
            $w: W,
        ) -> crate::ser::helpers::future_if_no_async![ [$($async)?]
            Result<
                crate::ser::Consumed<Self::$JsonKind, W>,
                <W::Writer as crate::ser::traits::AsyncTryConsumeTextChunk>::Err,
            >
        ] {
            $( $use crate::ser::define_traits::async_try as $use_alias $use_semi )?
            $imp
        }
    };
}

macro_rules! future_if_no_async {
    ([async] $Out:ty) => {
        $Out
    };
    ([     ] $Out:ty) => {
        impl Future<Output = $Out>
    };
}

pub(crate) use {future_if_no_async, json_fns};
