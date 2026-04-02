use proc_macro::{Span, TokenStream, TokenTree};

use typed_quote::{IntoTokenTree as _, IntoTokens as _, WithSpan as _, quote};

use crate::{
    ErrorCollector,
    expand_props::{self, TokensCollector},
    to_json::ctx::HasConstCircularRefMsg,
};

use super::{
    CustomTokensExpandErrorOr, IntoParseErrorWithSpan, OnlyFieldResult, StructToDefault,
    StructToDefaultExpandError, StructToTaggedDefaultExpandError, TryWithOutSpan as _,
    context_with_prop_name::ContextWithPropName,
    context_with_prop_tag::ContextWithPropTag,
    context_with_prop_to_default::CalcToUntaggedDefault,
    custom::{PropDefaultCustom, TokensExpanded},
};

pub trait CalcToTaggedKvsDefault: CalcToUntaggedDefault {
    fn span_to_calc_to_tagged_kvs_default(&self) -> Span;

    fn calc_to_tagged_kvs_default(
        &mut self,
    ) -> TokensExpanded<StructToTaggedKvsDefaultExpandError> {
        calc_to_tagged_kvs_default(self)
    }
}

pub trait CalcToInternallyTaggedDefaultWith: ContextWithPropTag + ContextWithPropName {
    fn calc_to_internally_tagged_default_with(
        &mut self,
        name_span: Span,
        try_expand_to_tagged_kvs: impl FnOnce(
            &mut Self,
            TokensCollector<'_>,
            Span,
        ) -> Result<
            (),
            CustomTokensExpandErrorOr<StructToTaggedKvsDefaultExpandError>,
        >,
    ) -> (TokenTree, Result<(), StructToTaggedDefaultExpandError>)
    where
        Self: expand_props::Context,
    {
        let mut object_inner = vec![];
        let mut out = TokensCollector::from(&mut object_inner);

        let expand_tag = self.try_expand_tag(out.as_mut());

        out.push(quote!(=).with_replaced_span(name_span).into_token_tree());

        self.expand_name(out.as_mut(), name_span);

        out.push(quote!(;).with_replaced_span(name_span).into_token_tree());

        out.extend(quote!(..).with_replaced_span(name_span).into_token_stream());

        let after_tag = try_expand_to_tagged_kvs(self, out, name_span);

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

        (tt, res)
    }
}

impl<T: ?Sized + ContextWithPropTag + ContextWithPropName> CalcToInternallyTaggedDefaultWith for T {}

#[derive(Default)]
pub struct CacheForToInternallyTaggedDefault(
    Option<(TokenTree, Result<(), StructToTaggedDefaultExpandError>)>,
);

pub trait ToInternallyTaggedDefaultWith:
    CalcToInternallyTaggedDefaultWith + CalcToTaggedKvsDefault
{
    fn cache_for_to_internally_tagged_default(&mut self) -> &mut CacheForToInternallyTaggedDefault;

    fn to_internally_tagged_default_with(
        &mut self,
        span: Span,
        try_expand_to_tagged_kvs: impl FnOnce(
            &mut Self,
            TokensCollector<'_>,
            Span,
        ) -> Result<
            (),
            CustomTokensExpandErrorOr<StructToTaggedKvsDefaultExpandError>,
        >,
    ) -> (&TokenTree, Result<(), StructToTaggedDefaultExpandError>)
    where
        Self: expand_props::Context,
    {
        if self.cache_for_to_internally_tagged_default().0.is_none() {
            let v = self.calc_to_internally_tagged_default_with(span, try_expand_to_tagged_kvs);
            self.cache_for_to_internally_tagged_default().0 = Some(v);
        }

        let (expanded, res) = self
            .cache_for_to_internally_tagged_default()
            .0
            .as_ref()
            .unwrap();
        (expanded, res.clone())
    }
}

pub trait ContextWithPropToTaggedDefault: Sized + ToInternallyTaggedDefaultWith {
    fn prop_to_tagged_kvs(
        &mut self,
    ) -> &mut PropDefaultCustom<(
        Vec<TokenTree>,
        Result<(), StructToTaggedKvsDefaultExpandError>,
    )>;

    fn span_to_calc_to_tagged_default(&self) -> Span;

    fn expand_to_tagged_default(
        &mut self,
        out: TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
    ) where
        Self: expand_props::Context,
    {
        self.try_with_out_span(out, span, errors, Self::try_expand_to_tagged_default)
    }

    fn try_expand_to_tagged_default(
        &mut self,
        mut out: TokensCollector<'_>,
        _span: Span, // TODO: link @to.tagged_default
    ) -> Result<(), StructToTaggedDefaultExpandError>
    where
        Self: expand_props::Context,
    {
        let span = self.span_to_calc_to_tagged_default();
        let (expanded, res) = self.to_internally_tagged_default_with(span, |this, out, span| {
            this.try_expand_to_tagged_kvs(out, span)
        });

        out.push(expanded.clone());
        res
    }

    fn try_expand_to_tagged_kvs(
        &mut self,
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @(to.tagged_kvs)
    ) -> Result<(), CustomTokensExpandErrorOr<StructToTaggedKvsDefaultExpandError>>
    where
        Self: expand_props::Context,
    {
        PropDefaultCustom::expand_custom_or_default(
            self,
            Self::prop_to_tagged_kvs,
            calc_to_tagged_kvs_default,
            out,
        )
    }

    fn try_expand_to_tagged_kvs_default(
        &mut self,
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @(to.tagged_kvs)
    ) -> Result<(), StructToTaggedKvsDefaultExpandError> {
        PropDefaultCustom::expand_default(
            self,
            Self::prop_to_tagged_kvs,
            calc_to_tagged_kvs_default,
            out,
        )
    }
}

#[derive(Clone)]
pub struct StructToTaggedKvsDefaultExpandError(super::StructToDefaultExpandError);

pub type StructToTaggedKvsExpandError =
    CustomTokensExpandErrorOr<StructToTaggedKvsDefaultExpandError>;

impl HasConstCircularRefMsg for StructToTaggedKvsDefaultExpandError {
    const CIRCULAR_REF_MSG: &str = "@to_tagged_kvs on struct circularly references itself";

    fn default_for_circular_ref(span: Span) -> Vec<TokenTree> {
        let tt = quote!({}).with_replaced_span(span).into_token_tree();
        vec![tt]
    }
}

impl IntoParseErrorWithSpan for StructToTaggedKvsDefaultExpandError {
    fn into_parse_error_with_span(self, span: Span) -> crate::syn_generic::ParseError {
        self.0.into_parse_error_with_span(span)
    }
}

fn calc_to_tagged_kvs_default(
    ctx: &mut (impl ?Sized + CalcToTaggedKvsDefault),
) -> TokensExpanded<StructToTaggedKvsDefaultExpandError> {
    let name_span = ctx.span_to_calc_to_tagged_kvs_default();

    let (object_inner, res) = match ctx.get_to_default() {
        StructToDefault::Transparent { span } => {
            let span = span.unwrap_or(name_span);

            let mut ts = Vec::new();
            let out = TokensCollector::from(&mut ts);

            let res = match ctx.context_of_only_field(span, None) {
                OnlyFieldResult::Existing(mut ctx, only_field) => {
                    let expand_to = ctx.try_expand_to(out, span).err();

                    match (only_field, expand_to) {
                        (None::<_>, None::<_>) => Ok(()),
                        (only_field, expand_to) => Err(StructToDefaultExpandError::Transparent {
                            only_field,
                            expand_to,
                        }),
                    }
                }
                OnlyFieldResult::EmptyFields(only_field) => {
                    Err(StructToDefaultExpandError::Transparent {
                        only_field: Some(only_field),
                        expand_to: None,
                    })
                }
            };

            return (ts, res.map_err(StructToTaggedKvsDefaultExpandError));
        }
        StructToDefault::Unit => (TokenStream::new(), Ok(())),
        StructToDefault::Object | StructToDefault::Array => {
            let mut object_inner = Vec::new();
            let mut out = TokensCollector::from(&mut object_inner);
            let mut errors = ErrorCollector::default();
            ctx.for_each_non_skip_field(name_span, |mut ctx| {
                let span = ctx.field_name_span();
                out.extend(quote!(..).with_replaced_span(name_span).into_token_stream());
                match ctx.try_expand_to_kvs(out.as_mut(), span) {
                    Ok(()) => (),
                    Err(e) => errors.push(e.into_parse_error_with_span(span)),
                }
                out.push(quote!(;).with_replaced_span(name_span).into_token_tree());
            });

            let res = errors
                .ok()
                .map_err(StructToDefaultExpandError::ObjectOrArray);
            (TokenStream::from_iter(object_inner), res)
        }
    };

    let tt = quote!( {#object_inner} )
        .with_default_span(name_span)
        .into_token_tree();
    (vec![tt], res.map_err(StructToTaggedKvsDefaultExpandError))
}
