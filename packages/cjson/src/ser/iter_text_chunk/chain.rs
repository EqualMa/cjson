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
                    Some((a_done, a, b)) => {
                        if !*a_done {
                            match a.next_text_chunk() {
                                Some(chunk) => {
                                    return PoloniusResult::Borrowing(Some(Chunk::A(chunk)));
                                }
                                None => {
                                    *a_done = true;
                                }
                            }
                        }

                        match b.next_text_chunk() {
                            Some(chunk) => PoloniusResult::Borrowing(Some(Chunk::B(chunk))),
                            None => PoloniusResult::Owned(AssignNoneToSelf0),
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
            Some((a_done, a, b)) => {
                if !*a_done {
                    (SizeHint(a.bytes_len_hint()) + SizeHint(b.bytes_len_hint())).0
                } else {
                    b.bytes_len_hint()
                }
            }
            None => (0, Some(0)),
        }
    }
}
