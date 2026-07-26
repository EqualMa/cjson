use polonius_the_crab::{ForLt, PoloniusResult, polonius};

use crate::{
    ser::{
        iter_text_chunk::IterTextChunk,
        texts::Bracketed,
        traits::{IntoTextChunks, proxy_IntoTextChunks},
    },
    utils::size_hint::SizeHint,
};

use super::{ToJson, ToJsonArray, traits};

macro_rules! impl_tuple {
    ($pre:tt) => {};
    (
        ($({$($pre:tt)+})*)
        $i:tt
        $($rest:tt)*
    ) => {
        impl_tuple_one! {
            $({$($pre)+ $i})*
            { $i }
        }
        impl_tuple! {
            (
                $({$($pre)+ $i})*
                { $i }
            )
            $($rest)*
        }
    };
}

macro_rules! impl_tuple_one {
    () => {};
    ($({( $TN:tt $tn:tt ) $($t_rest:tt)*})+) => {
        impl<$($TN: ToJson,)+> ToJson for ($($TN,)+) {
            type ToJson<'a>
                = <Self as ToJsonArray>::ToJsonArray<'a>
            where
                Self: 'a;

            fn to_json(&self) -> Self::ToJson<'_> {
                Self::to_json_array(self)
            }
        }

        impl<$($TN: ToJson,)+> ToJsonArray for ($($TN,)+) {
            type ToJsonArray<'a>
                = Bracketed<NonEmptyTupleToItems<($($TN::ToJson<'a>,)+)>>
            where
                Self: 'a;

            fn to_json_array(&self) -> Self::ToJsonArray<'_> {
                let ($($tn,)+) = self;
                Bracketed(NonEmptyTupleToItems((
                    $(
                        $TN::to_json($tn),
                    )+
                )))
            }
        }

        impl<$($TN: traits::Text,)+> NonEmptyTuple for ($($TN,)+) {
            type IntoTextChunks = NonEmptyTupleItemsChunks<Self>;

            fn into_text_chunks(self) -> Self::IntoTextChunks {
                NonEmptyTupleItemsChunks(self)
            }
        }

        impl_IntoTextChunks_for_NonEmptyTupleItemsChunks! {
            $({( $TN $tn ) $($t_rest)*})+
        }
    };
}

macro_rules! underscore {
    ($ignore:tt) => {
        _
    };
}

macro_rules! update_discriminant {
    ($dis:ident) => {
        _ = $dis;
        PoloniusResult::Owned(LastDone())
    };
    ($dis:ident ( $TN:tt $tn:tt ) $($rest:tt)*) => {
        *$dis = ChunkDiscriminant::$TN;
        PoloniusResult::Borrowing(Some(Chunk::Comma))
    };
}

macro_rules! one {
    ($ignore:tt) => {
        1
    };
}

macro_rules! tn {
    ($TN:tt $tn:ident) => {
        $tn
    };
}

macro_rules! size_hint {
    (($TLast:tt $last:tt)) => {
        $TLast::bytes_len_hint($last)
    };
    ($(($TN:tt $tn:tt))+) => {
        (
            $( SizeHint($TN::bytes_len_hint($tn)) + )+
            const {
                // commas
                0 $( + one!($TN) )+
            }
        )
        .0
    };
}

macro_rules! impl_IntoTextChunks_for_NonEmptyTupleItemsChunks {
    ($T0:tt) => {};
    ($({( $TN:tt $tn:tt ) $($t_rest:tt)*})+) => {
        const _: () = {

            pub enum Chunk<$($TN,)+> {
                Comma,
                $($TN($TN),)+
            }

            impl<$($TN: AsRef<[u8]>,)+> AsRef<[u8]> for Chunk<$($TN,)+> {
                fn as_ref(&self) -> &[u8] {
                    match self {
                        Chunk::Comma => b",",
                        $(Chunk::$TN(this) => $TN::as_ref(this),)+
                    }
                }
            }

            pub struct Chunks<$($TN,)+>(Option<(($($TN,)+), ChunkDiscriminant)>);

            enum ChunkDiscriminant {
                $($TN,)+
            }

            impl<$($TN: IterTextChunk,)+> IterTextChunk for Chunks<$($TN,)+> {
                type Chunk<'a>
                    = Chunk<$($TN::Chunk<'a>,)+>
                where
                    Self: 'a;

                fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
                    struct LastDone();
                    match polonius::<_, LastDone, ForLt![<'a> = Option<Chunk<$($TN::Chunk<'a>,)+>>]>(
                        &mut self.0,
                        |this| match this {
                            $(
                                Some(((.., cur, $(underscore![$t_rest],)*), dis @ ChunkDiscriminant::$TN)) => match $TN::next_text_chunk(cur)
                                {
                                    Some(chunk) => PoloniusResult::Borrowing(Some(Chunk::$TN(chunk))),
                                    None => {
                                        update_discriminant! {dis $($t_rest)*}
                                    }
                                },
                            )+
                            None => PoloniusResult::Borrowing(None),
                        },
                    ) {
                        PoloniusResult::Borrowing(out) => out,
                        PoloniusResult::Owned {
                            value: LastDone(),
                            input_borrow: this,
                        } => {
                            *this = None;
                            None
                        }
                    }
                }

                fn bytes_len_hint(&self) -> (usize, Option<usize>) {
                    match &self.0 {
                        $(
                            Some(((.., $tn, $(tn! $t_rest,)*), ChunkDiscriminant::$TN)) => {
                                size_hint! {( $TN $tn ) $($t_rest)*}
                            }
                        )+
                        None => (0, Some(0)),
                    }
                }

                // TODO:
                // fn _private_collect_into_vec(mut self) -> ::alloc::vec::Vec<u8> {}
            }

            impl<$($TN: traits::Text,)+> IntoTextChunks for NonEmptyTupleItemsChunks<($($TN,)+)> {
                type IntoTextChunks = Chunks<$($TN::IntoTextChunks,)+>;

                fn into_text_chunks(self) -> Self::IntoTextChunks {
                    let ($($tn,)+) = self.0;
                    Chunks(Some((
                        (
                            $($TN::into_text_chunks($tn),)+
                        ),
                        ChunkDiscriminant::T0,
                    )))
                }

                // TODO:
                // fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {}

                fn write_into<W: ?Sized + traits::ConsumeTextChunk>(self, w: &mut W) {
                    let ($($tn,)+) = self.0;
                    $(
                        $TN::write_into($tn, w);
                    )+
                }

                fn try_write_into<W: ?Sized + traits::TryConsumeTextChunk>(
                    self,
                    w: &mut W,
                ) -> Result<(), W::Err> {
                    let ($($tn,)+) = self.0;
                    $(
                        $TN::try_write_into($tn, w)?;
                    )+
                    Ok(())
                }

                async fn async_try_write_into<W: ?Sized + traits::AsyncTryConsumeTextChunk>(
                    self,
                    w: &mut W,
                ) -> Result<(), W::Err> {
                    let ($($tn,)+) = self.0;
                    $(
                        $TN::async_try_write_into($tn, w).await?;
                    )+
                    Ok(())
                }
            }
        };
    };
}

impl_tuple! {
    ()
    (T0 t0)
    (T1 t1)
    (T2 t2)
    (T3 t3)
    (T4 t4)
    (T5 t5)
    (T6 t6)
    (T7 t7)
    (T8 t8)
    (T9 t9)
    (T10 t10)
    (T11 t11)
}

pub trait NonEmptyTuple {
    type IntoTextChunks: IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks;
}

pub struct NonEmptyTupleToItems<Items>(Items);

impl<Items: NonEmptyTuple> traits::sealed::EmptyOrCommaSeparatedElements
    for NonEmptyTupleToItems<Items>
{
}
impl<Items: NonEmptyTuple> traits::EmptyOrCommaSeparatedElements for NonEmptyTupleToItems<Items> {
    traits::impl_EmptyOrCommaSeparatedElements_for_NonEmptyCommaSeparatedElements! {}
}

impl<Items: NonEmptyTuple> traits::sealed::NonEmptyCommaSeparatedElements
    for NonEmptyTupleToItems<Items>
{
}
impl<Items: NonEmptyTuple> traits::NonEmptyCommaSeparatedElements for NonEmptyTupleToItems<Items> {}

impl<Items: NonEmptyTuple> IntoTextChunks for NonEmptyTupleToItems<Items> {
    proxy_IntoTextChunks!(|self| -> Items::IntoTextChunks { Items::into_text_chunks(self.0) });
}

pub struct NonEmptyTupleItemsChunks<Items>(Items);

impl<T0: traits::Text> IntoTextChunks for NonEmptyTupleItemsChunks<(T0,)> {
    proxy_IntoTextChunks!(|self| -> T0 { self.0.0 });
}
