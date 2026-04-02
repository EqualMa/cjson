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
        let (ts, res) = self.prop_tag_mut().try_into_tokens();
        out.extend_from_slice(ts);
        res.map_err(|()| StructTagExpandError(Self::MSG_PROP_TAG_NOT_DEFINED))
    }
}

#[derive(Default)]
pub struct CacheForDummyTag(Option<TokenTree>);

pub enum ContextPropTagMut<'a> {
    Untagged {
        default_span: Span,
        cache_for_dummy: &'a mut CacheForDummyTag,
    },
    Tagged {
        span_tag: Span,
        ts: &'a [TokenTree],
    },
}

impl<'a> ContextPropTagMut<'a> {
    pub fn try_into_tokens(self) -> (&'a [TokenTree], Result<(), ()>) {
        let res;
        let ts = match self {
            ContextPropTagMut::Untagged {
                default_span,
                cache_for_dummy: CacheForDummyTag(cache_for_dummy),
            } => {
                res = Err(());
                let tt = cache_for_dummy.get_or_insert_with(|| {
                    let tt = Literal::string("").with_replaced_span(default_span).into();
                    tt
                });

                std::array::from_ref(tt)
            }
            ContextPropTagMut::Tagged { span_tag: _, ts } => {
                res = Ok(());
                ts
            }
        };
        (ts, res)
    }
}
