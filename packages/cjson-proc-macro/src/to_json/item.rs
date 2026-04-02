use std::{collections::HashMap, vec};

use proc_macro::{Group, Ident, Literal, Span, TokenTree};
use typed_quote::{IntoTokenTree, IntoTokens, ToTokens, WithSpan, quote};

use crate::{
    ConsumedTokens, ErrorCollector, IdentTree, MetaSimple, ParsingTokenStream, TokenTreeExt,
    expand_props::TokensCollector,
    ident_match,
    syn_generic::{
        EnumVariantBody, GroupBrace, GroupBracket, GroupParen, MetaAfterPath, ParseError,
        StructData,
        parse_meta::ParseMeta,
        parse_meta_utils::{EqValue as EqValueGeneric, FlagPresent, MetaPathSpanWith},
    },
    to_json::ctx::r#enum::MakeContextOfEnum,
};

use super::ctx::{
    ContextOfStruct, CustomTokens, StructField, StructFieldToItemsDefault, StructFieldToKvsDefault,
    StructToDefault,
    r#enum::{ContextOfEnum, EnumVariant, MakeEnumVariant, SpecifiedTagMode, TagMode, VariantType},
};

type EqValue = EqValueGeneric<vec::IntoIter<TokenTree>>;

macro_rules! impl_parse_attrs {
    (
        $(#$attr:tt)*
        $vis:vis struct $Struct:ident {
            $($field_vis:vis $field:ident: $FieldType:ty ),* $(,)?
        }

        fn $push_meta_simple:ident();
    ) => {
        $(#$attr)*
        $vis struct $Struct {
            $($field_vis $field : Option<$FieldType> ),*
        }

        impl PushMetaSimple for $Struct {
            fn $push_meta_simple(
                &mut self,
                MetaSimple { path, after_path }: MetaSimple<ConsumedTokens<'_>>,
                errors: &mut ErrorCollector,
            ) -> Option<IdentTree> {
                mod __private_meta_names {
                    $(
                        pub mod $field {
                            pub const BYTES: &[u8] = ::core::stringify!($field).as_bytes();
                        }
                    )*
                }

                use $crate::const_pattern;

                let children = ident_match!(match path {
                    $(
                    const_pattern!(__private_meta_names::$field::BYTES) => {
                        if self.$field.is_some() {
                            errors.push(ParseError::custom("duplicated attribute", path.span()))
                        }

                        let mut children = Vec::new();

                        match <$FieldType as ParseMeta>::parse_meta(
                            $crate::syn_generic::parse_meta::MetaToParse::from_ident(&path, after_path),
                            errors,
                            From::from(&mut children),
                        ) {
                            Ok(v) => {
                                if self.$field.is_none() {
                                    self.$field = Some(v);
                                }
                            }
                            Err(e) => {
                                errors.push(e);
                            }
                        }

                        children
                    }
                    )*
                    _ => {
                        errors.push(ParseError::custom("unknown attribute", path.span()));
                        return None;
                    },
                });

                Some(IdentTree {
                    ident: path,
                    mod_name: "",
                    children,
                })
            }
        }
    };
}

impl_parse_attrs!(
    #[derive(Default)]
    pub struct ItemAttrsParser {
        to: MetaPathSpanWith<ItemTo>,
        to_untagged: MetaPathSpanWith<EnumToUntagged>,
        to_tagged_kvs: MetaPathSpanWith<StructToTaggedKvs>,
        transparent: FlagPresent,
        rename: MetaPathSpanWith<Rename>,
        tag: MetaPathSpanWith<EqValue>,
        content: MetaPathSpanWith<EqValue>,
        untagged: FlagPresent,
        tag_only: FlagPresent,
        externally_tagged: FlagPresent,
        internally_tagged: FlagPresent,
        adjacently_tagged: FlagPresent,
        rename_variants: MetaPathSpanWith<GroupParen>,
        rename_fields: MetaPathSpanWith<GroupParen>,
    }

    fn push_meta_simple();
);

impl_parse_attrs!(
    #[derive(Default)]
    pub struct StructFieldAttrsParser {
        skip: FlagPresent,
        flatten: FlagPresent,
        rename: MetaPathSpanWith<Rename>,
        to: MetaPathSpanWith<StructFieldTo>,
        to_kvs: MetaPathSpanWith<StructFieldToKvs>,
        to_items: MetaPathSpanWith<StructFieldToItems>,
    }

    fn push_meta_simple();
);

impl_parse_attrs!(
    #[derive(Default)]
    pub struct EnumVariantAttrsParser {
        to: MetaPathSpanWith<ItemTo>,
        to_untagged: MetaPathSpanWith<EnumToUntagged>,
        to_tagged_kvs: MetaPathSpanWith<StructToTaggedKvs>,
        transparent: FlagPresent,
        rename: MetaPathSpanWith<Rename>,
        tag: MetaPathSpanWith<EqValue>,
        content: MetaPathSpanWith<EqValue>,
        untagged: FlagPresent,
        tag_only: FlagPresent,
        externally_tagged: FlagPresent,
        internally_tagged: FlagPresent,
        adjacently_tagged: FlagPresent,
        rename_fields: MetaPathSpanWith<GroupParen>,
    }

    fn push_meta_simple();
);

pub trait PushMetaSimple {
    fn push_meta_simple(
        &mut self,
        meta: MetaSimple<ConsumedTokens<'_>>,
        errors: &mut ErrorCollector,
    ) -> Option<IdentTree>;
}

fn push_outer_attr(
    attrs: &mut impl PushMetaSimple,
    errors: &mut ErrorCollector,
    _pound: crate::syn_generic::PunctPound,
    bracketed_meta: TokenTree,
) -> Option<IdentTree> {
    match GroupBracket::parse_from_token_tree(bracketed_meta) {
        Ok(bracketed_meta) => {
            let ident_tree = crate::push_top_level_attr_meta_with_one(
                bracketed_meta.stream(),
                |meta, errors| attrs.push_meta_simple(meta, errors),
                errors,
            );

            ident_tree
        }
        Err(e) => {
            errors.push(e);
            None
        }
    }
}

fn make_push_outer_attr<Attrs: PushMetaSimple>(
    ident_trees: &mut Vec<IdentTree>,
) -> impl FnMut(&mut Attrs, &mut ErrorCollector, crate::syn_generic::PunctPound, TokenTree) {
    |attrs, errors, pound, bracketed_meta| {
        if let Some(ident_tree) = push_outer_attr(attrs, errors, pound, bracketed_meta) {
            ident_trees.push(ident_tree);
        }
    }
}

pub struct ItemTo(GroupParen);
pub struct EnumToUntagged(GroupParen);
pub struct StructToTaggedKvs(GroupParen);
pub struct StructFieldTo(GroupParen);
pub struct StructFieldToKvs(GroupParen);
pub struct StructFieldToItems(GroupParen);

crate::utils::impl_many!({
    {
        {
            use ItemTo as To;
            macro_rules! dummy {[] => { quote!( null ) }}
        }
        {
            use EnumToUntagged as To;
            macro_rules! dummy {[] => { quote!( null ) }}
        }
        {
            use StructToTaggedKvs as To;
            macro_rules! dummy {[] => { quote!( {} ) }}
        }
        {
            use StructFieldTo as To;
            macro_rules! dummy {[] => { quote!( null ) }}
        }
        {
            use StructFieldToKvs as To;
            macro_rules! dummy {[] => { quote!( {} ) }}
        }
        {
            use StructFieldToItems as To;
            macro_rules! dummy {[] => { quote!( [] ) }}
        }
    }
    impl To {
        fn dummy(span: Span) -> Self {
            Self(
                GroupParen::new(dummy!().with_default_span(span).into_token_stream())
                    .with_delimiter_span(span),
            )
        }
    }

    impl ParseMeta<'_> for To {
        fn parse_meta(
            input: crate::syn_generic::parse_meta::MetaToParse<'_, '_>,
            errors: &mut ErrorCollector,
            ident_trees: crate::syn_generic::parse_meta::IdentTreeCollector<'_>,
        ) -> Result<Self, ParseError> {
            let span = input.path_span();
            Ok(
                GroupParen::parse_meta(input, errors, ident_trees).map_or_else(
                    |e| {
                        errors.push(e);
                        Self::dummy(span)
                    },
                    Self,
                ),
            )
        }
    }

    impl From<To> for GroupParen {
        fn from(value: To) -> Self {
            value.0
        }
    }
});

pub enum Rename {
    Paren(GroupParen),
    Eq(EqValue),
}

pub trait RenameAbleName: ToTokens {}

impl RenameAbleName for &Ident {}
impl RenameAbleName for &typed_quote::Either<Ident, Literal> {}

impl Rename {
    pub fn to_tokens_as_json_object_key(
        &self,
        crate_path: impl ToTokens,
        rename_span: Span,
        name: impl RenameAbleName,
    ) -> Vec<TokenTree> {
        let mut out = vec![];
        self.expand_as_json_object_key(From::from(&mut out), crate_path, rename_span, name);
        out
    }
    fn expand_as_json_object_key(
        &self,
        mut out: TokensCollector<'_>,
        crate_path: impl ToTokens,
        rename_span: Span,
        name: impl RenameAbleName,
    ) {
        match self {
            Rename::Paren(group_paren) => {
                let group_paren: &Group = &group_paren;
                out.extend([
                    quote!(const)
                        .with_default_span(group_paren.span_open())
                        .into_token_tree(),
                    {
                        let rename_bang = quote!(rename !).with_default_span(rename_span);
                        quote!({ #crate_path ::__private::proc_macro:: #rename_bang ( #group_paren #name ) })
                    }
                    .into_token_tree(),
                ]);
            }
            Rename::Eq(EqValue { eq: _, value }) => {
                out.extend_from_slice(value.as_slice());
            }
        }
    }
}

impl ParseMeta<'_> for Rename {
    fn parse_meta(
        input: crate::syn_generic::parse_meta::MetaToParse<'_, '_>,
        _: &mut ErrorCollector,
        mut ident_trees: crate::syn_generic::parse_meta::IdentTreeCollector<'_>,
    ) -> Result<Self, ParseError> {
        let err_span = match input.after_path {
            MetaAfterPath::Empty => input.path_span(),
            MetaAfterPath::Group(group) => match GroupParen::try_from(group) {
                Ok(g) => {
                    'it: {
                        let mut s = g.stream().into_iter();
                        let ident = match s.next() {
                            Some(TokenTree::Ident(ident)) => ident,
                            _ => break 'it,
                        };

                        let mut children = Vec::new();

                        {
                            let mut to_push = &mut children;
                            loop {
                                match s.next() {
                                    Some(TokenTree::Punct(p)) if p == '-' => {}
                                    _ => break,
                                }

                                let Some(TokenTree::Ident(ident)) = s.next() else {
                                    break;
                                };

                                to_push.push(IdentTree {
                                    ident,
                                    mod_name: "",
                                    children: vec![],
                                });

                                to_push = &mut to_push.last_mut().unwrap().children;
                            }
                        }

                        ident_trees.push(IdentTree {
                            ident,
                            mod_name: "",
                            children,
                        });
                    }
                    return Ok(Self::Paren(g));
                }
                Err(g) => g.span_open(),
            },
            MetaAfterPath::Eq {
                eq,
                before_comma_or_eof,
            } => {
                return Ok(Self::Eq(EqValue {
                    eq,
                    value: before_comma_or_eof.into_vec_iter(),
                }));
            }
        };

        return Err(ParseError::custom(
            "expect `($rename_fn)` or `= \"name\"`",
            err_span,
        ));
    }
}

pub struct StructAttrs {
    to: Option<MetaPathSpanWith<ItemTo>>,
    to_tagged_kvs: Option<MetaPathSpanWith<StructToTaggedKvs>>,
    transparent: Option<FlagPresent>,
    rename: Option<MetaPathSpanWith<Rename>>,
    tag: Option<MetaPathSpanWith<EqValue>>,
    rename_fields: Option<MetaPathSpanWith<GroupParen>>,
}

impl StructAttrs {
    pub fn parse(
        self,
        name: Ident,
        struct_data: StructData,
        errors: &mut ErrorCollector,
        field_ident_trees: &mut Vec<IdentTree>,
        options: super::ctx::Options,
    ) -> ContextOfStruct {
        let Self {
            to,
            to_tagged_kvs,
            transparent,
            rename,
            tag,
            rename_fields,
        } = self;

        let ParsedFields {
            fields,
            fields_ident_to_index,
            to_untagged_unspecified,
        } = match struct_data {
            StructData::Paren { paren, semi: _ } => Fields::Paren(paren),
            StructData::Brace(g) => Fields::Brace(g),
            StructData::Semi(_) => Fields::Unit,
        }
        .parse(name.span(), transparent, errors, field_ident_trees);

        super::ctx::MakeContextOfStruct {
            name,
            rename,
            rename_fields,
            options,
            fields,
            fields_ident_to_index,
            to_default: to_untagged_unspecified,
            to_custom: to.map(CustomTokens::from),
            to_tagged_kvs: to_tagged_kvs.map(CustomTokens::from),
            tag: From::from(tag),
        }
        .into()
    }
}

enum Fields {
    Unit,
    Paren(GroupParen),
    Brace(GroupBrace),
}

struct ParsedFields {
    fields: Vec<StructField>,
    fields_ident_to_index: Option<HashMap<String, usize>>,
    to_untagged_unspecified: StructToDefault,
}

impl Fields {
    fn parse(
        self,
        default_span: Span,
        transparent: Option<FlagPresent>,
        errors: &mut ErrorCollector,
        ident_trees: &mut Vec<IdentTree>,
    ) -> ParsedFields {
        let fields;
        let fields_ident_to_index;

        let to_default_opt = match &transparent {
            Some(transparent) => Some(StructToDefault::Transparent {
                span: Some(transparent.0),
            }),
            None => None,
        };

        let to_default;

        let non_skip_field_count: usize;

        match self {
            Self::Paren(paren) => {
                fields_ident_to_index = None;

                let mut fs = vec![];

                let mut non_skip_field_len = 0;

                let res = ParsingTokenStream::from(paren.stream()).parse_into_unnamed_fields(
                    errors,
                    |_| StructFieldAttrsParser::default(),
                    make_push_outer_attr(ident_trees),
                    |_, attrs, _vis, ty, comma| {
                        if attrs.skip.is_none() {
                            non_skip_field_len += 1;
                        }

                        let i = fs.len();
                        let span = ty
                            .as_slice()
                            .first()
                            .map(TokenTreeExt::span_open_or_entire)
                            .or_else(|| comma.map(|v| v.span()))
                            .unwrap_or(default_span);

                        let name = typed_quote::Either::B(
                            Literal::usize_unsuffixed(i).with_replaced_span(span),
                        );

                        let f = attrs.make_struct_field(name, ty);
                        fs.push(f);
                    },
                );

                match res {
                    Ok(()) => {}
                    Err(e) => errors.push(e),
                }

                fields = fs;
                non_skip_field_count = non_skip_field_len;
                to_default = to_default_opt.unwrap_or_else(|| {
                    if fields.len() == 1 && non_skip_field_count == 1 {
                        // Implicitly transparent struct
                        StructToDefault::Transparent { span: None }
                    } else {
                        StructToDefault::Array
                    }
                });
            }
            Self::Brace(brace) => {
                to_default = to_default_opt.unwrap_or(StructToDefault::Object);

                let mut ident_to_index = HashMap::new();

                let mut fs = vec![];

                let mut non_skip_field_len = 0;

                let res = ParsingTokenStream::from(brace.stream()).parse_into_named_fields(
                    errors,
                    |_| StructFieldAttrsParser::default(),
                    make_push_outer_attr(ident_trees),
                    |_, attrs, _vis, name, _colon, ty, _comma| {
                        if attrs.skip.is_none() {
                            non_skip_field_len += 1;
                        }

                        let i = fs.len();
                        ident_to_index.insert(name.to_string(), i);

                        let name = typed_quote::Either::A(name);
                        let f = attrs.make_struct_field(name, ty);
                        fs.push(f);
                    },
                );

                match res {
                    Ok(()) => {}
                    Err(e) => errors.push(e),
                }

                fields = fs;
                non_skip_field_count = non_skip_field_len;
                fields_ident_to_index = Some(ident_to_index);
            }
            Self::Unit => {
                non_skip_field_count = 0;
                fields = vec![];
                fields_ident_to_index = None;
                to_default = to_default_opt.unwrap_or(StructToDefault::Unit);
            }
        }

        if let Some(transparent) = &transparent {
            if non_skip_field_count != 1 {
                errors.push(ParseError::custom(
                    "`transparent` only works with struct with exactly one non-skipped field",
                    transparent.0,
                ));
            }
        }

        ParsedFields {
            fields,
            fields_ident_to_index,
            to_untagged_unspecified: to_default,
        }
    }
}

impl ItemAttrsParser {
    pub fn r#struct(self, errors: &mut ErrorCollector) -> StructAttrs {
        let Self {
            to,
            to_untagged,
            to_tagged_kvs,
            transparent,
            rename,
            tag,
            content,
            untagged,
            tag_only,
            externally_tagged,
            internally_tagged,
            adjacently_tagged,
            rename_variants,
            rename_fields,
        } = self;

        if let Some(to_untagged) = to_untagged {
            errors.push_custom(
                "not working with struct\nuse #[cjson(to_untagged(...))] instead",
                to_untagged.0,
            );
        }

        [
            content.map(|v| v.0),
            untagged.map(|v| v.0),
            tag_only.map(|v| v.0),
            externally_tagged.map(|v| v.0),
            internally_tagged.map(|v| v.0),
            adjacently_tagged.map(|v| v.0),
            rename_variants.map(|v| v.0),
        ]
        .into_iter()
        .filter_map(|v| v)
        .for_each(|span| errors.push(ParseError::custom("not working with struct", span)));

        StructAttrs {
            to,
            to_tagged_kvs,
            transparent,
            rename,
            tag,
            rename_fields,
        }
    }

    pub fn r#enum(self, errors: &mut ErrorCollector) -> EnumAttrs {
        let Self {
            to,
            to_untagged,
            to_tagged_kvs,
            transparent,
            rename,
            tag,
            content,
            untagged,
            tag_only,
            externally_tagged,
            internally_tagged,
            adjacently_tagged,
            rename_variants,
            rename_fields,
        } = self;

        if let Some(transparent) = transparent {
            errors.push_custom(
                "not working with enum\nspecify it on enum variants instead",
                transparent.0,
            );
        }

        EnumAttrs {
            to,
            to_untagged,
            to_tagged_kvs,
            rename,
            tag,
            content,
            untagged,
            tag_only,
            externally_tagged,
            internally_tagged,
            adjacently_tagged,
            rename_variants,
            rename_fields,
        }
    }
}

impl StructFieldAttrsParser {
    fn make_struct_field(
        self,
        name: typed_quote::Either<Ident, Literal>,
        ty: crate::syn_generic::TokenStreamCow<'_>,
    ) -> StructField {
        let StructFieldAttrsParser {
            skip,
            flatten,
            rename,
            to,
            to_kvs,
            to_items,
        } = self;

        let to_kvs_default;
        let to_items_default;

        if let Some(flatten) = flatten {
            let span = flatten.0;
            to_kvs_default = StructFieldToKvsDefault::Flatten { span };
            to_items_default = StructFieldToItemsDefault::Flatten { span };
        } else {
            to_kvs_default = StructFieldToKvsDefault::BracedNameEqTo;
            to_items_default = StructFieldToItemsDefault::BracketedTo;
        }

        let ty = ty.into_vec_iter();

        super::ctx::MakeStructField {
            skip,
            name,
            type_: ty,
            rename,
            to: to.map(From::from),
            to_kvs_default,
            to_kvs_custom: to_kvs.map(From::from),
            to_items_default,
            to_items_custom: to_items.map(From::from),
        }
        .into()
    }
}

pub struct EnumAttrs {
    to: Option<MetaPathSpanWith<ItemTo>>,
    to_untagged: Option<MetaPathSpanWith<EnumToUntagged>>,
    to_tagged_kvs: Option<MetaPathSpanWith<StructToTaggedKvs>>,
    rename: Option<MetaPathSpanWith<Rename>>,
    tag: Option<MetaPathSpanWith<EqValue>>,
    content: Option<MetaPathSpanWith<EqValue>>,
    untagged: Option<FlagPresent>,
    tag_only: Option<FlagPresent>,
    externally_tagged: Option<FlagPresent>,
    internally_tagged: Option<FlagPresent>,
    adjacently_tagged: Option<FlagPresent>,
    rename_variants: Option<MetaPathSpanWith<GroupParen>>,
    rename_fields: Option<MetaPathSpanWith<GroupParen>>,
}

impl EnumAttrs {
    pub fn parse(
        self,
        name: Ident,
        enum_brace: GroupBrace,
        errors: &mut ErrorCollector,
        variant_ident_trees: &mut Vec<IdentTree>,
        options: super::ctx::Options,
    ) -> ContextOfEnum {
        let Self {
            to,
            to_untagged,
            to_tagged_kvs,
            rename,
            tag,
            content,
            untagged,
            tag_only,
            externally_tagged,
            internally_tagged,
            adjacently_tagged,
            rename_variants,
            rename_fields,
        } = self;
        MakeContextOfEnum {
            name,
            rename,
            rename_variants,
            rename_fields,
            specified_tag_mode: TagModeAttrs {
                untagged,
                tag_only,
                externally_tagged,
                internally_tagged,
                adjacently_tagged,
            }
            .into_tag_mode(errors),
            tag,
            content,
            to: to.map(From::from),
            to_untagged: to_untagged.map(From::from),
            to_tagged_kvs: to_tagged_kvs.map(From::from),
            variants: parse_enum_variants(enum_brace, errors, variant_ident_trees),
            options,
        }
        .into()
    }
}

fn parse_enum_variants(
    enum_brace: GroupBrace,
    errors: &mut ErrorCollector,
    variant_ident_trees: &mut Vec<IdentTree>,
) -> Vec<EnumVariant> {
    let mut vars = Vec::new();
    let res = ParsingTokenStream::from(enum_brace.stream()).parse_into_variants(
        &mut (&mut *errors, variant_ident_trees),
        |_| EnumVariantAttrsParser::default(),
        |attrs, (errors, variant_ident_trees), pound, bracketed_meta| {
            if let Some(ident_tree) = push_outer_attr(attrs, errors, pound, bracketed_meta) {
                variant_ident_trees.push(ident_tree);
            }
        },
        |(errors, variant_ident_trees), attrs, var, _comma| {
            let EnumVariantAttrsParser {
                to,
                to_untagged,
                to_tagged_kvs,
                transparent,
                rename,
                tag,
                content,
                untagged,
                tag_only,
                externally_tagged,
                internally_tagged,
                adjacently_tagged,
                rename_fields,
            } = attrs;
            let crate::syn_generic::EnumVariant {
                vis: _,
                name,
                body,
                discriminant,
            } = var;

            let variant_type;
            let fields;
            match body {
                EnumVariantBody::Unit => {
                    variant_type = VariantType::Unit;
                    fields = Fields::Unit;
                }
                EnumVariantBody::Paren(g) => {
                    variant_type = VariantType::Array;
                    fields = Fields::Paren(g);
                }
                EnumVariantBody::Brace(g) => {
                    variant_type = VariantType::Object;
                    fields = Fields::Brace(g);
                }
            };

            let mut field_ident_trees = Vec::new();

            let ParsedFields {
                fields,
                fields_ident_to_index,
                to_untagged_unspecified,
            } = fields.parse(name.span(), transparent, errors, &mut field_ident_trees);

            variant_ident_trees.push(IdentTree {
                ident: Ident::new("field", Span::call_site()),
                mod_name: "",
                children: field_ident_trees,
            });

            let var = MakeEnumVariant {
                name,
                discriminant,
                variant_type,
                rename,
                rename_fields: rename_fields.map(|MetaPathSpanWith(span, paren)| {
                    MetaPathSpanWith(span, Rename::Paren(paren))
                }),
                specified_tag_mode: TagModeAttrs {
                    untagged,
                    tag_only,
                    externally_tagged,
                    internally_tagged,
                    adjacently_tagged,
                }
                .into_tag_mode(errors),
                tag,
                content,
                fields,
                fields_ident_to_index,
                to: to.map(From::from),
                to_untagged_unspecified,
                to_untagged: to_untagged.map(From::from),
                to_tagged_kvs: to_tagged_kvs.map(From::from),
            }
            .into();

            vars.push(var);
        },
    );

    if let Err(e) = res {
        errors.push(e);
    }

    vars
}

pub struct TagModeAttrs {
    untagged: Option<FlagPresent>,
    tag_only: Option<FlagPresent>,
    externally_tagged: Option<FlagPresent>,
    internally_tagged: Option<FlagPresent>,
    adjacently_tagged: Option<FlagPresent>,
}

impl TagModeAttrs {
    fn into_tag_mode(self, errors: &mut ErrorCollector) -> Option<SpecifiedTagMode> {
        let Self {
            untagged,
            tag_only,
            externally_tagged,
            internally_tagged,
            adjacently_tagged,
        } = self;
        [
            (untagged, TagMode::Untagged),
            (tag_only, TagMode::TagOnly),
            (externally_tagged, TagMode::ExternallyTagged),
            (internally_tagged, TagMode::InternallyTagged),
            (adjacently_tagged, TagMode::AdjacentlyTagged),
        ]
        .into_iter()
        .fold(None, |acc, (flag, mode)| {
            if acc.is_some() {
                if let Some(flag) = flag {
                    errors.push_custom("tag mode can only be specified once", flag.0);
                }
                acc
            } else {
                flag.map(|flag| SpecifiedTagMode::new(flag.0, mode))
            }
        })
    }
}
