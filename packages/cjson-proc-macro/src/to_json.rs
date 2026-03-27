use proc_macro::{Ident, Punct, Span, TokenStream, TokenTree};
use typed_quote::{Either, IntoTokens, quote, tokens::IterTokens};

use crate::{
    ErrorCollector, ident_match,
    syn_generic::{
        self, GroupBrace, ParseError, ParseGenericsOutput, SomeVisibility, StructData, WhereClause,
        with_trailing_punct_if_not_empty,
    },
    to_json::item::Options,
};

pub mod item;

pub struct ToJson<'a> {
    pub input: &'a mut syn_generic::ParsingTokenStream,
    pub first_ident: proc_macro::Ident,
    pub append_where_clause: Option<(Span, TokenStream)>,
    pub item_attrs: item::ItemAttrsParser,
}

impl<'a> ToJson<'a> {
    pub fn try_parse(
        self,
        errors: &mut ErrorCollector,
        crate_path: TokenStream,
    ) -> Result<ToJsonItem, ParseError> {
        let Self {
            input,
            first_ident,
            append_where_clause,
            item_attrs,
        } = self;

        enum Kind {
            Struct,
            Enum,
        }

        let kind = ident_match!(match first_ident {
            b"struct" => Kind::Struct,
            b"enum" => Kind::Enum,
            _ =>
                return Err(ParseError::custom(
                    "expect `struct` or `enum`",
                    first_ident.span()
                )),
        });

        let item_name = input.parse_ident()?;

        let ParseGenericsOutput {
            impl_generics,
            ty_generics,
        } = match input.parse_generics() {
            Ok(v) => v,
            Err(e) => {
                errors.push(e);
                Default::default()
            }
        };

        let where_clause;

        let data = match kind {
            Kind::Struct => {
                let struct_data;
                (where_clause, struct_data) = input.parse_struct_after_generics()?;

                let ctx = item_attrs.r#struct(errors).parse(
                    item_name.clone(),
                    struct_data,
                    errors,
                    Options { crate_path },
                );

                ToJsonItemData::Struct(ctx.into_to_json(errors))
            }
            Kind::Enum => {
                let enum_brace;
                (where_clause, enum_brace) = input.parse_enum_after_generics()?;
                ToJsonItemData::Enum(enum_brace)
            }
        };

        let where_clause = where_clause.map(
            |WhereClause {
                 r#where,
                 predicates,
             }| {
                WhereClause {
                    r#where,
                    predicates: syn_generic::with_trailing_punct_if_not_empty(
                        predicates.into_vec(),
                        ',',
                    ),
                }
            },
        );

        if let Err(e) = input.expect_eof() {
            errors.push(e);
        }

        let where_clause = match (where_clause, append_where_clause) {
            (v, None::<_>) => v,
            (None, Some((span, bounds))) => Some(WhereClause {
                r#where: span.into(),
                predicates: {
                    with_trailing_punct_if_not_empty(
                        //
                        bounds.into_iter().collect::<Vec<_>>(),
                        ',',
                    )
                },
            }),
            (Some(mut where_clause), Some((_, bounds))) => {
                where_clause.predicates.extend(bounds);

                where_clause.predicates =
                    with_trailing_punct_if_not_empty(where_clause.predicates, ',');

                Some(where_clause)
            }
        };

        Ok(ToJsonItem {
            name: item_name,
            impl_generics,
            ty_generics,
            where_clause,
            data,
        })
    }
}

pub struct ToJsonItem {
    name: Ident,
    impl_generics: TokenStream,
    ty_generics: TokenStream,
    where_clause: Option<WhereClause<Vec<TokenTree>>>,
    data: ToJsonItemData,
}
impl ToJsonItem {
    pub fn into_tokens(self, crate_path: impl IntoTokens) -> impl IntoTokens {
        let Self {
            name,
            impl_generics,
            ty_generics,
            where_clause,
            data,
        } = self;

        let where_clause = where_clause.map(
            |WhereClause {
                 r#where,
                 predicates,
             }| {
                let r#where: Ident = r#where.into();
                let predicates = IterTokens(predicates);
                quote!(
                    #r#where
                    #predicates
                )
            },
        );

        let data = data.into_tokens();

        quote!(
            #crate_path ::impl_to_json! {
                impl_generics![#impl_generics],
                where_clause![#where_clause],
                |self: #name< #ty_generics >|
                    #data
            }
        )
    }
}

enum ToJsonItemData {
    Struct(Vec<TokenTree>),
    Enum(GroupBrace),
}

impl ToJsonItemData {
    fn into_tokens(self) -> impl IntoTokens {
        match self {
            ToJsonItemData::Struct(ts) => Either::A({
                let ts = TokenStream::from_iter(ts);
                ts
            }),
            ToJsonItemData::Enum(group_brace) => Either::B({
                let group_brace: proc_macro::Group = group_brace.into();
                quote!(enum #group_brace)
            }),
        }
    }
}
