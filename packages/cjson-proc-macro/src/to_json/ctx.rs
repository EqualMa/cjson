use std::{collections::HashMap, vec};

use proc_macro::{Ident, Literal, Span, TokenStream, TokenTree};
use typed_quote::{IntoTokenTree, IntoTokens, WithSpan, quote};

use crate::{
    ErrorCollector,
    expand_props::{self, Context, ContextAtBracketStar, TokensCollector},
    ident_match,
    syn_generic::{
        GroupParen, ParseError,
        parse_meta_utils::{EqValue as EqValueGeneric, FlagPresent, MetaPathSpanWith},
    },
};

use super::item::Rename;

use self::{
    context_with_fields::ContextWithFields as _,
    context_with_prop_tag::{ContextPropTagMut, ContextWithPropTag},
    non_field::ContextSupportsNonFieldProp,
    only_field::ContextSupportsOnlyField as _,
};

mod field;

mod bracket_star;

mod context_with_fields;

mod non_field;

mod only_field;

mod context_with_prop_tag;

type EqValue = EqValueGeneric<vec::IntoIter<TokenTree>>;

pub struct MakeContextOfStruct {
    pub name: Ident,
    pub rename: Option<MetaPathSpanWith<Rename>>,
    pub rename_fields: Option<MetaPathSpanWith<GroupParen>>,
    pub options: Options,
    pub fields: Vec<StructField>,
    pub fields_ident_to_index: Option<HashMap<String, usize>>,

    pub to_default: StructToDefault,
    pub to_custom: Option<CustomTokens>,

    pub tag: ContextOfStructTag,
}

impl From<MakeContextOfStruct> for ContextOfStruct {
    fn from(value: MakeContextOfStruct) -> Self {
        let MakeContextOfStruct {
            name,
            rename,
            rename_fields,
            options,
            fields,
            fields_ident_to_index,
            to_default,
            to_custom,
            tag,
        } = value;
        ContextOfStruct {
            name,
            rename,
            accessed_rename: false,
            rename_fields: rename_fields
                .map(|MetaPathSpanWith(span, paren)| MetaPathSpanWith(span, Rename::Paren(paren))),
            accessed_rename_fields: false,
            expanded_name: None,
            options,
            only_field_index: None,
            fields,
            fields_ident_to_index,
            self_dot: None,
            to: PropDefaultCustom::new(to_default, to_custom),
            to_tagged_default: PropExpanded::new(()),
            tag,
        }
    }
}

pub struct ContextOfStruct {
    name: Ident,

    rename: Option<MetaPathSpanWith<Rename>>,
    accessed_rename: bool,

    /// Asserts [Rename::Paren]
    rename_fields: Option<MetaPathSpanWith<Rename>>,
    accessed_rename_fields: bool,

    expanded_name: Option<Vec<TokenTree>>,

    options: Options,

    /// The index of the only non-skip field
    only_field_index: Option<OnlyFieldResult<usize>>,

    fields: Vec<StructField>,
    /// Asserts `self.fields_ident_to_index.len() == self.fields.len()`
    fields_ident_to_index: Option<HashMap<String, usize>>,

    self_dot: Option<Vec<TokenTree>>,

    to: PropDefaultCustom<StructToDefault, StructToDefaultExpandError>,

    to_tagged_default: PropExpandedWithErr<(), StructToTaggedDefaultExpandError>,

    tag: ContextOfStructTag,
}

pub enum ContextOfStructTag {
    Untagged {
        dummy: Option<Vec<TokenTree>>,
    },
    Tagged {
        span_tag: Span,
        ts: std::vec::IntoIter<TokenTree>,
        accessed: bool,
    },
}

impl From<Option<MetaPathSpanWith<EqValue>>> for ContextOfStructTag {
    fn from(value: Option<MetaPathSpanWith<EqValue>>) -> Self {
        match value {
            Some(MetaPathSpanWith(span_tag, eq_value)) => ContextOfStructTag::Tagged {
                span_tag,
                ts: eq_value.value,
                accessed: false,
            },
            None => ContextOfStructTag::Untagged { dummy: None },
        }
    }
}

type ContextOfStructField<'a> = field::ContextOfField<&'a mut ContextOfStruct>;

macro_rules! ctx_struct_field {
    ($this:expr) => {
        $this.ctx_struct.fields[$this.index_field]
    };
}

trait TryWithOutSpan {
    fn try_with_out_span<'a, E: IntoParseErrorWithSpan>(
        &'a mut self,
        out: TokensCollector<'_>,
        span: Span,
        errors: &mut ErrorCollector,
        f: impl FnOnce(&'a mut Self, TokensCollector<'_>, Span) -> Result<(), E>,
    ) {
        let res = f(self, out, span);

        if let Err(e) = res {
            errors.push(e.into_parse_error_with_span(span));
        }
    }
}

impl<T: ?Sized> TryWithOutSpan for T {}

pub struct FieldHelper<'a> {
    ctx_struct: &'a mut ContextOfStruct,
    index_field: usize,
}

impl field::ContextSupportsField for ContextOfStruct {
    type FieldHelper<'a>
        = FieldHelper<'a>
    where
        Self: 'a;

    fn field_helper(&mut self, index_field: usize) -> Self::FieldHelper<'_> {
        FieldHelper {
            ctx_struct: self,
            index_field,
        }
    }

    fn field(&self, index_field: usize) -> &StructField {
        &self.fields[index_field]
    }

    fn field_mut(&mut self, index_field: usize) -> &mut StructField {
        &mut self.fields[index_field]
    }

    fn try_expand_prop_at_field(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) -> Result<(), expand_props::PropPath> {
        self.impl_try_expand_at_field(prop, out, errors)
    }
}

impl<'a> field::FieldHelper for FieldHelper<'a> {
    fn to_calc_name(&mut self) -> field::CalcName<'_> {
        let this = &mut ctx_struct_field!(self);
        this.accessed_rename = true;

        let rename = match &this.rename {
            Some(v) => Some(v),
            None => {
                self.ctx_struct.accessed_rename_fields = true;
                self.ctx_struct.rename_fields.as_ref()
            }
        };

        field::CalcName {
            options: &self.ctx_struct.options,
            rename,
            name: &this.name,
        }
    }

    fn to_calc_expr(&mut self) -> field::CalcExpr<'_> {
        let name = ctx_struct_field!(self).name.clone();
        let ref_self_dot = self.ctx_struct.self_dot();
        field::CalcExpr::RefSelfDot { ref_self_dot, name }
    }
}

impl ContextOfStructField<'_> {}

pub struct MakeStructField {
    pub skip: Option<FlagPresent>,
    pub name: typed_quote::Either<Ident, Literal>,
    pub type_: std::vec::IntoIter<TokenTree>,
    pub rename: Option<MetaPathSpanWith<Rename>>,
    pub to: Option<CustomTokens>,
    pub to_kvs_default: StructFieldToKvsDefault,
    pub to_kvs_custom: Option<CustomTokens>,
    pub to_items_default: StructFieldToItemsDefault,
    pub to_items_custom: Option<CustomTokens>,
}

impl From<MakeStructField> for StructField {
    fn from(value: MakeStructField) -> Self {
        let MakeStructField {
            skip,
            name,
            type_,
            rename,
            to,
            to_kvs_default,
            to_kvs_custom,
            to_items_default,
            to_items_custom,
        } = value;
        StructField {
            skip,
            name,
            type_,
            rename,
            accessed_rename: false,
            expanded_name: None,
            expanded_index_to_str: None,
            calc_index_to_str: Default::default(),
            calc_pattern_destruct_unnamed: None,
            to: PropExpanded::new(to),
            to_kvs: PropDefaultCustom::new(to_kvs_default, to_kvs_custom),
            to_items: PropDefaultCustom::new(to_items_default, to_items_custom),
        }
    }
}

pub struct StructField {
    skip: Option<FlagPresent>,

    name: typed_quote::Either<Ident, Literal>,

    type_: std::vec::IntoIter<TokenTree>,

    rename: Option<MetaPathSpanWith<Rename>>,
    accessed_rename: bool,

    expanded_name: Option<(Vec<TokenTree>, Result<(), StructFieldExpandNameError>)>,

    expanded_index_to_str: Option<(Vec<TokenTree>, Result<(), StructFieldExpandIndexToStrError>)>,

    calc_index_to_str: CacheIndexToString,
    calc_pattern_destruct_unnamed: Option<Ident>,

    /// The custom `to`
    to: PropExpandedWithErr<Option<CustomTokens>, CustomTokensExpandError>,

    to_kvs: PropDefaultCustom<StructFieldToKvsDefault, StructFieldExpandToKvsDefaultError>,

    to_items: PropDefaultCustom<StructFieldToItemsDefault, StructFieldExpandToItemsDefaultError>,
}

#[derive(Default)]
struct CacheIndexToString(Option<String>);

impl CacheIndexToString {
    fn get_or_insert(&mut self, index_field: usize) -> &str {
        self.0.get_or_insert_with(|| index_field.to_string())
    }
}

struct PropDefaultCustom<D, DE> {
    default: PropExpandedWithErr<D, DE>,
    custom: PropExpandedWithErr<Option<CustomTokens>, CustomTokensExpandErrorOr<DE>>,
}

impl<D, DE> PropDefaultCustom<D, DE> {
    fn new(default: D, custom: Option<CustomTokens>) -> Self {
        Self {
            default: PropExpanded::new(default),
            custom: PropExpanded::new(custom),
        }
    }
}

type PropExpandedWithErr<P, E> = PropExpanded<P, (Vec<TokenTree>, Result<(), E>)>;

struct PropExpanded<P, EXP> {
    value: P,
    accessed: bool,
    expanded: Option<EXP>,
}

impl<P, EXP> PropExpanded<P, EXP> {
    fn new(value: P) -> Self {
        Self {
            value,
            accessed: false,
            expanded: None,
        }
    }
}

impl<P, E> PropExpandedWithErr<P, E> {
    fn try_expand<Ctx>(
        ctx: &mut Ctx,
        mut get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        calc: impl FnOnce(&mut Ctx) -> (Vec<TokenTree>, Result<(), E>),
        mut out: TokensCollector<'_>,
    ) -> Result<(), E>
    where
        E: Clone,
    {
        let mut this = get_prop(ctx);
        this.accessed = true;

        let (expanded, res) = match &mut this.expanded {
            Some(v) => v,
            None => {
                let v = calc(ctx);

                this = get_prop(ctx);
                this.expanded.insert(v)
            }
        };

        out.extend_from_slice(expanded);

        res.clone()
    }
}

impl StructField {
    fn name_span(&self) -> Span {
        match &self.name {
            typed_quote::Either::A(tt) => tt.span(),
            typed_quote::Either::B(tt) => tt.span(),
        }
    }
}

/// ExprAsType
struct StructFieldToDefault;

pub enum StructFieldToKvsDefault {
    /// `{ @name = @to; }`
    BracedNameEqTo,
    Flatten {
        span: Span,
    },
}

pub struct CustomTokens {
    span: Span,
    tokens: MaybeCalculating<proc_macro::token_stream::IntoIter>,
}

impl<T: Into<GroupParen>> From<MetaPathSpanWith<T>> for CustomTokens {
    fn from(MetaPathSpanWith(span, g): MetaPathSpanWith<T>) -> Self {
        Self {
            span,
            tokens: MaybeCalculating::NotCalculating(g.into().stream().into_iter()),
        }
    }
}

impl CustomTokens {
    fn take_for_calculating(&mut self) -> Self {
        Self {
            span: self.span,
            tokens: self.tokens.take(),
        }
    }

    fn expand_map_err<E: HasConstCircularRefMsg>(
        self,
        ctx: &mut impl Context,
        default: impl FnOnce(Span) -> Vec<TokenTree>,
    ) -> (Vec<TokenTree>, Result<(), CustomTokensExpandErrorOr<E>>) {
        let (ts, res) = self.expand(ctx, default, || E::CIRCULAR_REF_MSG);

        (ts, res.map_err(CustomTokensExpandErrorOr::Custom))
    }

    fn expand(
        self,
        ctx: &mut impl Context,
        default: impl FnOnce(Span) -> Vec<TokenTree>,
        get_cir_ref_msg: impl FnOnce() -> &'static str,
    ) -> (Vec<TokenTree>, Result<(), CustomTokensExpandError>) {
        let CustomTokens { span, tokens } = self;

        let tokens = match tokens {
            MaybeCalculating::NotCalculating(tokens) => tokens,
            MaybeCalculating::Calculating => {
                return (
                    default(span),
                    Err(CustomTokensExpandError::CircularRef {
                        msg: get_cir_ref_msg(),
                    }),
                );
            }
        };
        let mut errors = ErrorCollector::default();
        let mut out = Vec::new();
        let _: expand_props::MaybeIntact<(), ()> =
            expand_props::expand_ts_iter_to(From::from(&mut out), tokens, ctx, &mut errors);
        (out, errors.ok().map_err(CustomTokensExpandError::Other))
    }

    fn try_expand_map_err<'ctx, Ctx: Context, E: HasConstCircularRefMsg>(
        this: Option<Self>,
        ctx: &'ctx mut Ctx,
        default_for_other: impl FnOnce(&'ctx mut Ctx, TokensCollector<'_>) -> Result<(), E>,
    ) -> (Vec<TokenTree>, Result<(), CustomTokensExpandErrorOr<E>>) {
        match this {
            Some(this) => this.expand_map_err::<E>(ctx, E::default_for_circular_ref),
            None => {
                let mut ts = Vec::new();
                let res = default_for_other(ctx, TokensCollector::from(&mut ts))
                    .map_err(CustomTokensExpandErrorOr::Other);

                (ts, res)
            }
        }
    }

    fn take_and_expand<'ctx, Ctx: Context, E: HasConstCircularRefMsg>(
        ctx: &'ctx mut Ctx,
        custom_tokens: impl FnOnce(&mut Ctx) -> &mut Option<CustomTokens>,
        default_for_other: impl FnOnce(&'ctx mut Ctx, TokensCollector<'_>) -> Result<(), E>,
    ) -> (Vec<TokenTree>, Result<(), CustomTokensExpandErrorOr<E>>) {
        let this = match custom_tokens(ctx) {
            Some(custom) => Some(custom.take_for_calculating()),
            None => None,
        };

        Self::try_expand_map_err(this, ctx, default_for_other)
    }
}

pub enum StructFieldToItemsDefault {
    BracketedTo,
    Flatten { span: Span },
}

enum MaybeCalculating<T> {
    NotCalculating(T),
    Calculating,
}

impl<T> MaybeCalculating<T> {
    fn take(&mut self) -> Self {
        std::mem::replace(self, Self::Calculating)
    }
    fn take_for_calculating(&mut self) -> Option<T> {
        match self.take() {
            MaybeCalculating::NotCalculating(v) => Some(v),
            MaybeCalculating::Calculating => None,
        }
    }
}

#[derive(Clone, Copy)]
struct StructFieldExpandNameError;

impl StructFieldExpandNameError {
    fn to_msg(&self) -> impl Into<std::borrow::Cow<'static, str>> {
        "unnamed field doesn't have `@name` unless renamed"
    }
}

#[derive(Debug, Clone, Copy)]
struct StructFieldExpandIndexToStrError;

impl IntoParseErrorWithSpan for StructFieldExpandIndexToStrError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        ParseError::custom(self.to_msg(), span)
    }
}

impl StructFieldExpandIndexToStrError {
    fn to_msg(&self) -> impl Into<std::borrow::Cow<'static, str>> {
        "named field doesn't have `@index_to_str` property"
    }
}

#[derive(Clone)]
enum StructFieldExpandToDefaultError {}

impl HasConstCircularRefMsg for StructFieldExpandToDefaultError {
    const CIRCULAR_REF_MSG: &str = "@to circularly references itself";

    fn default_for_circular_ref(span: Span) -> Vec<TokenTree> {
        vec![quote!(null).with_replaced_span(span).into_token_tree()]
    }
}

type StructFieldExpandToError = CustomTokensExpandError;

#[derive(Clone)]
enum StructFieldExpandToKvsDefaultError {
    NameEqTo {
        expand_name: Option<StructFieldExpandNameError>,
        expand_to: Option<StructFieldExpandToError>,
    },
    FlattenOnUnnamedField,
}

impl HasConstCircularRefMsg for StructFieldExpandToKvsDefaultError {
    const CIRCULAR_REF_MSG: &str = "@to_kvs circularly references itself";

    fn default_for_circular_ref(span: Span) -> Vec<TokenTree> {
        vec![quote!({}).with_default_span(span).into_token_tree()]
    }
}

type StructFieldExpandToKvsError = CustomTokensExpandErrorOr<StructFieldExpandToKvsDefaultError>;

#[derive(Clone)]
enum CustomTokensExpandError {
    CircularRef { msg: &'static str },
    Other(ParseError),
}

impl IntoParseErrorWithSpan for CustomTokensExpandError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            Self::CircularRef { msg } => ParseError::custom(msg, span),
            Self::Other(e) => e,
        }
    }
}

#[derive(Clone)]
enum CustomTokensExpandErrorOr<E> {
    Custom(CustomTokensExpandError),
    Other(E),
}

impl<E> From<CustomTokensExpandError> for CustomTokensExpandErrorOr<E> {
    fn from(v: CustomTokensExpandError) -> Self {
        Self::Custom(v)
    }
}

trait HasConstCircularRefMsg {
    const CIRCULAR_REF_MSG: &str;

    fn default_for_circular_ref(span: Span) -> Vec<TokenTree>;
}

impl<E: HasConstCircularRefMsg> CustomTokensExpandErrorOr<E> {
    const CIRCULAR_REF: Self = Self::Custom(CustomTokensExpandError::CircularRef {
        msg: E::CIRCULAR_REF_MSG,
    });
}

impl<E: IntoParseErrorWithSpan> IntoParseErrorWithSpan for CustomTokensExpandErrorOr<E> {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            CustomTokensExpandErrorOr::Custom(e) => e.into_parse_error_with_span(span),
            CustomTokensExpandErrorOr::Other(e) => e.into_parse_error_with_span(span),
        }
    }
}

impl IntoParseErrorWithSpan for StructFieldExpandToKvsDefaultError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            Self::NameEqTo {
                expand_name,
                expand_to,
            } => {
                let mut es = ErrorCollector::default();
                if let Some(e) = expand_name {
                    es.push_custom(e.to_msg(), span);
                }
                if let Some(e) = expand_to {
                    es.push(e.into_parse_error_with_span(span));
                }
                es.ok().unwrap_err()
            }
            Self::FlattenOnUnnamedField => ParseError::custom(
                "@to_kvs must be explicitly specified on flattened unnamed field to avoid ambiguity",
                span,
            ),
        }
    }
}

#[derive(Clone)]
enum StructFieldExpandToItemsDefaultError {
    To(StructFieldExpandToError),
    FlattenOnNamedField,
}

type StructFieldExpandToItemsError =
    CustomTokensExpandErrorOr<StructFieldExpandToItemsDefaultError>;

impl HasConstCircularRefMsg for StructFieldExpandToItemsDefaultError {
    const CIRCULAR_REF_MSG: &str = "@to_items circularly references itself";
    fn default_for_circular_ref(span: Span) -> Vec<TokenTree> {
        vec![quote!([]).with_default_span(span).into_token_tree()]
    }
}

impl IntoParseErrorWithSpan for StructFieldExpandToItemsDefaultError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            Self::To(e) => e.into_parse_error_with_span(span),
            Self::FlattenOnNamedField => ParseError::custom(
                "@to_items must be explicitly specified on flattened named field to avoid ambiguity",
                span,
            ),
        }
    }
}

pub struct Options {
    pub crate_path: TokenStream,
}

type ContextAtBracketStarOfStruct<'a> =
    bracket_star::ContextAtBracketStarOf<&'a mut ContextOfStruct>;

impl StructField {
    fn not_skipped(&self) -> bool {
        self.skip.is_none()
    }
    fn skipped(&self) -> bool {
        self.skip.is_some()
    }
}

impl context_with_fields::ContextWithFields for ContextOfStruct {
    fn fields(&self) -> &[StructField] {
        &self.fields
    }

    fn fields_ident_to_index(&self) -> Option<&std::collections::HashMap<String, usize>> {
        self.fields_ident_to_index.as_ref()
    }
}

impl bracket_star::ContextSupportsAtBracketStar for ContextOfStruct {
    const MSG_CANNOT_NEST_BRACKET_STAR: &'static str = "struct `@[...]*` cannot be nested";

    fn should_expand_bracket_question(&self, _field_index: usize) -> Result<bool, &'static str> {
        Err("struct doesn't support `@[...]?`")
    }
}

impl only_field::ContextSupportsOnlyField for ContextOfStruct {
    fn cache_for_only_field_index(&mut self) -> &mut Option<OnlyFieldResult<usize>> {
        &mut self.only_field_index
    }
}

impl Context for ContextOfStruct {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        _: &mut ErrorCollector,
    ) -> impl use<'a> + ContextAtBracketStar {
        ContextAtBracketStarOfStruct::new(self, star_span)
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a> + Context> {
        errors.push_custom("struct doesn't support `@[...]?`", question_span);

        None::<expand_props::NeverContext>
    }

    fn expand_prop(
        &mut self,
        prop: expand_props::PropPath,
        out: expand_props::TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        self.expand_only_field_or_non_field(prop, out, errors)
    }
}

impl ContextSupportsNonFieldProp for ContextOfStruct {
    fn expand_non_field_prop<'a>(
        &'a mut self,
        expand_props::PropPath(first_prop, rest_prop): expand_props::PropPath,
        out: expand_props::TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let first_ident_span;

        enum FirstIdentType {
            Name,
            Self_,
            Tag,
            To,
        }
        let first_ident_type = 'first: {
            let err_span = match first_prop {
                expand_props::Prop::Ident(ident) => {
                    first_ident_span = ident.span();
                    ident_match!(match ident {
                        b"name" => break 'first FirstIdentType::Name,
                        b"self" => break 'first FirstIdentType::Self_,
                        b"tag" => break 'first FirstIdentType::Tag,
                        b"to" => break 'first FirstIdentType::To,
                        _ => ident.span(),
                    })
                }
                expand_props::Prop::Literal(literal) => literal.span(),
            };
            errors.push_custom("property not defined on struct", err_span);
            return;
        };

        match first_ident_type {
            FirstIdentType::Name => {
                if let Some(rest_prop) = rest_prop.first() {
                    errors.push_custom("property not defined on struct @name", rest_prop.span());
                }
                self.expand_name(out, first_ident_span);
                return;
            }
            FirstIdentType::Self_ => self.expand_self(first_ident_span, rest_prop, out, errors),
            FirstIdentType::Tag => {
                if let Some(rest_prop) = rest_prop.first() {
                    errors.push_custom("property not defined on struct @tag", rest_prop.span());
                }
                self.expand_tag(out, first_ident_span, errors)
            }
            FirstIdentType::To => {
                enum AfterTo {
                    None,
                    Default(Span),
                    TaggedDefault(Span),
                }
                let mut rest_prop = rest_prop.into_iter();
                let after_to = match rest_prop.next() {
                    Some(v) => 'ok: {
                        let err_span = match v {
                            expand_props::Prop::Ident(ident) => ident_match!(match ident {
                                b"default" => {
                                    if let Some(p) = rest_prop.as_slice().first() {
                                        errors.push_custom(
                                            "property not defined on struct @to.default",
                                            p.span(),
                                        );
                                    }
                                    break 'ok AfterTo::Default(ident.span());
                                }
                                b"tagged_default" => {
                                    if let Some(p) = rest_prop.as_slice().first() {
                                        errors.push_custom(
                                            "property not defined on struct @to.tagged_default",
                                            p.span(),
                                        );
                                    }
                                    break 'ok AfterTo::TaggedDefault(ident.span());
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
                    AfterTo::None => self.expand_to(out, first_ident_span, errors),
                    AfterTo::Default(span) => self.expand_to_default(out, span, errors),
                    AfterTo::TaggedDefault(span) => {
                        self.expand_to_tagged_default(out, span, errors)
                    }
                }
            }
        }
    }
}

impl ContextOfStruct {
    pub fn into_to_json(mut self, errors: &mut ErrorCollector) -> Vec<TokenTree> {
        let mut ts = Vec::new();
        let span = self.name.span();
        self.expand_to(From::from(&mut ts), span, errors);

        // TODO: report unused
        ts
    }

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
            |this| &mut this.to.custom,
            Self::calc_expand_to,
            out,
        )
    }

    fn calc_expand_to(&mut self) -> (Vec<TokenTree>, Result<(), StructToExpandError>) {
        CustomTokens::take_and_expand::<_, StructToDefaultExpandError>(
            self,
            |ctx| &mut ctx.to.custom.value,
            |ctx, out| {
                let span = ctx.name.span();
                ctx.try_expand_to_default(out, span)
            },
        )
    }

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
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @to.default
    ) -> Result<(), StructToDefaultExpandError> {
        PropExpanded::try_expand(
            self,
            |this| &mut this.to.default,
            Self::calc_expand_to_default,
            out,
        )
    }

    fn calc_expand_to_default(
        &mut self,
    ) -> (Vec<TokenTree>, Result<(), StructToDefaultExpandError>) {
        let name_span = self.name.span();
        match self.to.default.value {
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
                    let span = ctx_struct_field!(ctx).name_span();
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
                    let span = ctx_struct_field!(ctx).name_span();
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
        out: TokensCollector<'_>,
        _span: Span, // TODO: link @to.tagged_default
    ) -> Result<(), StructToTaggedDefaultExpandError> {
        PropExpanded::try_expand(
            self,
            |this| &mut this.to_tagged_default,
            Self::calc_expand_to_tagged_default,
            out,
        )
    }

    fn calc_expand_to_tagged_default(
        &mut self,
    ) -> (Vec<TokenTree>, Result<(), StructToTaggedDefaultExpandError>) {
        let name_span = self.name.span();
        let mut object_inner = vec![];
        let mut out = TokensCollector::from(&mut object_inner);

        let expand_tag = self.try_expand_tag(out.as_mut());

        out.push(quote!(=).with_replaced_span(name_span).into_token_tree());

        self.expand_name(out.as_mut(), name_span);

        out.push(quote!(;).with_replaced_span(name_span).into_token_tree());

        let after_tag = match self.to.default.value {
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
                    let span = ctx_struct_field!(ctx).name_span();
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

impl ContextWithPropTag for ContextOfStruct {
    const MSG_PROP_TAG_NOT_DEFINED: &'static str = "@tag not defined on struct";

    fn prop_tag_mut(&mut self) -> ContextPropTagMut<'_> {
        match &mut self.tag {
            ContextOfStructTag::Untagged { dummy } => ContextPropTagMut::Untagged {
                default_span: self.name.span(),
                cache_for_dummy: dummy,
            },
            ContextOfStructTag::Tagged {
                span_tag,
                ts,
                accessed,
            } => ContextPropTagMut::Tagged {
                span_tag: *span_tag,
                ts: ts.as_slice(),
                accessed,
            },
        }
    }
}

pub enum StructToDefault {
    Transparent {
        /// span of transparent
        span: Option<Span>,
    },
    Unit,
    Object,
    Array,
}

#[derive(Clone)]
enum StructToDefaultExpandError {
    Transparent {
        only_field: Option<OnlyFieldError>,
        expand_to: Option<StructFieldExpandToError>,
    },
    ObjectOrArray(ParseError),
}

type StructToExpandError = CustomTokensExpandErrorOr<StructToDefaultExpandError>;

impl HasConstCircularRefMsg for StructToDefaultExpandError {
    const CIRCULAR_REF_MSG: &str = "@to on struct circularly references itself";
    fn default_for_circular_ref(span: Span) -> Vec<TokenTree> {
        vec![quote!(null).with_default_span(span).into_token_tree()]
    }
}

impl IntoParseErrorWithSpan for StructToDefaultExpandError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        match self {
            StructToDefaultExpandError::Transparent {
                only_field,
                expand_to,
            } => {
                let mut errors = ErrorCollector::default();

                if let Some(e) = only_field {
                    errors.push(e.into_parse_error_with_span(span));
                }

                if let Some(e) = expand_to {
                    errors.push(e.into_parse_error_with_span(span));
                }

                errors.ok().unwrap_err()
            }
            StructToDefaultExpandError::ObjectOrArray(e) => e,
        }
    }
}

#[derive(Clone)]
struct StructToTaggedDefaultExpandError {
    expand_tag: Option<StructTagExpandError>,
    after_tag: Option<StructToDefaultExpandError>,
}

impl IntoParseErrorWithSpan for StructToTaggedDefaultExpandError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        let mut errors = ErrorCollector::default();
        if let Some(e) = self.expand_tag {
            errors.push(e.into_parse_error_with_span(span));
        }
        if let Some(e) = self.after_tag {
            errors.push(e.into_parse_error_with_span(span));
        }
        errors.ok().unwrap_err()
    }
}

#[derive(Clone, Copy)]
struct StructTagExpandError(&'static str);

impl IntoParseErrorWithSpan for StructTagExpandError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        ParseError::custom(self.0, span)
    }
}

fn make_fn_clone_and_set_span(span: Span) -> impl Fn(&TokenTree) -> TokenTree {
    move |tt: &TokenTree| {
        let mut tt = tt.clone();
        tt.set_span(span); // not setting spans inside groups
        tt
    }
}

impl ContextOfStruct {
    fn expand_name(&mut self, mut out: expand_props::TokensCollector<'_>, span: Span) {
        let expanded_name = match &mut self.expanded_name {
            Some(expanded_name) => expanded_name,
            None => {
                let ts = self.calc_expand_name();
                self.expanded_name.insert(ts)
            }
        };

        out.extend(expanded_name.iter().map(make_fn_clone_and_set_span(span)));
    }

    fn calc_expand_name(&mut self) -> Vec<TokenTree> {
        self.accessed_rename = true;
        if let Some(MetaPathSpanWith(rename_span, ref rename)) = self.rename {
            rename.to_tokens_as_json_object_key(
                //
                &self.options.crate_path,
                rename_span,
                &self.name,
            )
        } else {
            let name = &self.name;

            let lit = crate::utils::ident_to_literal_string(name);

            vec![lit.into()]
        }
    }

    fn self_dot(&mut self) -> &[TokenTree] {
        self.self_dot.get_or_insert_with(|| {
            let span = self.name.span();
            [
                quote!(&).with_replaced_span(span).into_token_tree(),
                quote!(self).with_replaced_span(span).into_token_tree(),
                quote!(.).with_replaced_span(span).into_token_tree(),
            ]
            .into()
        })
    }
}

#[derive(Clone, Copy)]
enum OnlyFieldResult<T> {
    Existing(T, Option<OnlyFieldError>),
    EmptyFields(OnlyFieldError),
}

impl<T> OnlyFieldResult<T> {
    fn report(self, errors: &mut ErrorCollector, span: Span) -> Option<T> {
        let (v, error) = match self {
            OnlyFieldResult::Existing(v, error) => (Some(v), error),
            OnlyFieldResult::EmptyFields(error) => (None, Some(error)),
        };
        if let Some(error) = error {
            errors.push_custom(error.0, span);
        }
        v
    }

    fn map<R>(self, f: impl FnOnce(T) -> R) -> OnlyFieldResult<R> {
        match self {
            OnlyFieldResult::Existing(v, e) => OnlyFieldResult::Existing(f(v), e),
            OnlyFieldResult::EmptyFields(e) => OnlyFieldResult::EmptyFields(e),
        }
    }
}

#[derive(Clone, Copy)]
struct OnlyFieldError(&'static str);

impl IntoParseErrorWithSpan for OnlyFieldError {
    fn into_parse_error_with_span(self, span: Span) -> ParseError {
        ParseError::custom(self.0, span)
    }
}

impl ContextOfStruct {
    fn impl_try_expand_at_field(
        &mut self,
        prop: expand_props::PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) -> Result<(), expand_props::PropPath> {
        enum FirstProp {
            Self_(Span),
            Item(Span),
        }
        let first_prop_type = 'first: {
            match &prop.0 {
                expand_props::Prop::Ident(ident) => ident_match!(match ident {
                    b"self" => break 'first FirstProp::Self_(ident.span()),
                    b"item" => break 'first FirstProp::Item(ident.span()),
                    _ => {}
                }),
                expand_props::Prop::Literal(_) => {}
            }

            return Err(prop);
        };

        let expand_props::PropPath(_first_prop, rest_prop) = prop;

        match first_prop_type {
            FirstProp::Self_(span_self) => {
                self.expand_self(span_self, rest_prop, out, errors);
            }
            FirstProp::Item(span) => 'ret: {
                let mut rest_prop = rest_prop.into_iter();

                let Some(first_prop) = rest_prop.next() else {
                    errors.push_custom("@item on struct field cannot expand to tokens", span);
                    break 'ret;
                };

                self.expand_prop(
                    expand_props::PropPath(first_prop, Vec::from_iter(rest_prop)),
                    out,
                    errors,
                )
            }
        }

        Ok(())
    }
}

trait IntoParseErrorWithSpan {
    fn into_parse_error_with_span(self, span: Span) -> ParseError;
}
