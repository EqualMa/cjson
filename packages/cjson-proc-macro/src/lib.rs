use std::convert::identity;

use proc_macro::{Delimiter, Group, Ident, Punct, Span, TokenStream, TokenTree};
use typed_quote::{prelude::*, tokens::IterTokens};

use crate::{
    syn_generic::{
        ConsumedTokens, ErrorCollector, MetaSimple, ParseItemStart, ParsingTokenStream,
        TokenTreeExt,
        ident_eq::{
            self, __ident_match_after_fat_arrow, __ident_match_parse_pats, const_pattern,
            ident_match, ident_matches,
        },
    },
    to_json::item::PushMetaSimple as _,
};

mod utils;

mod syn_generic;

mod to_json;

// mod to;

mod expand_props;

#[derive(Default)]
struct ItemAttrsParser {
    crate_path: Option<TokenStream>,
    r#where: Option<ItemAttrWhere>,

    item_attrs: to_json::item::ItemAttrsParser,
}

struct ItemAttrWhere {
    where_span: Span,
    bound: TokenStream,
}

struct IdentTree {
    ident: Ident,
    mod_name: &'static str,
    children: Vec<IdentTree>,
}

fn make_ident(name: &str, span: Span) -> Ident {
    if let Some(name) = name.strip_prefix("r#") {
        Ident::new_raw(name, span)
    } else {
        Ident::new(name, span)
    }
}

impl IdentTree {
    fn into_name_and_children(self) -> (Ident, Vec<IdentTree>) {
        (
            if self.mod_name.is_empty() {
                self.ident
            } else {
                make_ident(self.mod_name, self.ident.span())
            },
            self.children,
        )
    }

    fn into_tokens(self) -> impl IntoTokens {
        let (ident, children) = self.into_name_and_children();
        let ts = IterTokens(children.into_iter().map(|v| {
            let ts = v.into_tokens();
            let ts = quote!( #ts , );

            Box::new(ts) as Box<dyn IntoTokens>
        }));

        quote!(#ident::{
            #ts
        })
    }
}

struct Config<V> {
    name: (Ident, Vec<Ident>),
    value: V,
}

fn push_top_level_attr_meta_with_one(
    attr_meta: TokenStream,
    extend_one_attr_meta: impl FnMut(
        MetaSimple<ConsumedTokens<'_>>,
        &mut ErrorCollector,
    ) -> Option<IdentTree>,
    errors: &mut ErrorCollector,
) -> Option<IdentTree> {
    push_top_level_attr_meta(
        attr_meta,
        |attr_meta, errors| extend_attr_meta(attr_meta, extend_one_attr_meta, errors),
        errors,
    )
}

fn push_top_level_attr_meta(
    attr_meta: TokenStream,
    extend_attr_meta: impl FnOnce(TokenStream, &mut ErrorCollector) -> Vec<IdentTree>,
    errors: &mut ErrorCollector,
) -> Option<IdentTree> {
    match try_push_top_level_attr_meta(attr_meta, extend_attr_meta, errors) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push(e);
            None
        }
    }
}

fn try_push_top_level_attr_meta(
    attr_meta: TokenStream,
    extend_attr_meta: impl FnOnce(TokenStream, &mut ErrorCollector) -> Vec<IdentTree>,
    errors: &mut ErrorCollector,
) -> Result<IdentTree, syn_generic::ParseError> {
    let mut input: syn_generic::ParsingTokenStream = attr_meta.into();
    let input = &mut input;

    let cjson = input.next_or_error()?;
    let cjson = match cjson {
        TokenTree::Ident(ident) if ident_matches!(ident, b"cjson") => ident,
        _ => {
            return Err(syn_generic::ParseError::custom(
                "expect `cjson`",
                cjson.span_open_or_entire(),
            ));
        }
    };

    let mut sub_ident_trees = vec![];

    {
        let after_path = syn_generic::parse_meta_after_path(input);

        match after_path {
            syn_generic::MetaAfterPath::Empty => {}
            syn_generic::MetaAfterPath::Group(group) => {
                sub_ident_trees = extend_attr_meta(group.stream(), errors);
            }
            syn_generic::MetaAfterPath::Eq {
                eq,
                before_comma_or_eof: _,
            } => {
                errors.push(syn_generic::ParseError::custom("expect `()`", eq.span()));
            }
        }
    }

    if let Err(err) = input.expect_eof() {
        errors.push(err)
    }

    Ok(IdentTree {
        ident: cjson,
        mod_name: "",
        children: sub_ident_trees,
    })
}

fn extend_attr_meta(
    attr_meta: TokenStream,
    mut extend_one_attr_meta: impl FnMut(
        MetaSimple<ConsumedTokens<'_>>,
        &mut ErrorCollector,
    ) -> Option<IdentTree>,
    errors: &mut ErrorCollector,
) -> Vec<IdentTree> {
    let ref mut input: syn_generic::ParsingTokenStream = attr_meta.into();

    let mut ident_trees = vec![];

    let res = (|| {
        syn_generic::parse_comma_separated(
            input,
            |ts| {
                let meta_simple = syn_generic::parse_meta_simple(ts)?;

                if let Some(ident_tree) = extend_one_attr_meta(meta_simple, errors) {
                    ident_trees.push(ident_tree);
                }

                Ok(())
            },
            (),
        )?;
        input.expect_eof()
    })();

    match res {
        Ok(()) => {}
        Err(e) => errors.push(e),
    }

    ident_trees
}

impl ItemAttrsParser {
    fn extend_one_attr_meta(
        &mut self,
        meta: MetaSimple<ConsumedTokens<'_>>,
        errors: &mut ErrorCollector,
    ) -> Option<IdentTree> {
        enum Config {
            Crate,
            Where,
        }

        let config_mod_name;

        let config = ident_match!(match (meta.path) {
            b"crate" => {
                config_mod_name = "crate_";
                Config::Crate
            }
            b"crate_" => {
                config_mod_name = "";
                Config::Crate
            }
            b"where" => {
                config_mod_name = "r#where";
                Config::Where
            }
            b"where_" => {
                config_mod_name = "";
                Config::Where
            }
            _ => return self.item_attrs.push_meta_simple(meta, errors),
        });

        let MetaSimple { path, after_path } = meta;

        let config_children;

        let res = match config {
            Config::Crate => 'v: {
                config_children = vec![];

                if self.crate_path.is_some() {
                    break 'v Err(syn_generic::ParseError::custom(
                        "duplicated attribute `crate`",
                        path.span(),
                    ));
                }

                let value = match after_path {
                    syn_generic::MetaAfterPath::Empty => {
                        break 'v Err(syn_generic::ParseError::custom(
                            "expect `crate(::path::to::crate_cjson)`",
                            path.span(),
                        ));
                    }
                    syn_generic::MetaAfterPath::Group(group) => group.stream(),
                    syn_generic::MetaAfterPath::Eq {
                        eq,
                        before_comma_or_eof: _,
                    } => {
                        break 'v Err(syn_generic::ParseError::custom(
                            "expect `crate(::path::to::crate_cjson)`",
                            eq.span(),
                        ));
                    }
                };

                self.crate_path = Some(value);

                Ok(())
            }
            Config::Where => 'v: {
                config_children = vec![];

                if self.r#where.is_some() {
                    break 'v Err(syn_generic::ParseError::custom(
                        "duplicated attribute `where`",
                        path.span(),
                    ));
                }

                let value = match after_path {
                    syn_generic::MetaAfterPath::Empty => Err(path.span()),
                    syn_generic::MetaAfterPath::Group(group) => Err(group.span_open()),
                    syn_generic::MetaAfterPath::Eq {
                        eq,
                        before_comma_or_eof,
                    } => {
                        let mut before_comma_or_eof = before_comma_or_eof.parse();
                        match before_comma_or_eof.next() {
                            Some(TokenTree::Group(g)) => {
                                let value = g.stream();
                                if let Err(e) = before_comma_or_eof.expect_eof() {
                                    errors.push(e);
                                }
                                Ok(value)
                            }
                            tt => Err(tt.map_or_else(|| eq.span(), |tt| tt.span_open_or_entire())),
                        }
                    }
                };

                let value = match value {
                    Ok(value) => value,
                    Err(span) => {
                        break 'v Err(syn_generic::ParseError::custom(
                            "expect `where = (Bounds:)`",
                            span,
                        ));
                    }
                };

                self.r#where = Some(ItemAttrWhere {
                    where_span: path.span(),
                    bound: value,
                });

                Ok(())
            }
        };

        match res {
            Ok(()) => {}
            Err(error) => errors.push(error),
        }

        Some(IdentTree {
            ident: path,
            mod_name: config_mod_name,
            children: config_children,
        })
    }

    pub fn push_top_level_attr_meta(
        &mut self,
        v: TokenStream,
        errors: &mut ErrorCollector,
    ) -> Option<IdentTree> {
        push_top_level_attr_meta_with_one(
            v,
            |meta, errors| self.extend_one_attr_meta(meta, errors),
            errors,
        )
    }
}

#[proc_macro_derive(ToJson, attributes(cjson))]
pub fn derive_to_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let lit = proc_macro::Literal::string(&input.to_string());
    // let lit_debug = proc_macro::Literal::string(&format!("{input:?}"));

    let mut errors = ErrorCollector::default();
    let mut input = input.into();

    let mut item_attrs = ItemAttrsParser::default();

    let mut config_ident_trees: Vec<IdentTree> = vec![];

    let ParseItemStart {
        vis: _,
        first_ident,
    } = match syn_generic::parse_item_start(&mut input, |_, tt| {
        match syn_generic::GroupBracket::parse_from_token_tree(tt) {
            Ok(group) => {
                if let Some(config_ident_tree) =
                    item_attrs.push_top_level_attr_meta(group.stream(), &mut errors)
                {
                    config_ident_trees.push(config_ident_tree);
                }
            }
            Err(e) => errors.push(e),
        }
    }) {
        Ok(v) => v,
        Err(e) => {
            return e
                .join(errors)
                .into_item(None::<typed_quote::Never>, Span::call_site())
                .into_token_stream();
        }
    };

    let ItemAttrsParser {
        crate_path,
        r#where,
        item_attrs,
    } = item_attrs;

    let ref crate_path = match crate_path {
        Some(v) => v,
        None::<_> => quote! { ::cjson }.into_token_stream(),
    };

    let default_span = first_ident.span();

    let mut item_ident_tree = {
        let root_mod_name = ident_match!(match first_ident {
            b"struct" => "r#struct",
            b"enum" => "r#enum",
            _ => "common",
        });

        let root_mod_name = make_ident(root_mod_name, first_ident.span());

        IdentTree {
            ident: root_mod_name,
            mod_name: "",
            children: config_ident_trees,
        }
    };

    let item = to_json::ToJson {
        input: &mut input,
        first_ident,
        append_where_clause: r#where.map(|v| (v.where_span, v.bound)),
        item_attrs,
    }
    .try_parse(
        &mut errors,
        crate_path.clone(),
        &mut item_ident_tree.children,
    );
    let item = match item {
        Ok(item) => Some(item),
        Err(error) => {
            errors.push(error);
            None
        }
    };

    let use_item_attrs = item_ident_tree.into_tokens();

    let ts = item.map(|item| item.into_tokens(crate_path));

    let errors = errors
        .ok()
        .err()
        .map(|v| v.into_item(Some(crate_path), default_span));

    typed_quote!(
        #ts

        const _: () = {
            #[allow(unused_imports)]
            use #crate_path ::proc_macro::attrs::{
                #use_item_attrs
            };
        };

        #errors
    )
    .into_token_stream()
}

#[proc_macro]
pub fn unnamed_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = input.into_iter();

    let macro_bang = match input.next() {
        Some(TokenTree::Group(group)) => group.stream(),
        _ => panic!(),
    };
    let macro_prepend = match input.next() {
        Some(TokenTree::Group(group)) => group.stream(),
        _ => panic!(),
    };

    let fields = IterTokens(
        input
            .enumerate()
            .map(|(i, _)| proc_macro::Literal::usize_unsuffixed(i)),
    );

    quote!(
        #macro_bang {
            #macro_prepend
            #fields
        }
    )
    .into_token_stream()
}

/// ```txt
/// [ $($cjson_crate_path:tt)* ]
/// $(
///     ($_:json_value_expr)
/// )*
/// ```
///
/// $_:json_value_expr
///
/// ```txt
/// const $const_expr:block
///
/// runtime($runtime_expr:expr)
/// ```
#[proc_macro]
pub fn impl_json_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = input.into_iter();

    let cjson_crate_path = input
        .next()
        .and_then(|tt| match tt {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
                Some(group.stream())
            }
            _ => None,
        })
        .expect("[ $($cjson_crate_path:tt)* ]");

    let mut some_is_runtime = false;

    while let Some(paren_json_value_expr) = input.next() {
        let json_value_expr =
            parse_paren_json_value_expr(paren_json_value_expr).expect("($_:json_value_expr)");

        if json_value_expr.is_runtime() {
            some_is_runtime = true;
        }
    }

    proc_macro::TokenStream::from_iter(cjson_crate_path);
    if some_is_runtime {}

    todo!()
}

fn parse_paren_json_value_expr(tt: TokenTree) -> Result<JsonValueExpr, ParseError> {
    let json_value_expr = match tt {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => group.stream(),
        _ => return Err(ParseError),
    };
    JsonValueExpr::parse(json_value_expr)
}

struct Spanned<T>(Span, T);

enum JsonValueExprPrefix {
    Const,
    Runtime,
}

impl JsonValueExprPrefix {
    fn parse_ident(ident: &Ident) -> Option<Spanned<Self>> {
        let span = ident.span();
        let prefix = match ident.to_string().as_str() {
            "const" => Self::Const,
            "runtime" => Self::Runtime,
            _ => return None,
        };
        Some(Spanned(span, prefix))
    }
}

enum JsonValueExpr {
    Const {
        prefix_span: Span,
        block: TokenTree,
    },
    Runtime {
        prefix_span: Span,
        paren_expr: TokenTree,
    },
}

impl JsonValueExpr {
    fn parse(ts: TokenStream) -> Result<Self, ParseError> {
        let mut json_value_expr = ts.into_iter().fuse();

        match (
            json_value_expr.next(),
            json_value_expr.next(),
            json_value_expr.next(),
        ) {
            (Some(TokenTree::Ident(ident)), Some(tt), None) => {
                JsonValueExprPrefix::parse_ident(&ident).map(|Spanned(prefix_span, prefix)| {
                    match prefix {
                        JsonValueExprPrefix::Const => JsonValueExpr::Const {
                            prefix_span,
                            block: tt,
                        },
                        JsonValueExprPrefix::Runtime => JsonValueExpr::Runtime {
                            prefix_span,
                            paren_expr: tt,
                        },
                    }
                })
            }
            _ => None,
        }
        .ok_or(ParseError)
    }

    /// Returns `true` if the json value expr is [`Runtime`].
    ///
    /// [`Runtime`]: JsonValueExpr::Runtime
    #[must_use]
    const fn is_runtime(&self) -> bool {
        matches!(self, Self::Runtime { .. })
    }
}

#[derive(Debug)]
struct ParseError;
