use proc_macro::{Span, TokenTree};

use typed_quote::{IntoTokenTree as _, WithSpan as _, quote};

use crate::{
    ErrorCollector,
    expand_props::{Context, TokensCollector},
    to_json::ctx::custom::{CustomTokensExpanded, TokensExpanded},
};

use super::{
    CustomTokens, CustomTokensExpandErrorOr, HasConstCircularRefMsg, IntoParseErrorWithSpan,
    PropExpanded, PropExpandedWithErr, StructToDefaultExpandError, TryWithOutSpan as _,
    context_with_prop_to_tagged_default::StructToTaggedKvsExpandError, custom::PropCustom,
};

pub trait ContextWithPropTo: Sized + Context {
    fn prop_custom_to(&mut self) -> Option<&mut PropCustom>;

    fn try_expand_to_unspecified(
        &mut self,
        out: TokensCollector<'_>,
    ) -> Result<(), StructToUnspecifiedExpandError>;

    fn expand_to(&mut self, out: TokensCollector<'_>, span: Span, errors: &mut ErrorCollector) {
        self.try_with_out_span(out, span, errors, Self::try_expand_to)
    }

    fn try_expand_to(
        &mut self,
        mut out: TokensCollector<'_>,
        _span: Span, // TODO: link @to
    ) -> Result<(), StructToExpandError> {
        match self.prop_custom_to() {
            Some(custom) => {
                let mut f = |(ts, res): &mut CustomTokensExpanded| {
                    out.extend_from_slice(ts);
                    res.clone().map_err(|e| {
                        e.map_circular_ref(|()| StructToUnspecifiedExpandError::CIRCULAR_REF_MSG)
                    })
                };
                match custom.cached() {
                    Some(v) => f(v),
                    None => PropCustom::use_expanded(
                        self,
                        |ctx| ctx.prop_custom_to().unwrap(),
                        StructToUnspecifiedExpandError::default_for_circular_ref,
                        f,
                    ),
                }
                .map_err(CustomTokensExpandErrorOr::Custom)
            }
            None => self
                .try_expand_to_unspecified(out)
                .map_err(CustomTokensExpandErrorOr::Other),
        }
    }
}

#[derive(Clone)]
pub enum StructToUnspecifiedExpandError {
    Untagged(StructToDefaultExpandError),
    Tagged(StructToTaggedKvsExpandError),
}

pub type StructToExpandError = CustomTokensExpandErrorOr<StructToUnspecifiedExpandError>;

impl HasConstCircularRefMsg for StructToUnspecifiedExpandError {
    const CIRCULAR_REF_MSG: &str = "@to on struct circularly references itself";
    fn default_for_circular_ref(span: Span) -> Vec<TokenTree> {
        vec![quote!(null).with_default_span(span).into_token_tree()]
    }
}

impl IntoParseErrorWithSpan for StructToUnspecifiedExpandError {
    fn into_parse_error_with_span(self, span: Span) -> crate::syn_generic::ParseError {
        match self {
            StructToUnspecifiedExpandError::Untagged(e) => e.into_parse_error_with_span(span),
            StructToUnspecifiedExpandError::Tagged(e) => e.into_parse_error_with_span(span),
        }
    }
}
