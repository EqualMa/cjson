use proc_macro::{Literal, Span, TokenTree};
use typed_quote::WithSpan as _;

use crate::{ErrorCollector, expand_props::TokensCollector};

use super::{StructTagExpandError, TryWithOutSpan as _};

pub trait ContextWithPropTag {
    const MSG_PROP_TAG_NOT_DEFINED: &'static str;

    fn prop_tag_mut(&mut self) -> ContextPropTagMut<'_>;

    fn expand_tag(&mut self, out: TokensCollector<'_>, span: Span, errors: &mut ErrorCollector) {
        self.try_with_out_span(out, span, errors, |ctx, out, _span| ctx.try_expand_tag(out));
    }

    fn try_expand_tag(&mut self, mut out: TokensCollector<'_>) -> Result<(), StructTagExpandError> {
        let (ts, res) = self.prop_tag_mut().expand(Self::MSG_PROP_TAG_NOT_DEFINED);
        out.extend_from_slice(ts);
        res
    }
}

pub enum ContextPropTagMut<'a> {
    Untagged {
        default_span: Span,
        cache_for_dummy: &'a mut Option<Vec<TokenTree>>,
    },
    Tagged {
        span_tag: Span,
        ts: &'a [TokenTree],
        accessed: &'a mut bool,
    },
}

impl<'a> ContextPropTagMut<'a> {
    fn expand(self, msg: &'static str) -> (&'a [TokenTree], Result<(), StructTagExpandError>) {
        let res;
        let ts = match self {
            ContextPropTagMut::Untagged {
                default_span,
                cache_for_dummy,
            } => {
                res = Err(StructTagExpandError(msg));
                cache_for_dummy.get_or_insert_with(|| {
                    let tt = Literal::string("").with_replaced_span(default_span).into();
                    vec![tt]
                })
            }
            ContextPropTagMut::Tagged {
                span_tag: _,
                ts,
                accessed,
            } => {
                *accessed = true;
                res = Ok(());
                ts
            }
        };
        (ts, res)
    }
}
