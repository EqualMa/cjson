use polonius_the_crab::{ForLt, PoloniusResult, polonius};

use crate::utils::size_hint::SizeHint;

use super::{Chain, IterTextChunk, either::Chunk};

impl<A: IterTextChunk, B: IterTextChunk> IterTextChunk for Chain<A, B> {
    type Chunk<'a>
        = Chunk<A::Chunk<'a>, B::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        struct AssignNoneToSelf0;

        macro_rules! BorrowingOutput {
            ($lt:lifetime) => {
                Option<Chunk<A::Chunk<$lt>, B::Chunk<$lt>>>
            };
        }

        match polonius::<Self, AssignNoneToSelf0, ForLt![<'r> = BorrowingOutput!['r]]>(
            self,
            |this: &mut _| -> PoloniusResult<BorrowingOutput!['_], AssignNoneToSelf0> {
                match &mut this.0 {
                    Some((a_opt, b)) => {
                        struct AssignNoneToAOpt(bool);

                        match polonius::<
                            Option<A>,
                            AssignNoneToAOpt,
                            ForLt![<'r> = BorrowingOutput!['r]],
                        >(a_opt, |a_opt| -> _ {
                            match a_opt {
                                Some(a) => match a.next_text_chunk() {
                                    Some(chunk) => {
                                        return PoloniusResult::Borrowing(Some(Chunk::A(chunk)));
                                    }
                                    None => PoloniusResult::Owned(AssignNoneToAOpt(true)),
                                },
                                None => PoloniusResult::Owned(AssignNoneToAOpt(false)),
                            }
                        }) {
                            PoloniusResult::Borrowing(v) => PoloniusResult::Borrowing(v),
                            PoloniusResult::Owned {
                                value: AssignNoneToAOpt(assign_none_to_a_opt),
                                input_borrow: a_opt,
                            } => {
                                if assign_none_to_a_opt {
                                    *a_opt = None;
                                }
                                debug_assert!(a_opt.is_none());
                                match b.next_text_chunk() {
                                    Some(chunk) => PoloniusResult::Borrowing(Some(Chunk::B(chunk))),
                                    None => PoloniusResult::Owned(AssignNoneToSelf0),
                                }
                            }
                        }
                    }
                    None => PoloniusResult::Borrowing(None),
                }
            },
        ) {
            PoloniusResult::Borrowing(v) => v,
            PoloniusResult::Owned {
                value: AssignNoneToSelf0,
                input_borrow: this,
            } => {
                this.0 = None;
                None
            }
        }
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Some((a, b)) => match a {
                Some(a) => (SizeHint(a.bytes_len_hint()) + SizeHint(b.bytes_len_hint())).0,
                None => b.bytes_len_hint(),
            },
            None => (0, Some(0)),
        }
    }
}
