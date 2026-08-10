use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use typed_quote::{Either, IntoTokens, WithSpan, quote, tokens::IterTokens};

use crate::{
    DeriveWhich, ErrorCollector, IdentTree, ItemAttrWhere, ident_match,
    syn_generic::{
        self, ParseError, ParseGenericsOutput, WhereClause, parse_meta_utils::MetaPathSpanWith,
        with_trailing_punct_if_not_empty,
    },
    to_json::{
        ctx::Options,
        item::{GroupMaybeFromEq, GroupOrExpr},
    },
};

pub mod item;

mod ctx;

pub struct ToJson<'a> {
    pub input: &'a mut syn_generic::ParsingTokenStream,
    pub first_ident: proc_macro::Ident,
    pub append_where_clause: Option<ItemAttrWhere>,
    pub special_attrs: item::ItemSpecialAttrsParser,
    pub item_attrs: item::ItemAttrsParser,
}

impl<'a> ToJson<'a> {
    pub fn try_parse(
        self,
        errors: &mut ErrorCollector,
        crate_path: TokenStream,
        ident_trees: &mut Vec<IdentTree>,
    ) -> Result<ToJsonItem, ParseError> {
        let Self {
            input,
            first_ident,
            append_where_clause,
            special_attrs:
                item::ItemSpecialAttrsParser {
                    where_to,
                    where_into,
                    derive_from,
                    is_chainable_and_always_empty,
                    json_kind,
                    any_value,
                },
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
                if let Some(any_value) = any_value {
                    errors.push_custom("struct doesn't support any_value", any_value.0);
                }

                let struct_data;
                (where_clause, struct_data) = input.parse_struct_after_generics()?;

                let mut field_ident_trees = vec![];
                let ctx = item_attrs.r#struct(errors).parse(
                    item_name.clone(),
                    struct_data,
                    errors,
                    &mut field_ident_trees,
                    Options { crate_path },
                );

                ident_trees.push(IdentTree {
                    ident: Ident::new("field", Span::call_site()),
                    mod_name: "",
                    children: field_ident_trees,
                });

                ToJsonItemData::Struct(ctx.into_to_json(errors))
            }
            Kind::Enum => {
                let enum_brace;
                (where_clause, enum_brace) = input.parse_enum_after_generics()?;

                let mut variant_ident_trees = vec![];
                let ctx = item_attrs.r#enum(errors).parse(
                    item_name.clone(),
                    enum_brace,
                    errors,
                    &mut variant_ident_trees,
                    Options { crate_path },
                );

                ident_trees.push(IdentTree {
                    ident: Ident::new("variant", Span::call_site()),
                    mod_name: "",
                    children: variant_ident_trees,
                });

                ToJsonItemData::Enum(ctx.into_to_json(any_value.map(|v| v.0), errors))
            }
        };

        let where_clause = where_clause.map(
            |WhereClause {
                 r#where,
                 predicates,
             }| {
                WhereClause {
                    r#where,
                    predicates: predicates.into_vec(),
                }
            },
        );

        if let Err(e) = input.expect_eof() {
            errors.push(e);
        }

        let where_clause = join_where(where_clause, append_where_clause);

        Ok(ToJsonItem {
            name: item_name,
            impl_generics,
            ty_generics,
            where_clause,
            where_to,
            where_into,
            derive_from,
            is_chainable_and_always_empty,
            json_kind,
            data,
        })
    }
}

pub struct ToJsonItem {
    name: Ident,
    impl_generics: TokenStream,
    ty_generics: TokenStream,
    where_clause: Option<JointWhere>,
    where_to: Option<ItemAttrWhere>,
    where_into: Option<ItemAttrWhere>,
    derive_from: Option<MetaPathSpanWith<GroupMaybeFromEq>>,
    is_chainable_and_always_empty: Option<MetaPathSpanWith<GroupOrExpr>>,
    json_kind: Option<MetaPathSpanWith<GroupMaybeFromEq>>,
    data: ToJsonItemData,
}
impl ToJsonItem {
    pub fn into_tokens(
        self,
        crate_path: impl IntoTokens,
        derive_which: DeriveWhich,
    ) -> impl IntoTokens {
        let Self {
            name,
            impl_generics,
            ty_generics,
            mut where_clause,
            mut where_to,
            mut where_into,
            derive_from,
            is_chainable_and_always_empty,
            json_kind,
            data,
        } = self;

        // 1. Silently ignore the irrelevant where clauses in case both #[derive(ToJson, IntoJson)]
        // 2. Merge where_[in]to to where
        'merge: {
            let where_append = match derive_which {
                DeriveWhich::Both => break 'merge,
                DeriveWhich::IntoJson => {
                    where_to = None;
                    where_into.take()
                }
                DeriveWhich::ToJson => {
                    where_into = None;
                    where_to.take()
                }
            };

            where_clause = match (where_clause, where_append) {
                (None, Some(ItemAttrWhere { where_span, bound })) => Some(JointWhere {
                    where_span,
                    predicates: Either::B(bound),
                }),
                (Some(w), Some(where_append)) => Some(w.concat(where_append)),
                (v, None) => v,
            };
        }

        let derive_from = derive_from.map(|MetaPathSpanWith(span, group)| {
            let group = make_bracket(group.into_group());
            quote!(derive_from! #group,).with_default_span(span)
        });

        let where_clause = where_clause.map(|mut w| {
            let should_make_sure_trailing_comma = where_to.is_some() || where_into.is_some();

            if should_make_sure_trailing_comma {
                w = w.with_trailing_comma(None);
            }

            let JointWhere {
                where_span,
                predicates,
            } = w;
            make_where_bang(where_span, quote!(where_clause!), predicates)
        });

        let where_clause_to = where_to.map(|ItemAttrWhere { where_span, bound }| {
            make_where_bang(where_span, quote!(where_clause_to!), bound)
        });
        let where_clause_into = where_into.map(|ItemAttrWhere { where_span, bound }| {
            make_where_bang(where_span, quote!(where_clause_into!), bound)
        });

        let is_chainable_and_always_empty =
            is_chainable_and_always_empty.map(|MetaPathSpanWith(span, group)| {
                let group = make_bracket(group.make_group());
                quote!(IS_CHAINABLE_AND_ALWAYS_EMPTY! #group,).with_default_span(span)
            });

        let json_kind = json_kind.map(|MetaPathSpanWith(span, group)| {
            let group = make_bracket(group.into_group());
            quote!(JsonKind! #group,).with_default_span(span)
        });

        let data = data.into_tokens();

        let macro_name = match derive_which {
            DeriveWhich::Both => quote!(impl_json).as_ident(),
            DeriveWhich::IntoJson => quote!(impl_into_json).as_ident(),
            DeriveWhich::ToJson => quote!(impl_to_json).as_ident(),
        };

        quote!(
            #crate_path::#macro_name!(
                impl_generics![#impl_generics],
                #derive_from
                #where_clause
                #where_clause_to
                #where_clause_into
                #json_kind
                #is_chainable_and_always_empty
                |self: #name< #ty_generics >|
                    #data
            );
        )
    }
}

enum ToJsonItemData {
    Struct(Vec<TokenTree>),
    Enum(Vec<TokenTree>),
}

impl ToJsonItemData {
    fn into_tokens(self) -> impl IntoTokens {
        let ts = match self {
            ToJsonItemData::Struct(ts) => ts,
            ToJsonItemData::Enum(ts) => ts,
        };
        TokenStream::from_iter(ts)
    }
}

fn make_where_bang(
    span: Span,
    where_bang: impl IntoTokens + WithSpan,
    predicates: impl IntoTokens,
) -> impl IntoTokens {
    let where_bang = where_bang.with_default_span(span);
    quote!(
        #where_bang
        [#predicates],
    )
}

struct JointWhere {
    where_span: Span,
    predicates: Either<IterTokens<Vec<TokenTree>>, TokenStream>,
}

impl JointWhere {
    fn with_trailing_comma(self, span: Option<Span>) -> Self {
        let Self {
            where_span,
            predicates,
        } = self;

        let predicates = match predicates {
            Either::A(IterTokens(predicates)) => predicates,
            Either::B(ts) => ts.into_iter().collect(),
        };

        let predicates =
            with_trailing_punct_if_not_empty(predicates, ',', Some(span.unwrap_or(where_span)));

        Self {
            where_span,
            predicates: Either::A(IterTokens(predicates)),
        }
    }

    fn concat(mut self, ItemAttrWhere { where_span, bound }: ItemAttrWhere) -> Self {
        self = self.with_trailing_comma(Some(where_span));
        self.predicates = {
            let predicates = self.predicates;
            Either::B(quote!(#predicates #bound).into_token_stream())
        };

        self
    }
}

fn join_where(
    where_clause: Option<WhereClause<Vec<TokenTree>>>,
    append_where_clause: Option<ItemAttrWhere>,
) -> Option<JointWhere> {
    match (where_clause, append_where_clause) {
        (None::<_>, None::<_>) => None,
        (
            Some(WhereClause {
                r#where,
                predicates,
            }),
            None::<_>,
        ) => Some(JointWhere {
            where_span: r#where.span(),
            predicates: Either::A(IterTokens(predicates)),
        }),
        (None, Some(ItemAttrWhere { where_span, bound })) => Some(JointWhere {
            where_span,
            predicates: Either::B(bound),
        }),
        (
            Some(WhereClause {
                r#where,
                mut predicates,
            }),
            Some(ItemAttrWhere { where_span, bound }),
        ) => {
            predicates = with_trailing_punct_if_not_empty(predicates, ',', Some(where_span));
            predicates.extend(bound);

            Some(JointWhere {
                where_span: r#where.span(),
                predicates: Either::A(IterTokens(predicates)),
            })
        }
    }
}

fn make_bracket(g: Group) -> Group {
    make_delimiter(g, Delimiter::Bracket)
}

fn make_delimiter(g: Group, delimiter: Delimiter) -> Group {
    if g.delimiter() == delimiter {
        return g;
    }

    let span = g.span();
    let stream = g.stream();

    let mut g = Group::new(delimiter, stream);
    g.set_span(span);

    g
}
