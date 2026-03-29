use proc_macro::{Ident, Literal, Span, TokenStream, TokenTree};
use typed_quote::{IntoTokenTree as _, IntoTokens, WithSpan as _, quote};

use crate::{
    ErrorCollector,
    expand_props::{self, PropPath, TokensCollector},
    ident_match,
    syn_generic::parse_meta_utils::MetaPathSpanWith,
    to_json::{ctx::TryWithOutSpan, item::Rename},
};

use super::{
    CustomTokens, CustomTokensExpandErrorOr, PropExpanded, StructField,
    StructFieldExpandIndexToStrError, StructFieldExpandNameError, StructFieldExpandToDefaultError,
    StructFieldExpandToError, StructFieldExpandToItemsDefaultError, StructFieldExpandToItemsError,
    StructFieldExpandToKvsDefaultError, StructFieldExpandToKvsError, StructFieldToItemsDefault,
    StructFieldToKvsDefault, make_fn_clone_and_set_span,
};

pub trait ContextSupportsField {
    type FieldHelper<'a>: FieldHelper
    where
        Self: 'a;
    fn field_helper(&mut self, index_field: usize) -> Self::FieldHelper<'_>;

    fn field(&self, index_field: usize) -> &StructField;
    fn field_mut(&mut self, index_field: usize) -> &mut StructField;

    fn try_expand_prop_at_field(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) -> Result<(), PropPath>;
}

impl<Ctx: ?Sized + ContextSupportsField> ContextSupportsField for &mut Ctx {
    type FieldHelper<'a>
        = Ctx::FieldHelper<'a>
    where
        Self: 'a;

    fn field_helper(&mut self, index_field: usize) -> Self::FieldHelper<'_> {
        Ctx::field_helper(self, index_field)
    }

    fn field(&self, index_field: usize) -> &StructField {
        Ctx::field(self, index_field)
    }

    fn field_mut(&mut self, index_field: usize) -> &mut StructField {
        Ctx::field_mut(self, index_field)
    }

    fn try_expand_prop_at_field(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) -> Result<(), PropPath> {
        Ctx::try_expand_prop_at_field(self, prop, out, errors)
    }
}

pub trait FieldHelper {
    fn to_calc_name(&mut self) -> CalcName<'_>;
    fn to_calc_expr(&mut self) -> CalcExpr<'_>;
}

pub struct ContextOfField<Ctx: ContextSupportsField> {
    pub(super) ctx_struct: Ctx,
    pub(super) index_field: usize,
    pub(super) span: Span,
    pub(super) span_self: Option<Span>,
}

impl<'a, Ctx: ?Sized + ContextSupportsField> ContextOfField<&'a mut &mut Ctx> {
    pub(super) fn mut_deref(self) -> ContextOfField<&'a mut Ctx> {
        let Self {
            ctx_struct,
            index_field,
            span,
            span_self,
        } = self;
        ContextOfField {
            ctx_struct,
            index_field,
            span,
            span_self,
        }
    }
}

impl<Ctx: ContextSupportsField> ContextOfField<Ctx> {
    pub(super) fn as_mut(&mut self) -> ContextOfField<&mut Ctx> {
        let Self {
            ref mut ctx_struct,
            index_field,
            span,
            span_self,
        } = *self;
        ContextOfField {
            ctx_struct,
            index_field,
            span,
            span_self,
        }
    }

    fn field_helper(&mut self) -> Ctx::FieldHelper<'_> {
        self.ctx_struct.field_helper(self.index_field)
    }

    fn field(&self) -> &StructField {
        self.ctx_struct.field(self.index_field)
    }

    fn field_mut(&mut self) -> &mut StructField {
        self.ctx_struct.field_mut(self.index_field)
    }
}

impl<Ctx: ContextSupportsField> ContextOfField<Ctx> {
    pub fn expand_field_props_maybe_empty(
        &mut self,
        mut rest_prop: std::vec::IntoIter<expand_props::Prop>,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let Some(first_prop) = rest_prop.next() else {
            errors.push_custom("this property cannot expand to tokens", self.span);
            return;
        };
        self.expand_field_prop((first_prop, rest_prop), out, errors)
    }

    pub fn expand_field_prop(
        &mut self,
        (first_prop, rest_prop): (expand_props::Prop, std::vec::IntoIter<expand_props::Prop>),
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let first_ident_span;

        enum FirstIdentType {
            Name,
            To,
            IndexToStr,
            NameOrIndexToStr,
            Expr,
            Type,
            ToKvs,
            ToItems,
        }
        let first_ident_type = 'first: {
            let err_span = match first_prop {
                expand_props::Prop::Ident(ident) => {
                    first_ident_span = ident.span();
                    ident_match!(match ident {
                        b"name" => break 'first FirstIdentType::Name,
                        b"to" => break 'first FirstIdentType::To,
                        b"index_to_str" => break 'first FirstIdentType::IndexToStr,
                        b"name_or_index_to_str" => break 'first FirstIdentType::NameOrIndexToStr,
                        b"expr" => break 'first FirstIdentType::Expr,
                        b"type" => break 'first FirstIdentType::Type,
                        b"to_kvs" => break 'first FirstIdentType::ToKvs,
                        b"to_items" => break 'first FirstIdentType::ToItems,
                        _ => ident.span(),
                    })
                }
                expand_props::Prop::Literal(literal) => literal.span(),
            };
            errors.push_custom("property not defined on field of struct", err_span);
            return;
        };

        let mut forbid_rest_prop = || {
            if let Some(prop) = rest_prop.as_slice().first() {
                errors.push_custom("property not defined", prop.span());
            }
        };

        match first_ident_type {
            FirstIdentType::Name => {
                forbid_rest_prop();
                self.expand_name(out, first_ident_span, errors)
            }
            FirstIdentType::To => match match_ident_default(rest_prop, errors) {
                Some(span) => self.expand_default_to(span, out),
                None => self.expand_to(out, first_ident_span, errors),
            },
            FirstIdentType::IndexToStr => {
                forbid_rest_prop();
                self.expand_index_to_str(out, first_ident_span, errors)
            }
            FirstIdentType::NameOrIndexToStr => {
                forbid_rest_prop();
                self.expand_name_or_index_to_str(out, first_ident_span)
            }
            FirstIdentType::Expr => {
                forbid_rest_prop();
                self.expand_expr(out, self.span, self.span_self)
            }
            FirstIdentType::Type => {
                forbid_rest_prop();
                self.expand_type(out)
            }
            FirstIdentType::ToKvs => match match_ident_default(rest_prop, errors) {
                Some(span) => self.expand_to_kvs_default(out, span, errors),
                None => self.expand_to_kvs(out, first_ident_span, errors),
            },
            FirstIdentType::ToItems => match match_ident_default(rest_prop, errors) {
                Some(span) => self.expand_to_items_default(out, span, errors),
                None => self.expand_to_items(out, first_ident_span, errors),
            },
        }
    }

    fn expand_name(
        &mut self,
        out: expand_props::TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
    ) {
        let res = self.try_expand_name(out, span);

        if let Err(e) = res {
            errors.push_custom(e.to_msg(), span);
        }
    }

    fn try_expand_name(
        &mut self,
        mut out: expand_props::TokensCollector<'_>,
        span: Span,
    ) -> Result<(), StructFieldExpandNameError> {
        let (ex, res) = match &mut self.field_mut().expanded_name {
            Some(ex) => ex,
            None => {
                let ts = self.field_helper().to_calc_name().calc();
                self.field_mut().expanded_name.insert(ts)
            }
        };

        out.extend(ex.iter().map(make_fn_clone_and_set_span(span)));

        *res
    }

    fn expand_expr(&mut self, mut out: TokensCollector<'_>, span: Span, span_self: Option<Span>) {
        let inner = self.field_helper().to_calc_expr().expand(span, span_self);

        let tt = quote!( (#inner) ).with_default_span(span).into_token_tree();

        out.push(tt);
    }

    fn expand_type(&self, mut out: TokensCollector<'_>) {
        let ts = self.field().type_.as_slice();
        out.extend_from_slice(ts);
    }

    fn expand_index_to_str(
        &mut self,
        out: TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
    ) {
        self.try_with_out_span(out, span, errors, Self::try_expand_index_to_str);
    }

    fn try_expand_index_to_str(
        &mut self,
        mut out: TokensCollector<'_>,
        span: Span,
    ) -> Result<(), StructFieldExpandIndexToStrError> {
        let (expanded_index_to_str, res) = match &mut self.field_mut().expanded_index_to_str {
            Some(v) => v,
            None => {
                let ts = self.calc_expand_index_to_str();

                self.field_mut().expanded_index_to_str.insert(ts)
            }
        };

        out.extend(
            expanded_index_to_str
                .iter()
                .map(|tt| tt.clone().with_replaced_span(span)),
        );

        *res
    }

    fn calc_expand_index_to_str(
        &mut self,
    ) -> (Vec<TokenTree>, Result<(), StructFieldExpandIndexToStrError>) {
        let res = match &self.field().name {
            typed_quote::Either::A(_) => Err(StructFieldExpandIndexToStrError),
            typed_quote::Either::B(index) => {
                let _: &Literal = index;
                Ok(())
            }
        };

        let lit = Literal::string(self.field_index_to_str());

        (vec![lit.into()], res)
    }

    fn field_index_to_str(&mut self) -> &str {
        let index_field = self.index_field;
        self.field_mut()
            .calc_index_to_str
            .get_or_insert(index_field)
    }

    fn expand_name_or_index_to_str(&mut self, mut out: TokensCollector<'_>, span: Span) {
        match self.try_expand_name(out.as_mut(), span) {
            Ok(()) => {}
            Err(_) => {
                self.try_expand_index_to_str(out, span)
                    .expect("unnamed field");
            }
        }
    }

    fn expand_to(&mut self, out: TokensCollector<'_>, span: Span, errors: &mut ErrorCollector) {
        self.try_with_out_span(out, span, errors, Self::try_expand_to);
    }

    pub fn try_expand_to(
        &mut self,
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @to
    ) -> Result<(), StructFieldExpandToError> {
        PropExpanded::try_expand(
            self,
            |this| &mut this.field_mut().to,
            Self::calc_expand_to,
            out,
        )
    }

    fn calc_expand_to(&mut self) -> (Vec<TokenTree>, Result<(), StructFieldExpandToError>) {
        let (ts, res) = CustomTokens::take_and_expand::<Self, StructFieldExpandToDefaultError>(
            self,
            |ctx| &mut ctx.field_mut().to.value,
            |ctx, out| {
                let span = ctx.field().name_span();
                ctx.expand_default_to(span, out);
                Ok(())
            },
        );

        let res = match res {
            Ok(()) => Ok(()),
            Err(e) => match e {
                CustomTokensExpandErrorOr::Custom(e) => Err(e.into()),
                CustomTokensExpandErrorOr::Other(e) => match e {},
            },
        };

        (ts, res)
    }

    fn expand_default_to(&mut self, span: Span, mut out: TokensCollector<'_>) {
        self.expand_expr(out.as_mut(), span, None);
        out.extend(
            quote!(as &'cjson_lt_to_json)
                .with_replaced_span(span)
                .into_token_stream(),
        );
        self.expand_type(out);
    }

    fn expand_to_kvs_default(
        &mut self,
        out: TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
    ) {
        self.try_with_out_span(out, span, errors, |this, out, span| {
            this.try_expand_to_kvs_default(out, Some(span))
        })
    }

    fn try_expand_to_kvs_default(
        &mut self,
        out: TokensCollector<'_>,
        _span: Option<Span>, // TODO: link @to_kvs.default
    ) -> Result<(), StructFieldExpandToKvsDefaultError> {
        PropExpanded::try_expand(
            self,
            |this| &mut this.field_mut().to_kvs.default,
            Self::calc_expand_to_kvs_default,
            out,
        )
    }

    fn calc_expand_to_kvs_default(
        &mut self,
    ) -> (
        Vec<TokenTree>,
        Result<(), StructFieldExpandToKvsDefaultError>,
    ) {
        let this = self.field_mut();

        match this.to_kvs.default.value {
            StructFieldToKvsDefault::BracedNameEqTo => {
                let mut inner = Vec::new();
                let mut out = TokensCollector::from(&mut inner);
                let span = this.name_span();
                let expand_name = self.try_expand_name(out.as_mut(), span).err();
                out.push(quote!(=).with_replaced_span(span).into_token_tree());
                let expand_to = self.try_expand_to(out.as_mut(), span).err();

                let inner = TokenStream::from_iter(inner);
                let tt = quote!({ #inner }).with_default_span(span).into_token_tree();
                let res = match (expand_name, expand_to) {
                    (None::<_>, None::<_>) => Ok(()),
                    (expand_name, expand_to) => Err(StructFieldExpandToKvsDefaultError::NameEqTo {
                        expand_name,
                        expand_to,
                    }),
                };
                (vec![tt], res)
            }
            StructFieldToKvsDefault::Flatten { span } => {
                match &this.name {
                    typed_quote::Either::A(ident) => {
                        let _: &Ident = ident;
                    }
                    typed_quote::Either::B(lit) => {
                        let _: &Literal = lit;
                        return (
                            vec![quote!({}).with_default_span(span).into_token_tree()],
                            Err(StructFieldExpandToKvsDefaultError::FlattenOnUnnamedField),
                        );
                    }
                }
                let mut ts = Vec::new();
                self.expand_default_to(span, From::from(&mut ts));
                (ts, Ok(()))
            }
        }
    }

    fn expand_to_kvs(&mut self, out: TokensCollector<'_>, span: Span, errors: &mut ErrorCollector) {
        self.try_with_out_span(out, span, errors, Self::try_expand_to_kvs)
    }

    pub fn try_expand_to_kvs(
        &mut self,
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @to_kvs
    ) -> Result<(), StructFieldExpandToKvsError> {
        PropExpanded::try_expand(
            self,
            |this| &mut this.field_mut().to_kvs.custom,
            Self::calc_expand_to_kvs,
            out,
        )
    }

    fn calc_expand_to_kvs(&mut self) -> (Vec<TokenTree>, Result<(), StructFieldExpandToKvsError>) {
        CustomTokens::take_and_expand(
            self,
            |this| &mut this.field_mut().to_kvs.custom.value,
            |this, out| this.try_expand_to_kvs_default(out, None),
        )
    }

    fn expand_to_items_default(
        &mut self,
        out: TokensCollector<'_>,
        span: Span, // TODO: link @to_items.default
        errors: &mut ErrorCollector,
    ) {
        self.try_with_out_span(out, span, errors, |this, out, span| {
            this.try_expand_to_items_default(out, Some(span))
        })
    }

    fn try_expand_to_items_default(
        &mut self,
        out: TokensCollector<'_>,
        _span: Option<Span>,
    ) -> Result<(), StructFieldExpandToItemsDefaultError> {
        PropExpanded::try_expand(
            self,
            |this| &mut this.field_mut().to_items.default,
            Self::calc_expand_to_items_default,
            out,
        )
    }

    fn calc_expand_to_items_default(
        &mut self,
    ) -> (
        Vec<TokenTree>,
        Result<(), StructFieldExpandToItemsDefaultError>,
    ) {
        let this = self.field_mut();

        match this.to_items.default.value {
            StructFieldToItemsDefault::BracketedTo => {
                let mut inner = Vec::new();
                let span = this.name_span();
                let expand_to = self.try_expand_to(From::from(&mut inner), span).err();

                let inner = TokenStream::from_iter(inner);
                let tt = quote!( [#inner] ).with_default_span(span).into_token_tree();
                let res = match expand_to {
                    Some(e) => Err(StructFieldExpandToItemsDefaultError::To(e)),
                    None => Ok(()),
                };
                (vec![tt], res)
            }
            StructFieldToItemsDefault::Flatten { span } => {
                match &this.name {
                    typed_quote::Either::A(ident) => {
                        let _: &Ident = ident;
                        return (
                            vec![quote!([]).with_default_span(span).into_token_tree()],
                            Err(StructFieldExpandToItemsDefaultError::FlattenOnNamedField),
                        );
                    }
                    typed_quote::Either::B(lit) => {
                        let _: &Literal = lit;
                    }
                }
                let mut ts = Vec::new();
                self.expand_default_to(span, From::from(&mut ts));
                (ts, Ok(()))
            }
        }
    }

    fn expand_to_items(
        &mut self,
        out: TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
    ) {
        self.try_with_out_span(out, span, errors, Self::try_expand_to_items)
    }

    pub fn try_expand_to_items(
        &mut self,
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @to_items
    ) -> Result<(), StructFieldExpandToItemsError> {
        PropExpanded::try_expand(
            self,
            |this| &mut this.field_mut().to_items.custom,
            Self::calc_expand_to_items,
            out,
        )
    }

    fn calc_expand_to_items(
        &mut self,
    ) -> (Vec<TokenTree>, Result<(), StructFieldExpandToItemsError>) {
        CustomTokens::take_and_expand(
            self,
            |ctx| &mut ctx.field_mut().to_items.custom.value,
            |ctx, out| ctx.try_expand_to_items_default(out, None),
        )
    }
}

fn match_ident_default(
    mut rest_prop: std::vec::IntoIter<expand_props::Prop>,
    errors: &mut ErrorCollector,
) -> Option<Span> {
    let res = match rest_prop.next() {
        Some(expand_props::Prop::Ident(ident)) => ident_match!(match ident {
            b"default" => Ok(ident.span()),
            _ => Err(ident.span()),
        }),
        Some(expand_props::Prop::Literal(lit)) => Err(lit.span()),
        None => return None,
    };

    let span_of_default = match res {
        Ok(span_of_default) => span_of_default,
        Err(err_span) => {
            errors.push_custom("property not defined", err_span);
            return None;
        }
    };

    if let Some(prop) = rest_prop.as_slice().first() {
        errors.push_custom("property not defined", prop.span());
    }

    Some(span_of_default)
}

type FieldName = typed_quote::Either<Ident, Literal>;

pub struct CalcName<'a> {
    pub options: &'a super::Options,
    pub rename: Option<&'a MetaPathSpanWith<Rename>>,
    pub name: &'a FieldName,
}

impl<'a> CalcName<'a> {
    fn calc(self) -> (Vec<TokenTree>, Result<(), StructFieldExpandNameError>) {
        let Self {
            options,
            rename,
            name,
        } = self;

        let res;

        let ts = if let Some(MetaPathSpanWith(rename_span, rename)) = rename {
            res = Ok(());
            rename.to_tokens_as_json_object_key(
                //
                &options.crate_path,
                *rename_span,
                name,
            )
        } else {
            let lit = match name {
                typed_quote::Either::A(name) => {
                    res = Ok(());
                    crate::utils::ident_to_literal_string(name)
                }
                typed_quote::Either::B(index) => {
                    res = Err(StructFieldExpandNameError);
                    Literal::string("").with_replaced_span(index.span())
                }
            };

            vec![lit.into()]
        };

        (ts, res)
    }
}

pub enum CalcExpr<'a> {
    RefSelfDot {
        ref_self_dot: &'a [TokenTree],
        name: FieldName,
    },
    PatternDestruct(&'a Ident),
}

impl<'a> CalcExpr<'a> {
    fn expand(self, span: Span, span_self: Option<Span>) -> TokenStream {
        match self {
            CalcExpr::RefSelfDot { ref_self_dot, name } => {
                let ref_self_dot: TokenStream = if let Some(span_self) = span_self {
                    ref_self_dot
                        .iter()
                        .map(make_fn_clone_and_set_span(span_self))
                        .collect()
                } else {
                    ref_self_dot.iter().cloned().collect()
                };
                let field_name = name.with_replaced_span(span);

                quote! (#ref_self_dot #field_name).into_token_stream()
            }
            CalcExpr::PatternDestruct(ident) => ident.with_replaced_span(span).into_token_stream(),
        }
    }
}

impl<Ctx: ContextSupportsField> expand_props::Context for ContextOfField<Ctx> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        errors: &mut ErrorCollector,
    ) -> impl use<'a, Ctx> + expand_props::ContextAtBracketStar {
        errors.push_custom("field doesn't support `@[...]*`", star_span);

        expand_props::ErroredContext
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a, Ctx> + expand_props::Context> {
        errors.push_custom("field doesn't support `@[...]?`", question_span);

        None::<expand_props::NeverContext>
    }

    fn expand_prop(
        &mut self,
        prop: PropPath,
        mut out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let Err(PropPath(first_prop, rest_prop)) =
            self.ctx_struct
                .try_expand_prop_at_field(prop, out.as_mut(), errors)
        else {
            return;
        };
        self.expand_field_prop((first_prop, rest_prop.into_iter()), out, errors)
    }
}
