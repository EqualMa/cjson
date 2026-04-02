use std::collections::HashMap;

use proc_macro::{Ident, Literal, Span, TokenStream, TokenTree};
use typed_quote::{IntoTokenTree, IntoTokens, WithSpan, quote};

use crate::{
    ErrorCollector,
    expand_props::{self, Context, ContextAtBracketStar, TokensCollector},
    ident_match, ident_matches,
    syn_generic::{GroupParen, ParseError, parse_meta_utils::MetaPathSpanWith},
    to_json::{
        ctx::{
            CustomTokensExpandErrorOr, EqValue, HasConstCircularRefMsg, PropExpanded,
            PropExpandedWithErr,
            bracket_star::ContextSupportsAtBracketStar,
            context_with_prop_to::StructToUnspecifiedExpandError,
            context_with_prop_to_tagged_default::ToInternallyTaggedDefaultWith,
            custom::{CustomTokensExpanded, TokensExpanded},
            field::ContextSupportsField,
            only_field::ContextSupportsOnlyField,
        },
        item::Rename,
    },
};

use super::{
    CustomTokens, CustomTokensExpandError, IntoParseErrorWithSpan, OnlyFieldResult, StructField,
    StructToDefault, StructToDefaultExpandError, StructToTaggedDefaultExpandError, bracket_star,
    context_with_fields::ContextWithFields,
    context_with_prop_name::{self, ContextWithPropName},
    context_with_prop_tag::{CacheForDummyTag, ContextPropTagMut, ContextWithPropTag},
    context_with_prop_to::{ContextWithPropTo, StructToExpandError},
    context_with_prop_to_default::{CalcToUntaggedDefault, ContextWithPropToDefault},
    context_with_prop_to_tagged_default::{
        CacheForToInternallyTaggedDefault, CalcToTaggedKvsDefault, ContextWithPropToTaggedDefault,
        StructToTaggedKvsDefaultExpandError, StructToTaggedKvsExpandError,
    },
    custom::PropDefaultCustom,
    field,
    non_field::ContextSupportsNonFieldProp,
    only_field,
};

mod discriminant;
mod inherit;
mod inherit_or_custom;

/// https://doc.rust-lang.org/reference/items/enumerations.html
///
/// Default tag mode:
/// - For unit variant: TagOnly
/// - Otherwise: ExternallyTagged
#[derive(Clone, Copy)]
pub enum TagMode {
    // @value
    Untagged,

    // @discriminant_or_name
    TagOnly,

    // { @name = @value }
    ExternallyTagged,

    // @(value.tagged_default)
    InternallyTagged,

    // { @tag = @name; @(@content = @value;)? }
    // For unit struct without explicit `value()`, `@()?` expands to empty.
    AdjacentlyTagged,
}

#[derive(Clone, Copy)]
pub struct SpecifiedTagMode {
    span: Span,
    mode: TagMode,
}

impl SpecifiedTagMode {
    pub fn new(span: Span, mode: TagMode) -> Self {
        Self { span, mode }
    }
}

pub struct MakeContextOfEnum {
    pub name: Ident,
    pub rename: Option<MetaPathSpanWith<Rename>>,
    pub rename_variants: Option<MetaPathSpanWith<GroupParen>>,
    pub rename_fields: Option<MetaPathSpanWith<GroupParen>>,
    pub specified_tag_mode: Option<SpecifiedTagMode>,
    pub tag: Option<MetaPathSpanWith<EqValue>>,
    pub content: Option<MetaPathSpanWith<EqValue>>,
    pub to: Option<CustomTokensNotCalculating>,
    pub to_untagged: Option<CustomTokensNotCalculating>,
    pub to_tagged_kvs: Option<CustomTokensNotCalculating>,
    pub variants: Vec<EnumVariant>,
    pub options: super::Options,
}

impl From<MakeContextOfEnum> for ContextOfEnum {
    fn from(
        MakeContextOfEnum {
            name,
            rename,
            rename_variants,
            rename_fields,
            specified_tag_mode,
            tag,
            content,
            to,
            to_untagged,
            to_tagged_kvs,
            variants,
            options,
        }: MakeContextOfEnum,
    ) -> Self {
        Self {
            name,
            cache_for_name: Default::default(),
            rename: MaybeAccessed::new(rename),
            rename_variants: MaybeAccessed::new(
                rename_variants.map(|MetaPathSpanWith(span, paren)| {
                    MetaPathSpanWith(span, Rename::Paren(paren))
                }),
            ),
            rename_fields: MaybeAccessed::new(
                rename_fields.map(|MetaPathSpanWith(span, paren)| {
                    MetaPathSpanWith(span, Rename::Paren(paren))
                }),
            ),
            specified_tag_mode: MaybeAccessed::new(specified_tag_mode),
            tag: MaybeAccessed::new(tag),
            cache_for_dummy_tag: Default::default(),
            content: MaybeAccessed::new(content),
            cache_for_dummy_content: Default::default(),
            to: MaybeAccessed::new(to),
            to_untagged: MaybeAccessed::new(to_untagged),
            to_tagged_kvs: MaybeAccessed::new(to_tagged_kvs),
            variants,
            options,
        }
    }
}

pub struct ContextOfEnum {
    name: Ident,
    cache_for_name: Option<Vec<TokenTree>>,

    rename: MaybeAccessed<Option<MetaPathSpanWith<Rename>>>,

    /// Asserts [Rename::Paren]
    rename_variants: MaybeAccessed<Option<MetaPathSpanWith<Rename>>>,
    /// Asserts [Rename::Paren]
    rename_fields: MaybeAccessed<Option<MetaPathSpanWith<Rename>>>,

    specified_tag_mode: MaybeAccessed<Option<SpecifiedTagMode>>,

    tag: MaybeAccessed<Option<MetaPathSpanWith<EqValue>>>,
    cache_for_dummy_tag: CacheForDummyTag,
    content: MaybeAccessed<Option<MetaPathSpanWith<EqValue>>>,
    cache_for_dummy_content: CacheForDummyTag,

    to: MaybeAccessed<Option<CustomTokensNotCalculating>>,
    to_untagged: MaybeAccessed<Option<CustomTokensNotCalculating>>,
    to_tagged_kvs: MaybeAccessed<Option<CustomTokensNotCalculating>>,

    variants: Vec<EnumVariant>,

    options: super::Options,
}

pub struct MakeEnumVariant {
    pub name: Ident,
    pub discriminant: Option<EqValue>,
    pub variant_type: VariantType,
    pub rename: Option<MetaPathSpanWith<Rename>>,
    pub rename_fields: Option<MetaPathSpanWith<Rename>>,
    pub specified_tag_mode: Option<SpecifiedTagMode>,
    pub tag: Option<MetaPathSpanWith<EqValue>>,
    pub content: Option<MetaPathSpanWith<EqValue>>,
    pub fields: Vec<StructField>,
    pub fields_ident_to_index: Option<HashMap<String, usize>>,
    pub to: Option<CustomTokens>,
    pub to_untagged_unspecified: StructToDefault,
    pub to_untagged: Option<CustomTokens>,
    pub to_tagged_kvs: Option<CustomTokens>,
}

impl From<MakeEnumVariant> for EnumVariant {
    fn from(
        MakeEnumVariant {
            name,
            discriminant,
            variant_type,
            rename,
            rename_fields,
            specified_tag_mode,
            tag,
            content,
            fields,
            fields_ident_to_index,
            to,
            to_untagged_unspecified,
            to_untagged,
            to_tagged_kvs,
        }: MakeEnumVariant,
    ) -> Self {
        Self {
            name,
            cache_for_name: Default::default(),
            discriminant: discriminant.map(From::from),
            cache_for_dummy_discriminant: Default::default(),
            variant_type,
            rename: MaybeAccessed::new(rename),
            rename_fields: MaybeAccessed::new(rename_fields),
            specified_tag_mode: MaybeAccessed::new(specified_tag_mode),
            tag: MaybeAccessed::new(tag),
            content: MaybeAccessed::new(content),
            only_field_index: None,
            fields,
            fields_ident_to_index,
            to: MaybeAccessed::new(to),
            cache_for_to: Default::default(),
            to_untagged_unspecified,
            to_untagged: inherit_or_custom::InheritOrCustom::new(to_untagged),
            cache_for_to_externally_tagged_default: Default::default(),
            to_tagged_kvs: inherit_or_custom::InheritOrCustom::new(to_tagged_kvs),
            cache_for_to_internally_tagged_default: Default::default(),
            cache_for_to_adjacently_tagged_default: Default::default(),
            cache_for_should_expand_bracket_question: Default::default(),
            cache_for_only_field_index: Default::default(),
        }
    }
}

pub struct EnumVariant {
    name: Ident,
    cache_for_name: Option<Vec<TokenTree>>,

    discriminant: Option<discriminant::Discriminant>,
    cache_for_dummy_discriminant: Option<TokenTree>,

    variant_type: VariantType,

    rename: MaybeAccessed<Option<MetaPathSpanWith<Rename>>>,

    /// Asserts [Rename::Paren]
    rename_fields: MaybeAccessed<Option<MetaPathSpanWith<Rename>>>,

    specified_tag_mode: MaybeAccessed<Option<SpecifiedTagMode>>,

    tag: MaybeAccessed<Option<MetaPathSpanWith<EqValue>>>,
    content: MaybeAccessed<Option<MetaPathSpanWith<EqValue>>>,

    /// The index of the only non-skip field
    only_field_index: Option<OnlyFieldResult<usize>>,

    fields: Vec<StructField>,
    /// Asserts `self.fields_ident_to_index.len() == self.fields.len()`
    fields_ident_to_index: Option<HashMap<String, usize>>,

    to: MaybeAccessed<Option<CustomTokens>>,
    cache_for_to: CacheForEnumVariantTo,

    to_untagged_unspecified: StructToDefault,
    to_untagged: inherit_or_custom::InheritOrCustom<StructToDefaultExpandError>,

    cache_for_to_externally_tagged_default: Option<(
        TokenTree,
        Result<(), CustomTokensExpandErrorOr<StructToDefaultExpandError>>,
    )>,

    to_tagged_kvs: inherit_or_custom::InheritOrCustom<StructToTaggedKvsDefaultExpandError>,

    cache_for_to_internally_tagged_default: CacheForToInternallyTaggedDefault,
    cache_for_to_adjacently_tagged_default: CacheForToAdjacentlyTaggedDefault,

    cache_for_should_expand_bracket_question: Option<bool>,

    cache_for_only_field_index: Option<OnlyFieldResult<usize>>,
}

#[derive(Default)]
struct CacheForEnumVariantTo {
    cache: Option<CachedEnumVariantTo>,
    is_expanding_inherited: bool,
}

enum CachedEnumVariantTo {
    SpecifiedOrInherited(TokensExpanded<EnumVariantToExpandError>),
    Unspecified,
}

#[derive(Clone)]
pub struct CustomTokensNotCalculating {
    span: Span,
    tokens: proc_macro::token_stream::IntoIter,
}

impl<G: Into<GroupParen>> From<MetaPathSpanWith<G>> for CustomTokensNotCalculating {
    fn from(MetaPathSpanWith(span, g): MetaPathSpanWith<G>) -> Self {
        Self {
            span,
            tokens: g.into().stream().into_iter(),
        }
    }
}

impl From<CustomTokensNotCalculating> for CustomTokens {
    fn from(CustomTokensNotCalculating { span, tokens }: CustomTokensNotCalculating) -> Self {
        CustomTokens {
            span,
            tokens: super::MaybeCalculating::NotCalculating(tokens),
        }
    }
}

struct ContextOfEnumVariant<'a> {
    ctx_enum: &'a mut ContextOfEnum,
    index_variant: usize,
}

impl ContextOfEnum {
    pub fn into_to_json(mut self, errors: &mut ErrorCollector) -> Vec<TokenTree> {
        let mut ts = Vec::new();
        self.expand_to(From::from(&mut ts), errors);

        // TODO: report unused
        ts
    }

    fn expand_to(&mut self, mut out: TokensCollector, errors: &mut ErrorCollector) {
        let item_span = self.name.span();

        // TODO: match
        let mut match_inner = TokenStream::new();
        (0..(self.variants.len())).for_each(|index_variant| {
            let mut ctx_variant = ContextOfEnumVariant {
                ctx_enum: self,
                index_variant,
            };
            let variant = ctx_variant.variant();
            let span = variant.name.span();
            let ref variant_name = variant.name.clone();

            let pat_start = quote!(
                #[cjson(match_branch_name(#variant_name))]
                Self::#variant_name
            )
            .with_default_span(item_span)
            .into_token_stream();
            match_inner.extend(Some(pat_start));

            let (to, res) = ctx_variant.to();
            if let Err(e) = res {
                errors.push(e.into_parse_error_with_span(span));
            }
            let to = TokenStream::from_iter(to.iter().cloned());

            let pat_body = ctx_variant.pat_body();

            match_inner.extend(pat_body);

            match_inner.extend(Some(
                quote!( => ).with_replaced_span(span).into_token_stream(),
            ));

            match_inner.extend(Some(
                quote! {
                    json!( #to ),
                }
                .with_default_span(span)
                .into_token_stream(),
            ));

            // TODO: unused errors
        });
    }
}

macro_rules! variant {
    ($ctx:ident) => {
        $ctx.ctx_enum.variants[$ctx.index_variant]
    };
}

macro_rules! access_inherit {
    ($self_:ident . $field:ident . $access:ident()) => {
        match variant!($self_).$field.$access() {
            v @ Some(_) => v,
            None => $self_.ctx_enum.$field.$access(),
        }
    };
}

type ContextOfEnumVariantField<'a> = field::ContextOfField<ContextOfEnumVariant<'a>>;

struct EnumVariantFieldHelper<'a> {
    ctx_enum_variant: ContextOfEnumVariant<'a>,
    index_field: usize,
}

impl field::ContextSupportsField for ContextOfEnumVariant<'_> {
    type FieldHelper<'a>
        = EnumVariantFieldHelper<'a>
    where
        Self: 'a;

    fn field_helper(&mut self, index_field: usize) -> Self::FieldHelper<'_> {
        EnumVariantFieldHelper {
            ctx_enum_variant: self.as_mut(),
            index_field,
        }
    }

    fn field(&self, index_field: usize) -> &StructField {
        &self.variant().fields[index_field]
    }

    fn field_mut(&mut self, index_field: usize) -> &mut StructField {
        &mut self.variant_mut().fields[index_field]
    }

    fn try_expand_prop_at_field(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) -> Result<(), expand_props::PropPath> {
        enum FirstProp {
            Self_(Span),
            Variant(Span),
            Item(Span),
        }
        let first_prop_type = 'first: {
            match &prop.0 {
                expand_props::Prop::Ident(ident) => ident_match!(match ident {
                    b"self" => break 'first FirstProp::Self_(ident.span()),
                    b"variant" => break 'first FirstProp::Variant(ident.span()),
                    b"item" => break 'first FirstProp::Item(ident.span()),
                    _ => {}
                }),
                expand_props::Prop::Literal(_) => {}
            }

            return Err(prop);
        };

        let expand_props::PropPath(_first_prop, rest_prop) = prop;

        match first_prop_type {
            FirstProp::Self_(span_self) => self.expand_self(span_self, rest_prop, out, errors),
            FirstProp::Variant(span_variant) => 'ret: {
                let mut rest_prop = rest_prop.into_iter();

                let Some(first_prop) = rest_prop.next() else {
                    errors.push_custom(
                        "@variant on enum variant field cannot expand to tokens",
                        span_variant,
                    );
                    break 'ret;
                };

                self.expand_prop_at_variant(
                    expand_props::PropPath(first_prop, Vec::from_iter(rest_prop)),
                    out,
                    errors,
                )
            }
            FirstProp::Item(span) => self.expand_item_rest_prop(span, rest_prop, out, errors),
        }

        Ok(())
    }
}

impl field::FieldHelper for EnumVariantFieldHelper<'_> {
    fn to_calc_name(&mut self) -> field::CalcName<'_> {
        let ctx_enum_variant = &mut self.ctx_enum_variant;
        let variant = &mut variant!(ctx_enum_variant);
        let field = &mut variant.fields[self.index_field];
        field.accessed_rename = true;

        let rename = match &field.rename {
            Some(v) => Some(v),
            None => match variant.rename_fields.access() {
                Some(v) => Some(v),
                None => ctx_enum_variant.ctx_enum.rename_fields.access().as_ref(),
            },
        };
        field::CalcName {
            options: &ctx_enum_variant.ctx_enum.options,
            rename,
            name: &field.name,
        }
    }

    fn to_calc_expr(&mut self) -> field::CalcExpr<'_> {
        let ctx_enum_variant = &mut self.ctx_enum_variant;
        let variant = &mut variant!(ctx_enum_variant);
        let field = &mut variant.fields[self.index_field];

        field.accessed_expr = true;

        let ident = match &field.name {
            typed_quote::Either::A(ident) => ident,
            typed_quote::Either::B(lit) => {
                let _: &Literal = lit;

                field.calc_pattern_destruct_unnamed.get_or_insert_with(|| {
                    let index_to_str = field.calc_index_to_str.get_or_insert(self.index_field);

                    let name = format!("cjson_macro_generated_unnamed_field_{index_to_str}");
                    Ident::new(&name, Span::call_site())
                })
            }
        };

        field::CalcExpr::PatternDestruct(ident)
    }
}

impl ContextWithFields for ContextOfEnumVariant<'_> {
    fn fields(&self) -> &[StructField] {
        &self.variant().fields
    }

    fn fields_ident_to_index(&self) -> Option<&std::collections::HashMap<String, usize>> {
        self.variant().fields_ident_to_index.as_ref()
    }
}

impl ContextOfEnumVariant<'_> {
    fn expand_prop_at_variant(
        &mut self,
        expand_props::PropPath(first_prop, rest_prop): expand_props::PropPath,
        mut out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let first_ident_span;

        enum FirstIdentType {
            Tag,
            Content,
            Name,
            Discriminant,
            DiscriminantOrName,
            To,
        }

        let first_ident_type = 'first: {
            let err_span = match first_prop {
                expand_props::Prop::Ident(ident) => {
                    first_ident_span = ident.span();
                    ident_match!(match ident {
                        b"tag" => break 'first FirstIdentType::Tag,
                        b"content" => break 'first FirstIdentType::Content,
                        b"name" => break 'first FirstIdentType::Name,
                        b"discriminant" => break 'first FirstIdentType::Discriminant,
                        b"discriminant_or_name" => break 'first FirstIdentType::DiscriminantOrName,
                        b"to" => break 'first FirstIdentType::To,
                        _ => ident.span(),
                    })
                }
                expand_props::Prop::Literal(literal) => literal.span(),
            };
            errors.push_custom("property not defined on enum variant", err_span);
            return;
        };

        let mut report_rest_prop = |msg: &'static str| {
            if let Some(rest_prop) = rest_prop.first() {
                errors.push_custom(msg, rest_prop.span());
            }
        };

        macro_rules! out {
            ($expr:expr) => {
                out!($expr, first_ident_span)
            };
            ($expr:expr, $span:expr) => {{
                let (ts, res) = $expr;
                out.extend_from_slice(ts);
                if let Err(e) = res {
                    errors.push(e.into_parse_error_with_span($span));
                }
            }};
        }

        macro_rules! out_tt {
            ($expr:expr, $span:expr) => {{
                let (tt, res) = $expr;
                out.push(TokenTree::clone(tt));
                if let Err(e) = res {
                    errors.push(e.into_parse_error_with_span($span));
                }
            }};
        }

        match first_ident_type {
            FirstIdentType::Tag => {
                report_rest_prop("property not defined on enum variant @tag");
                self.expand_tag(out, first_ident_span, errors)
            }
            FirstIdentType::Content => {
                report_rest_prop("property not defined on enum variant @content");
                out!(self.content())
            }
            FirstIdentType::Name => {
                report_rest_prop("property not defined on enum variant @name");
                self.expand_name(out, first_ident_span);
            }
            FirstIdentType::Discriminant => {
                report_rest_prop("property not defined on enum variant @discriminant");
                out!(self.discriminant())
            }
            FirstIdentType::DiscriminantOrName => {
                report_rest_prop("property not defined on enum variant @discriminant_or_name");
                out.extend_from_slice(self.to_discriminant_or_name());
            }
            FirstIdentType::To => {
                enum AfterTo {
                    None,
                    Untagged(Span),
                    UntaggedDefault(Span),
                    TaggedDefault(Span),
                    TaggedKvs(Span),
                    TaggedKvsDefault(Span),
                    ExternallyTaggedDefault(Span),
                    AdjacentlyTaggedDefault(Span),
                }
                let mut rest_prop = rest_prop.into_iter();
                let after_to = match rest_prop.next() {
                    Some(v) => 'ok: {
                        let mut report_rest_prop = |msg: &'static str| {
                            if let Some(p) = rest_prop.as_slice().first() {
                                errors.push_custom(msg, p.span());
                            }
                        };
                        let err_span = match v {
                            expand_props::Prop::Ident(ident) => ident_match!(match ident {
                                b"untagged" => {
                                    report_rest_prop(
                                        "property not defined on enum variant @(to.untagged)",
                                    );
                                    break 'ok AfterTo::Untagged(ident.span());
                                }
                                b"untagged_default" => {
                                    report_rest_prop(
                                        "property not defined on enum variant @(to.untagged_default)",
                                    );
                                    break 'ok AfterTo::UntaggedDefault(ident.span());
                                }
                                b"tagged_default" => {
                                    report_rest_prop(
                                        "property not defined on enum variant @(to.tagged_default)",
                                    );
                                    break 'ok AfterTo::TaggedDefault(ident.span());
                                }
                                b"tagged_kvs" => {
                                    report_rest_prop(
                                        "property not defined on enum variant @(to.tagged_kvs)",
                                    );
                                    break 'ok AfterTo::TaggedKvs(ident.span());
                                }
                                b"tagged_kvs_default" => {
                                    report_rest_prop(
                                        "property not defined on enum variant @(to.tagged_kvs_default)",
                                    );
                                    break 'ok AfterTo::TaggedKvsDefault(ident.span());
                                }
                                b"externally_tagged_default" => {
                                    report_rest_prop(
                                        "property not defined on enum variant @(to.externally_tagged_default)",
                                    );
                                    break 'ok AfterTo::ExternallyTaggedDefault(ident.span());
                                }
                                b"adjacently_tagged_default" => {
                                    report_rest_prop(
                                        "property not defined on enum variant @(to.adjacently_tagged_default)",
                                    );
                                    break 'ok AfterTo::AdjacentlyTaggedDefault(ident.span());
                                }
                                _ => ident.span(),
                            }),
                            expand_props::Prop::Literal(literal) => literal.span(),
                        };
                        errors.push_custom("property not defined on struct @to", err_span);
                        AfterTo::None
                    }
                    None => AfterTo::None,
                };

                match after_to {
                    AfterTo::None => out!(self.to()),
                    AfterTo::Untagged(span) => out!(self.to_untagged(), span),
                    AfterTo::UntaggedDefault(span) => out!(self.to_untagged_default(), span),
                    AfterTo::TaggedDefault(span) => {
                        out_tt!(self.to_internally_tagged_default(), span)
                    }
                    AfterTo::TaggedKvs(span) => out!(self.to_tagged_kvs(), span),
                    AfterTo::TaggedKvsDefault(span) => out!(self.to_tagged_kvs_default(), span),
                    AfterTo::ExternallyTaggedDefault(span) => {
                        out_tt!(self.to_externally_tagged_default(), span)
                    }
                    AfterTo::AdjacentlyTaggedDefault(span) => {
                        out_tt!(self.to_adjacently_tagged_default(), span)
                    }
                }
            }
        }
    }
}

impl ContextSupportsNonFieldProp for ContextOfEnumVariant<'_> {
    fn expand_non_field_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let first_ident_span;

        enum FirstIdentType {
            Item,
            Self_,
        }
        let first_ident_type = 'first: {
            match &prop.0 {
                expand_props::Prop::Ident(ident) => {
                    first_ident_span = ident.span();
                    ident_match!(match ident {
                        b"item" => break 'first FirstIdentType::Item,
                        b"self" => break 'first FirstIdentType::Self_,
                        _ => {}
                    })
                }
                expand_props::Prop::Literal(_) => {}
            }
            self.expand_prop_at_variant(prop, out, errors);
            return;
        };

        let rest_prop = prop.1;

        match first_ident_type {
            FirstIdentType::Item => {
                self.expand_item_rest_prop(first_ident_span, rest_prop, out, errors)
            }
            FirstIdentType::Self_ => self.expand_self(first_ident_span, rest_prop, out, errors),
        }
    }
}

impl ContextOfEnumVariant<'_> {
    /// Should be called after [Self::to] is called.
    fn pat_body(&self) -> Option<TokenTree> {
        let variant = self.variant();

        match variant.variant_type {
            VariantType::Unit => None,
            VariantType::Object => {
                let mut has_unused = false;
                let inner =
                    typed_quote::tokens::IterTokens(variant.fields.iter().filter_map(|f| {
                        if !f.accessed_expr {
                            has_unused = true;
                            return None;
                        }
                        let name: &Ident = match &f.name {
                            typed_quote::Either::A(ident) => ident,
                            typed_quote::Either::B(_) => {
                                unreachable!()
                            }
                        };
                        Some(quote!( #name , ))
                    }))
                    .into_token_stream();
                let dot_dot = has_unused.then(|| quote!(..));
                Some(quote!( { #inner #dot_dot } ).into_token_tree())
            }
            VariantType::Array => {
                let inner = typed_quote::tokens::IterTokens(variant.fields.iter().map(|f| {
                    match &f.name {
                        typed_quote::Either::A(_) => unreachable!(),
                        typed_quote::Either::B(lit) => {
                            let _: &Literal = lit;
                        }
                    }

                    if f.accessed_expr {
                        let pat = f.calc_pattern_destruct_unnamed.as_ref().unwrap();
                        typed_quote::Either::A(quote! { #pat , })
                    } else {
                        typed_quote::Either::B(quote! { _, })
                    }
                }));
                Some(quote!( ( #inner ) ).into_token_tree())
            }
        }
    }

    fn to(&mut self) -> (&[TokenTree], Result<(), EnumVariantToExpandError>) {
        fn dummy(span: Span) -> Vec<TokenTree> {
            let tt = quote!(null).with_replaced_span(span).into_token_tree();
            vec![tt]
        }

        let variant = &mut variant!(self);

        if variant.cache_for_to.cache.is_none() {
            let value = match variant.to.access_mut() {
                Some(variant_to) => CachedEnumVariantTo::SpecifiedOrInherited({
                    let (ts, res) = variant_to
                        .take_for_calculating()
                        .expand(self, dummy, || EnumToCircularRefError::VariantTo);

                    (
                        ts,
                        res.map_err(EnumVariantToExpandError::SpecifiedOrInherited),
                    )
                }),
                None => match self.ctx_enum.to.access_mut() {
                    Some(item_to) => CachedEnumVariantTo::SpecifiedOrInherited({
                        if variant.cache_for_to.is_expanding_inherited {
                            (
                                dummy(item_to.span),
                                Err(EnumVariantToExpandError::SpecifiedOrInherited(
                                    CustomTokensExpandError::CircularRef {
                                        msg: EnumToCircularRefError::ItemTo {
                                            variant_name: variant.name.clone(),
                                        },
                                    },
                                )),
                            )
                        } else {
                            variant.cache_for_to.is_expanding_inherited = true;
                            let item_to = item_to.clone();
                            let (ts, res) = CustomTokens::from(item_to).expand(
                                &mut ContextOfEnumItemTo(self.as_mut()),
                                dummy,
                                || (),
                            );
                            (
                                ts,
                                match res {
                                    Ok(()) => Ok(()),
                                    Err(e) => Err(EnumVariantToExpandError::SpecifiedOrInherited(
                                        match e {
                                            CustomTokensExpandError::CircularRef { msg: () } => {
                                                unreachable!()
                                            }
                                            CustomTokensExpandError::Other(e) => {
                                                CustomTokensExpandError::Other(e)
                                            }
                                        },
                                    )),
                                },
                            )
                        }
                    }),
                    None => CachedEnumVariantTo::Unspecified,
                },
            };

            variant!(self).cache_for_to.cache = Some(value);
        }

        let cached = variant!(self).cache_for_to.cache.as_ref().unwrap();

        let is_unspecified = if matches!(cached, CachedEnumVariantTo::Unspecified) {
            true
        } else {
            false
        };

        if is_unspecified {
            let (ts, res) = self.to_unspecified();
            (ts, res.map_err(EnumVariantToExpandError::Unspecified))
        } else {
            let cached = variant!(self).cache_for_to.cache.as_ref().unwrap();
            match cached {
                CachedEnumVariantTo::SpecifiedOrInherited((ts, res)) => (ts, res.clone()),
                CachedEnumVariantTo::Unspecified => unreachable!(),
            }
        }
    }

    fn to_unspecified(
        &mut self,
    ) -> (
        &[TokenTree],
        Result<(), EnumVariantToUnspecifiedExpandError>,
    ) {
        let SpecifiedTagMode { span: _, mode } = match self.access_specified_tag_mode() {
            Some(v) => v,
            None => self.access_inferred_tag_mode(),
        };

        match mode {
            TagMode::Untagged => {
                let (ts, res) = self.to_untagged();
                (
                    ts,
                    res.map_err(EnumVariantToUnspecifiedExpandError::UntaggedOrExternallyTagged),
                )
            }
            TagMode::TagOnly => (self.to_discriminant_or_name(), Ok(())),
            TagMode::ExternallyTagged => {
                let (tt, res) = self.to_externally_tagged_default();
                (
                    std::array::from_ref(tt),
                    res.map_err(EnumVariantToUnspecifiedExpandError::UntaggedOrExternallyTagged),
                )
            }
            TagMode::InternallyTagged => {
                let (tt, res) = self.to_internally_tagged_default();
                (
                    std::array::from_ref(tt),
                    res.map_err(EnumVariantToUnspecifiedExpandError::InternallyTagged),
                )
            }
            TagMode::AdjacentlyTagged => {
                let (tt, res) = self.to_adjacently_tagged_default();
                (
                    std::array::from_ref(tt),
                    res.map_err(EnumVariantToUnspecifiedExpandError::AdjacentlyTagged),
                )
            }
        }
    }

    fn access_specified_tag_mode(&mut self) -> Option<SpecifiedTagMode> {
        match *variant!(self).specified_tag_mode.access() {
            Some(mode) => Some(mode),
            None => *self.ctx_enum.specified_tag_mode.access(),
        }
    }

    fn access_inferred_tag_mode(&mut self) -> SpecifiedTagMode {
        match self.access_specified_tag_mode() {
            Some(mode) => mode,
            None => {
                let span;
                let mode = if let Some(content) = access_inherit!(self.content.access()) {
                    span = content.0;
                    TagMode::AdjacentlyTagged
                } else if let Some(tag) = access_inherit!(self.tag.access()) {
                    span = tag.0;
                    TagMode::InternallyTagged
                } else {
                    let variant = &variant!(self);
                    span = variant.name.span();
                    if matches!(variant.variant_type, VariantType::Unit) {
                        TagMode::TagOnly
                    } else {
                        TagMode::ExternallyTagged
                    }
                };
                SpecifiedTagMode { span, mode }
            }
        }
    }

    fn to_untagged(
        &mut self,
    ) -> (
        &mut Vec<TokenTree>,
        Result<(), CustomTokensExpandErrorOr<StructToDefaultExpandError>>,
    ) {
        self.try_to_inherit_or_custom(
            |this| &mut this.variant_mut().to_untagged,
            |ctx_enum| ctx_enum.to_untagged.access().clone(),
            |this| this.calc_expand_to_default(),
        )
    }

    fn to_untagged_default(
        &mut self,
    ) -> (&mut Vec<TokenTree>, Result<(), StructToDefaultExpandError>) {
        let (ts, res) = self.try_to_unspecified_prop(
            |this| &mut this.variant_mut().to_untagged,
            |this| this.calc_expand_to_default(),
        );

        (ts, res.clone())
    }

    fn to_tagged_kvs(
        &mut self,
    ) -> (
        &mut Vec<TokenTree>,
        Result<(), CustomTokensExpandErrorOr<StructToTaggedKvsDefaultExpandError>>,
    ) {
        self.try_to_inherit_or_custom(
            |this| &mut this.variant_mut().to_tagged_kvs,
            |ctx_enum| ctx_enum.to_tagged_kvs.access().clone(),
            |this| this.calc_to_tagged_kvs_default(),
        )
    }

    fn to_tagged_kvs_default(
        &mut self,
    ) -> (
        &mut Vec<TokenTree>,
        Result<(), StructToTaggedKvsDefaultExpandError>,
    ) {
        let (ts, res) = self.try_to_unspecified_prop(
            |this| &mut this.variant_mut().to_tagged_kvs,
            |this| this.calc_to_tagged_kvs_default(),
        );
        (ts, res.clone())
    }

    fn to_discriminant_or_name(&mut self) -> &[TokenTree] {
        let has_discriminant = match self.try_to_discriminant() {
            Ok(_) => true,
            Err(EnumVariantDiscriminantNotDefinedError {}) => false,
        };

        if has_discriminant {
            self.try_to_discriminant().ok().unwrap()
        } else {
            self.name_as_json_value()
        }
    }

    fn discriminant(
        &mut self,
    ) -> (
        &[TokenTree],
        Result<(), EnumVariantDiscriminantNotDefinedError>,
    ) {
        let res = match self.try_to_discriminant() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };

        let ts = if res.is_ok() {
            self.try_to_discriminant().ok().unwrap()
        } else {
            let index_variant = self.index_variant;
            let variant = self.variant_mut();
            let span = variant.name.span();
            let tt = variant.cache_for_dummy_discriminant.get_or_insert_with(|| {
                Literal::usize_unsuffixed(index_variant)
                    .with_replaced_span(span)
                    .into()
            });
            std::slice::from_ref(tt)
        };

        (ts, res)
    }

    fn try_to_discriminant(
        &mut self,
    ) -> Result<&[TokenTree], EnumVariantDiscriminantNotDefinedError> {
        match &mut variant!(self).discriminant {
            Some(d) => Ok(d.expand_as_json_value()),
            None => Err(EnumVariantDiscriminantNotDefinedError),
        }
    }

    fn cache_for_to_externally_tagged_default(
        &mut self,
    ) -> &mut Option<(
        TokenTree,
        Result<(), CustomTokensExpandErrorOr<StructToDefaultExpandError>>,
    )> {
        &mut self.variant_mut().cache_for_to_externally_tagged_default
    }

    fn to_externally_tagged_default(
        &mut self,
    ) -> (
        &TokenTree,
        Result<(), CustomTokensExpandErrorOr<StructToDefaultExpandError>>,
    ) {
        if self.cache_for_to_externally_tagged_default().is_none() {
            let span = self.variant().name.span();
            let mut inner = Vec::new();
            inner.extend_from_slice(self.name_as_json_value());
            inner.push(quote!(=).with_default_span(span).into_token_tree());
            let (ts, res) = self.to_untagged();
            inner.extend_from_slice(ts);

            let inner = TokenStream::from_iter(inner);
            let tt = quote!({#inner}).with_default_span(span).into_token_tree();

            *self.cache_for_to_externally_tagged_default() = Some((tt, res));
        }

        let (v, res) = self
            .cache_for_to_externally_tagged_default()
            .as_ref()
            .unwrap();
        (v, res.clone())
    }

    fn to_internally_tagged_default(
        &mut self,
    ) -> (&TokenTree, Result<(), StructToTaggedDefaultExpandError>) {
        let span = self.variant().name.span();
        self.to_internally_tagged_default_with(span, |this, mut out, _span| {
            let (ts, res) = this.to_tagged_kvs();
            out.extend_from_slice(ts);
            res
        })
    }

    fn cache_for_to_adjacently_tagged_default(&mut self) -> &mut CacheForToAdjacentlyTaggedDefault {
        &mut self.variant_mut().cache_for_to_adjacently_tagged_default
    }

    fn to_adjacently_tagged_default(
        &mut self,
    ) -> (&TokenTree, Result<(), ToAdjacentlyTaggedDefaultExpandError>) {
        if self.cache_for_to_adjacently_tagged_default().0.is_none() {
            let v = self.calc_to_adjacently_tagged_default();
            self.cache_for_to_adjacently_tagged_default().0 = Some(v);
        }

        let (expanded, res) = self
            .cache_for_to_adjacently_tagged_default()
            .0
            .as_ref()
            .unwrap();
        (expanded, res.clone())
    }

    fn calc_to_adjacently_tagged_default(
        &mut self,
    ) -> (TokenTree, Result<(), ToAdjacentlyTaggedDefaultExpandError>) {
        let name_span = self.variant().name.span();
        let mut object_inner = vec![];
        let mut out = TokensCollector::from(&mut object_inner);

        let expand_tag = self.try_expand_tag(out.as_mut());

        out.push(quote!(=).with_replaced_span(name_span).into_token_tree());

        self.expand_name(out.as_mut(), name_span);

        let expand_content;
        let expand_to_untagged;

        if self.should_expand_bracket_question() {
            out.push(quote!(;).with_replaced_span(name_span).into_token_tree());

            {
                let content_ts;
                (content_ts, expand_content) = self.content();
                out.extend_from_slice(content_ts);
            }

            out.push(quote!(=).with_replaced_span(name_span).into_token_tree());

            {
                let to_untagged_ts;
                (to_untagged_ts, expand_to_untagged) = self.to_untagged();
                out.extend_from_slice(to_untagged_ts);
            }
        } else {
            expand_content = Ok(());
            expand_to_untagged = Ok(());
        }

        let res = match (expand_tag, expand_content, expand_to_untagged) {
            (Ok(()), Ok(()), Ok(())) => Ok(()),
            (expand_tag, expand_content, expand_to_untagged) => {
                Err(ToAdjacentlyTaggedDefaultExpandError {
                    expand_tag: expand_tag.err(),
                    expand_content: expand_content.err(),
                    expand_to_untagged: expand_to_untagged.err(),
                })
            }
        };

        let object_inner = TokenStream::from_iter(object_inner);
        let tt = quote!({ #object_inner })
            .with_default_span(name_span)
            .into_token_tree();

        (tt, res)
    }

    fn content(&mut self) -> (&[TokenTree], Result<(), ContentNotDefinedError>) {
        let prop = match variant!(self)
            .content
            .access()
            .as_ref()
            .or_else(|| self.ctx_enum.content.access().as_ref())
        {
            Some(MetaPathSpanWith(span, eq_tag)) => ContextPropTagMut::Tagged {
                span_tag: *span,
                ts: eq_tag.value.as_slice(),
            },
            None => ContextPropTagMut::Untagged {
                default_span: self.ctx_enum.name.span(),
                cache_for_dummy: &mut self.ctx_enum.cache_for_dummy_content,
            },
        };
        let (ts, res) = prop.try_into_tokens();

        (ts, res.map_err(|()| ContentNotDefinedError))
    }

    fn expand_item_rest_prop(
        &mut self,
        first_ident_span: Span,
        rest_prop: Vec<expand_props::Prop>,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let mut rest_prop = rest_prop.into_iter();

        let Some(first_prop) = rest_prop.next() else {
            errors.push_custom("@item cannot expand to tokens", first_ident_span);
            return;
        };

        match first_prop {
            expand_props::Prop::Ident(ident) if ident_matches!(ident, b"name") => {
                self.ctx_enum.expand_name(out, ident.span());
            }
            first_prop => {
                errors.push_custom("property not defined on @item", first_prop.span());
                return;
            }
        }
    }

    fn should_expand_bracket_question(&mut self) -> bool {
        let variant = &mut variant!(self);

        *variant
            .cache_for_should_expand_bracket_question
            .get_or_insert_with(|| {
                variant.to_untagged.is_specified() || self.ctx_enum.to_untagged.value.is_some()
            })
    }
}

#[derive(Clone, Copy)]
struct ContentNotDefinedError;

impl IntoParseErrorWithSpan for ContentNotDefinedError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        ParseError::custom("@content not defined on enum variant", span)
    }
}

#[derive(Default)]
struct CacheForToAdjacentlyTaggedDefault(
    Option<(TokenTree, Result<(), ToAdjacentlyTaggedDefaultExpandError>)>,
);

#[derive(Clone)]
struct ToAdjacentlyTaggedDefaultExpandError {
    expand_tag: Option<super::StructTagExpandError>,
    expand_content: Option<ContentNotDefinedError>,
    expand_to_untagged: Option<CustomTokensExpandErrorOr<StructToDefaultExpandError>>,
}

impl IntoParseErrorWithSpan for ToAdjacentlyTaggedDefaultExpandError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        let mut errors = ErrorCollector::default();
        let Self {
            expand_tag,
            expand_content,
            expand_to_untagged,
        } = self;

        if let Some(e) = expand_tag {
            errors.push(e.into_parse_error_with_span(span));
        }
        if let Some(e) = expand_content {
            errors.push(e.into_parse_error_with_span(span));
        }
        if let Some(e) = expand_to_untagged {
            errors.push(e.into_parse_error_with_span(span));
        }
        errors.ok().unwrap_err()
    }
}

impl CalcToUntaggedDefault for ContextOfEnumVariant<'_> {
    fn get_to_default(&self) -> StructToDefault {
        self.variant().to_untagged_unspecified
    }

    fn span_to_calc_to_default(&self) -> Span {
        self.variant().name.span()
    }
}

impl CalcToTaggedKvsDefault for ContextOfEnumVariant<'_> {
    fn span_to_calc_to_tagged_kvs_default(&self) -> Span {
        self.variant().name.span()
    }
}

impl ContextWithPropTag for ContextOfEnumVariant<'_> {
    const MSG_PROP_TAG_NOT_DEFINED: &'static str = "@tag not defined on enum variant";

    fn prop_tag_mut(&mut self) -> ContextPropTagMut<'_> {
        match variant!(self)
            .tag
            .access()
            .as_ref()
            .or_else(|| self.ctx_enum.tag.access().as_ref())
        {
            Some(MetaPathSpanWith(span, eq_tag)) => ContextPropTagMut::Tagged {
                span_tag: *span,
                ts: eq_tag.value.as_slice(),
            },
            None => ContextPropTagMut::Untagged {
                default_span: self.ctx_enum.name.span(),
                cache_for_dummy: &mut self.ctx_enum.cache_for_dummy_tag,
            },
        }
    }
}

impl ToInternallyTaggedDefaultWith for ContextOfEnumVariant<'_> {
    fn cache_for_to_internally_tagged_default(&mut self) -> &mut CacheForToInternallyTaggedDefault {
        &mut self.variant_mut().cache_for_to_internally_tagged_default
    }
}

#[derive(Clone, Copy)]
pub enum VariantType {
    Unit,
    Object,
    Array,
}

#[derive(Clone)]
enum EnumToCircularRefError {
    VariantTo,
    ItemTo { variant_name: Ident },
}

impl IntoParseErrorWithSpan for EnumToCircularRefError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            EnumToCircularRefError::VariantTo => ParseError::custom("msg", span),
            EnumToCircularRefError::ItemTo { variant_name } => ParseError::custom(
                format!("@to on enum circularly references itself on variant `{variant_name}`"),
                span,
            ),
        }
    }
}

#[derive(Clone)]
enum EnumVariantToExpandError {
    SpecifiedOrInherited(CustomTokensExpandError<EnumToCircularRefError>),
    Unspecified(EnumVariantToUnspecifiedExpandError),
}

impl IntoParseErrorWithSpan for EnumVariantToExpandError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            EnumVariantToExpandError::SpecifiedOrInherited(e) => match e {
                CustomTokensExpandError::CircularRef { msg: e } => {
                    e.into_parse_error_with_span(span)
                }
                CustomTokensExpandError::Other(e) => e,
            },
            EnumVariantToExpandError::Unspecified(e) => e.into_parse_error_with_span(span),
        }
    }
}

#[derive(Clone)]
enum EnumVariantToUnspecifiedExpandError {
    UntaggedOrExternallyTagged(CustomTokensExpandErrorOr<StructToDefaultExpandError>),
    InternallyTagged(StructToTaggedDefaultExpandError),
    AdjacentlyTagged(ToAdjacentlyTaggedDefaultExpandError),
}

impl IntoParseErrorWithSpan for EnumVariantToUnspecifiedExpandError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            Self::UntaggedOrExternallyTagged(e) => e.into_parse_error_with_span(span),
            Self::InternallyTagged(e) => e.into_parse_error_with_span(span),
            Self::AdjacentlyTagged(e) => e.into_parse_error_with_span(span),
        }
    }
}

#[derive(Clone, Copy)]
struct EnumVariantDiscriminantNotDefinedError;

impl IntoParseErrorWithSpan for EnumVariantDiscriminantNotDefinedError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        ParseError::custom("@discriminant not defined on this enum variant", span)
    }
}

impl<'a> ContextOfEnumVariant<'a> {
    fn as_mut(&mut self) -> ContextOfEnumVariant<'_> {
        ContextOfEnumVariant {
            ctx_enum: self.ctx_enum,
            index_variant: self.index_variant,
        }
    }

    fn variant(&self) -> &EnumVariant {
        &variant!(self)
    }

    fn variant_mut(&mut self) -> &mut EnumVariant {
        &mut variant!(self)
    }
}

enum EnumToExpandError {}

struct MaybeAccessed<T> {
    value: T,
    accessed: bool,
}

impl<T> MaybeAccessed<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            accessed: false,
        }
    }

    fn access(&mut self) -> &T {
        self.accessed = true;
        &self.value
    }

    fn access_mut(&mut self) -> &mut T {
        self.accessed = true;
        &mut self.value
    }
}

type ContextAtBracketStarOfEnumVariant<'a> =
    bracket_star::ContextAtBracketStarOf<ContextOfEnumVariant<'a>>;

impl ContextSupportsAtBracketStar for ContextOfEnumVariant<'_> {
    const MSG_CANNOT_NEST_BRACKET_STAR: &'static str = "enum variant `@[...]*` cannot be nested";
    const MSG_NOT_SUPPORT_BRACKET_QUESTION: &'static str = "enum variant doesn't support `@[...]?`";
}

impl<'this> ContextSupportsOnlyField for ContextOfEnumVariant<'this> {
    fn cache_for_only_field_index(&mut self) -> &mut Option<OnlyFieldResult<usize>> {
        &mut self.variant_mut().cache_for_only_field_index
    }
}

impl<'this> ContextWithPropName for ContextOfEnumVariant<'this> {
    fn cache_for_name(&mut self) -> &mut Option<Vec<TokenTree>> {
        &mut self.variant_mut().cache_for_name
    }

    fn to_calc_name(&mut self) -> context_with_prop_name::CalcName<'_> {
        let variant = &mut variant!(self);

        let rename = match variant.rename.access() {
            Some(v) => Some(v),
            None => self.ctx_enum.rename_variants.access().as_ref(),
        };

        context_with_prop_name::CalcName {
            options: &self.ctx_enum.options,
            rename,
            name: &variant.name,
        }
    }
}

pub struct ContextAtBracketQuestionOfEnumItemTo<'a> {
    ctx_variant: ContextOfEnumVariant<'a>,
    question_span: Span,
}

impl<'a> ContextAtBracketQuestionOfEnumItemTo<'a> {
    fn as_mut(&mut self) -> ContextAtBracketQuestionOfEnumItemTo<'_> {
        ContextAtBracketQuestionOfEnumItemTo {
            ctx_variant: self.ctx_variant.as_mut(),
            question_span: self.question_span,
        }
    }
}

pub struct ContextAtBracketStarOfEnumItemTo<'a>(ContextAtBracketStarOfEnumVariant<'a>);

pub struct ContextAtBracketQuestionInsideBracketStarOfEnumItemTo<'a, 'star> {
    ctx_star: &'star mut ContextAtBracketStarOfEnumVariant<'a>,
    question_span: Span,
}

pub struct ContextAtBracketStarInsideQuestionOfEnumItemTo<'a> {
    ctx_star: ContextAtBracketStarOfEnumVariant<'a>,
    question_span: Span,
}

impl<'a> ContextAtBracketQuestionOfEnumItemTo<'a> {
    fn try_from_mut(ctx: &'a mut ContextOfEnumVariant<'_>, question_span: Span) -> Option<Self> {
        if ctx.should_expand_bracket_question() {
            Some(Self {
                ctx_variant: ctx.as_mut(),
                question_span,
            })
        } else {
            None
        }
    }
}

impl<'this> ContextOfEnumVariant<'this> {
    fn into_at_bracket_star<'a>(
        self,
        star_span: Span,
    ) -> bracket_star::ContextAtBracketStarOf<Self> {
        bracket_star::ContextAtBracketStarOf::new(self, star_span)
    }
}

impl<'this> Context for ContextOfEnumVariant<'this> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        _: &mut ErrorCollector,
    ) -> impl use<'a, 'this> + expand_props::ContextAtBracketStar {
        self.as_mut().into_at_bracket_star(star_span)
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        _: &mut ErrorCollector,
    ) -> Option<impl use<'a, 'this> + Context> {
        ContextAtBracketQuestionOfEnumItemTo::try_from_mut(self, question_span)
    }

    fn expand_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        match bracket_star::field_or(prop) {
            Ok((field_ident, rest_prop)) => {
                self.expand_only_field(field_ident.span(), rest_prop, out, errors)
            }
            Err(prop) => self.expand_non_field_prop(prop, out, errors),
        }
    }
}

impl<'this> Context for ContextAtBracketStarOfEnumItemTo<'this> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        errors: &mut ErrorCollector,
    ) -> impl use<'a, 'this> + expand_props::ContextAtBracketStar {
        self.0.at_bracket_star(star_span, errors)
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        _: &mut ErrorCollector,
    ) -> Option<impl use<'a, 'this> + Context> {
        Some(ContextAtBracketQuestionInsideBracketStarOfEnumItemTo {
            ctx_star: &mut self.0,
            question_span,
        })
    }

    fn expand_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        self.0.expand_prop(prop, out, errors)
    }
}

impl<'this> ContextAtBracketStar for ContextAtBracketStarOfEnumItemTo<'this> {
    fn has_current(&self) -> bool {
        self.0.has_current()
    }

    fn next(&mut self) {
        self.0.next()
    }
}

impl<'this, 'star> Context for ContextAtBracketQuestionInsideBracketStarOfEnumItemTo<'this, 'star> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        errors: &mut ErrorCollector,
    ) -> impl use<'a, 'this, 'star> + ContextAtBracketStar {
        self.ctx_star.at_bracket_star(star_span, errors)
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a, 'this, 'star> + Context> {
        errors.push_custom("enum variant `@[...]?` cannot be nested", question_span);
        None::<expand_props::NeverContext>
    }

    fn expand_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        self.ctx_star.expand_prop(prop, out, errors)
    }
}

impl<'this> Context for ContextAtBracketQuestionOfEnumItemTo<'this> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        _: &mut ErrorCollector,
    ) -> impl use<'a, 'this> + expand_props::ContextAtBracketStar {
        ContextAtBracketStarInsideQuestionOfEnumItemTo {
            ctx_star: self.ctx_variant.as_mut().into_at_bracket_star(star_span),
            question_span: self.question_span,
        }
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a, 'this> + Context> {
        errors.push_custom("enum variant `@[...]?` cannot be nested", question_span);
        None::<expand_props::NeverContext>
    }

    fn expand_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        self.ctx_variant.expand_prop(prop, out, errors)
    }
}

impl<'this> Context for ContextAtBracketStarInsideQuestionOfEnumItemTo<'this> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        errors: &mut ErrorCollector,
    ) -> impl use<'a, 'this> + expand_props::ContextAtBracketStar {
        errors.push_custom("enum variant `@[...]*` cannot be nested", star_span);
        expand_props::ErroredContext
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a, 'this> + Context> {
        errors.push_custom("enum variant `@[...]?` cannot be nested", question_span);
        None::<expand_props::NeverContext>
    }

    fn expand_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        self.ctx_star.expand_prop(prop, out, errors)
    }
}

impl<'this> expand_props::ContextAtBracketStar
    for ContextAtBracketStarInsideQuestionOfEnumItemTo<'this>
{
    fn has_current(&self) -> bool {
        self.ctx_star.has_current()
    }

    fn next(&mut self) {
        self.ctx_star.next()
    }
}

struct ContextOfEnumItemTo<'a>(ContextOfEnumVariant<'a>);

impl<'this> Context for ContextOfEnumItemTo<'this> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        _: &mut ErrorCollector,
    ) -> impl use<'a, 'this> + expand_props::ContextAtBracketStar {
        ContextAtBracketStarOfEnumItemTo(self.0.as_mut().into_at_bracket_star(star_span))
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        _: &mut ErrorCollector,
    ) -> Option<impl use<'a, 'this> + Context> {
        ContextAtBracketQuestionOfEnumItemTo::try_from_mut(&mut self.0, question_span)
    }

    fn expand_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        self.0.expand_prop(prop, out, errors)
    }
}

// impl Context for ContextOfEnumItemTo<'_> {}

impl ContextWithPropName for ContextOfEnum {
    fn cache_for_name(&mut self) -> &mut Option<Vec<TokenTree>> {
        &mut self.cache_for_name
    }

    fn to_calc_name(&mut self) -> context_with_prop_name::CalcName<'_> {
        context_with_prop_name::CalcName {
            options: &self.options,
            rename: self.rename.access().as_ref(),
            name: &self.name,
        }
    }
}
