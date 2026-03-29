use proc_macro::{Span, TokenTree};

use crate::{
    ErrorCollector,
    expand_props::{Context, TokensCollector},
};

use super::{
    CustomTokens, PropExpanded, PropExpandedWithErr, StructToDefaultExpandError,
    StructToExpandError, TryWithOutSpan as _,
};

pub trait ContextWithPropTo: Sized + Context {
    fn prop_custom_to(
        &mut self,
    ) -> &mut PropExpandedWithErr<Option<CustomTokens>, StructToExpandError>;

    fn calc_to_no_custom(
        &mut self,
        out: TokensCollector<'_>,
    ) -> Result<(), StructToDefaultExpandError>;

    fn expand_to(&mut self, out: TokensCollector<'_>, span: Span, errors: &mut ErrorCollector) {
        self.try_with_out_span(out, span, errors, Self::try_expand_to)
    }

    fn try_expand_to(
        &mut self,
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @to
    ) -> Result<(), StructToExpandError> {
        PropExpanded::try_expand(
            //
            self,
            Self::prop_custom_to,
            Self::calc_expand_to,
            out,
        )
    }

    fn calc_expand_to(&mut self) -> (Vec<TokenTree>, Result<(), StructToExpandError>) {
        CustomTokens::take_and_expand::<_, StructToDefaultExpandError>(
            self,
            |ctx| &mut ctx.prop_custom_to().value,
            Self::calc_to_no_custom,
        )
    }
}
