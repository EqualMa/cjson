use proc_macro::{Span, TokenStream, TokenTree};

use typed_quote::{IntoTokenTree as _, IntoTokens as _, WithSpan as _, quote};

use crate::{ErrorCollector, expand_props::TokensCollector, to_json::ctx::custom::TokensExpanded};

use super::{
    IntoParseErrorWithSpan as _, OnlyFieldResult, StructToDefault, StructToDefaultExpandError,
    TryWithOutSpan as _, only_field::ContextSupportsOnlyField,
};

pub trait CalcToUntaggedDefault: ContextSupportsOnlyField {
    fn get_to_default(&self) -> StructToDefault;
    fn span_to_calc_to_default(&self) -> Span;

    fn calc_expand_to_default(
        &mut self,
    ) -> (Vec<TokenTree>, Result<(), StructToDefaultExpandError>) {
        let name_span = self.span_to_calc_to_default();
        match self.get_to_default() {
            StructToDefault::Transparent { span } => {
                let span = span.unwrap_or(name_span);

                match self.context_of_only_field(span, None) {
                    OnlyFieldResult::Existing(mut ctx, only_field) => {
                        let mut ts = Vec::new();
                        let expand_to = ctx.try_expand_to(From::from(&mut ts), span).err();

                        let res = match (only_field, expand_to) {
                            (None::<_>, None::<_>) => Ok(()),
                            (only_field, expand_to) => {
                                Err(StructToDefaultExpandError::Transparent {
                                    only_field,
                                    expand_to,
                                })
                            }
                        };
                        (ts, res)
                    }
                    OnlyFieldResult::EmptyFields(only_field) => (
                        vec![quote!(null).with_default_span(span).into_token_tree()],
                        Err(StructToDefaultExpandError::Transparent {
                            only_field: Some(only_field),
                            expand_to: None,
                        }),
                    ),
                }
            }
            StructToDefault::Unit => (
                vec![quote!(null).with_default_span(name_span).into_token_tree()],
                Ok(()),
            ),
            StructToDefault::Object => {
                let mut inner = Vec::new();

                let mut out = TokensCollector::from(&mut inner);

                let mut errors = ErrorCollector::default();
                self.for_each_non_skip_field(name_span, |mut ctx| {
                    let span = ctx.field_name_span();
                    out.extend(quote!(..).with_replaced_span(name_span).into_token_stream());
                    match ctx.try_expand_to_kvs(out.as_mut(), span) {
                        Ok(()) => (),
                        Err(e) => errors.push(e.into_parse_error_with_span(span)),
                    }
                    out.push(quote!(;).with_replaced_span(name_span).into_token_tree());
                });

                let inner = TokenStream::from_iter(inner);
                let tt = quote!({ #inner }).with_default_span(name_span);

                (
                    vec![tt.into_token_tree()],
                    errors
                        .ok()
                        .map_err(StructToDefaultExpandError::ObjectOrArray),
                )
            }
            StructToDefault::Array => {
                let mut inner = Vec::new();

                let mut out = TokensCollector::from(&mut inner);

                let mut errors = ErrorCollector::default();
                self.for_each_non_skip_field(name_span, |mut ctx| {
                    let span = ctx.field_name_span();
                    out.extend(quote!(..).with_replaced_span(name_span).into_token_stream());
                    match ctx.try_expand_to_items(out.as_mut(), span) {
                        Ok(()) => (),
                        Err(e) => errors.push(e.into_parse_error_with_span(span)),
                    }
                    out.push(quote!(,).with_replaced_span(name_span).into_token_tree());
                });

                let inner = TokenStream::from_iter(inner);
                let tt = quote!([ #inner ]).with_default_span(name_span);

                (
                    vec![tt.into_token_tree()],
                    errors
                        .ok()
                        .map_err(StructToDefaultExpandError::ObjectOrArray),
                )
            }
        }
    }
}

pub trait ContextWithPropToDefault: Sized + CalcToUntaggedDefault {
    fn cache_for_to_untagged_default(
        &mut self,
    ) -> &mut Option<TokensExpanded<StructToDefaultExpandError>>;

    fn expand_to_default(
        &mut self,
        out: TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
    ) {
        self.try_with_out_span(out, span, errors, Self::try_expand_to_default)
    }

    fn try_expand_to_default(
        &mut self,
        mut out: TokensCollector<'_>,
        _span: Span, // TODO: link @to.default
    ) -> Result<(), StructToDefaultExpandError> {
        let (ts, res) = match self.cache_for_to_untagged_default() {
            Some(v) => v,
            None => {
                let v = self.calc_expand_to_default();
                self.cache_for_to_untagged_default().insert(v)
            }
        };

        out.extend_from_slice(ts);
        res.clone()
    }
}
