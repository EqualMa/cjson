use proc_macro::{Span, TokenStream, TokenTree};

use typed_quote::{IntoTokenTree as _, IntoTokens as _, WithSpan as _, quote};

use crate::{ErrorCollector, expand_props::TokensCollector};

use super::{
    IntoParseErrorWithSpan as _, OnlyFieldResult, StructToDefault, StructToDefaultExpandError,
    StructToTaggedDefaultExpandError, TryWithOutSpan as _,
    context_with_prop_name::ContextWithPropName, context_with_prop_tag::ContextWithPropTag,
    context_with_prop_to_default::ContextWithPropToDefault,
};

pub trait ContextWithPropToTaggedDefault:
    Sized + ContextWithPropTag + ContextWithPropName + ContextWithPropToDefault
{
    fn cache_for_to_tagged_default(
        &mut self,
    ) -> &mut Option<(Vec<TokenTree>, Result<(), StructToTaggedDefaultExpandError>)>;

    fn span_to_calc_to_tagged_default(&self) -> Span;

    fn expand_to_tagged_default(
        &mut self,
        out: TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
    ) {
        self.try_with_out_span(out, span, errors, Self::try_expand_to_tagged_default)
    }

    fn try_expand_to_tagged_default(
        &mut self,
        mut out: TokensCollector<'_>,
        _span: Span, // TODO: link @to.tagged_default
    ) -> Result<(), StructToTaggedDefaultExpandError> {
        let (expanded, res) = match self.cache_for_to_tagged_default() {
            Some(v) => v,
            None => {
                let v = self.calc_expand_to_tagged_default();
                self.cache_for_to_tagged_default().insert(v)
            }
        };

        out.extend_from_slice(expanded);

        res.clone()
    }

    fn calc_expand_to_tagged_default(
        &mut self,
    ) -> (Vec<TokenTree>, Result<(), StructToTaggedDefaultExpandError>) {
        let name_span = self.span_to_calc_to_tagged_default();
        let mut object_inner = vec![];
        let mut out = TokensCollector::from(&mut object_inner);

        let expand_tag = self.try_expand_tag(out.as_mut());

        out.push(quote!(=).with_replaced_span(name_span).into_token_tree());

        self.expand_name(out.as_mut(), name_span);

        out.push(quote!(;).with_replaced_span(name_span).into_token_tree());

        let after_tag = match self.get_to_default() {
            StructToDefault::Transparent { span } => {
                let span = span.unwrap_or(name_span);

                match self.context_of_only_field(span, None) {
                    OnlyFieldResult::Existing(mut ctx, only_field) => {
                        out.extend(quote!(..).with_replaced_span(span).into_token_stream());

                        let expand_to = ctx.try_expand_to(out, span).err();

                        match (only_field, expand_to) {
                            (None::<_>, None::<_>) => Ok(()),
                            (only_field, expand_to) => {
                                Err(StructToDefaultExpandError::Transparent {
                                    only_field,
                                    expand_to,
                                })
                            }
                        }
                    }
                    OnlyFieldResult::EmptyFields(only_field) => {
                        Err(StructToDefaultExpandError::Transparent {
                            only_field: Some(only_field),
                            expand_to: None,
                        })
                    }
                }
            }
            StructToDefault::Unit => Ok(()),
            StructToDefault::Object | StructToDefault::Array => {
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

                errors
                    .ok()
                    .map_err(StructToDefaultExpandError::ObjectOrArray)
            }
        };

        let res = match (expand_tag, after_tag) {
            (Ok(()), Ok(())) => Ok(()),
            (expand_tag, after_tag) => Err(StructToTaggedDefaultExpandError {
                expand_tag: expand_tag.err(),
                after_tag: after_tag.err(),
            }),
        };

        let object_inner = TokenStream::from_iter(object_inner);
        let tt = quote!({ #object_inner })
            .with_default_span(name_span)
            .into_token_tree();

        (vec![tt], res)
    }
}
