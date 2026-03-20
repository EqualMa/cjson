use std::convert::identity;

use proc_macro::{Delimiter, Group, Ident, Punct, Span, TokenStream, TokenTree};
use typed_quote::{prelude::*, tokens::IterTokens};

use crate::{
    ident_eq::ident_matches,
    syn_generic::{
        ErrorCollector, MetaSimple, ParseItemStart, TokenTreeExt,
        ident_eq::{self, __ident_match_after_fat_arrow, __ident_match_parse_pats, ident_match},
    },
};

mod syn_generic;

mod to_json;

struct ItemAttrsParser<'a> {
    crate_path: Option<TokenStream>,
    r#where: Option<ItemAttrWhere>,

    errors: &'a mut ErrorCollector,
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

impl<P> syn_generic::CollectSeparated<MetaSimple, P>
    for (&mut ItemAttrsParser<'_>, &mut Vec<IdentTree>)
{
    fn push_pair(&mut self, item: MetaSimple, _: P) {
        <(&mut ItemAttrsParser<'_>, &mut Vec<IdentTree>) as syn_generic::CollectSeparated<
            MetaSimple,
            P,
        >>::collect_with_last((self.0, self.1), item);
    }

    type Collect = ();

    fn collect_with_last(self, item: MetaSimple) -> Self::Collect {
        if let Some(ident_tree) = self.0.extend_one_attr_meta(item) {
            self.1.push(ident_tree);
        }
    }

    fn collect(self) -> Self::Collect {}
}

impl<'a> ItemAttrsParser<'a> {
    pub fn new(errors: &'a mut ErrorCollector) -> Self {
        Self {
            crate_path: Default::default(),
            r#where: Default::default(),
            errors,
        }
    }

    fn try_with<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, syn_generic::ParseError>,
    ) -> Result<T, ()> {
        match f(self) {
            Ok(v) => Ok(v),
            Err(error) => {
                self.errors.push(error);
                Err(())
            }
        }
    }

    fn extend_one_attr_meta(&mut self, v: MetaSimple) -> Option<IdentTree> {
        self.try_with(|this| this.try_extend_one_attr_meta(v)).ok()
    }

    fn try_extend_one_attr_meta(
        &mut self,
        MetaSimple { path, after_path }: MetaSimple,
    ) -> Result<IdentTree, syn_generic::ParseError> {
        enum Config {
            Crate,
            Where,
        }

        let config_mod_name;

        let config = ident_match!(match path {
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
            _ =>
                return Err(syn_generic::ParseError::custom(
                    "unknown attribute",
                    path.span(),
                )),
        });

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
                        mut before_comma_or_eof,
                    } => match before_comma_or_eof.next() {
                        Some(TokenTree::Group(g)) => {
                            let value = g.stream();
                            if let Err(e) = before_comma_or_eof.expect_eof() {
                                self.errors().push(e);
                            }
                            Ok(value)
                        }
                        tt => Err(tt.map_or_else(|| eq.span(), |tt| tt.span_open_or_entire())),
                    },
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
            Err(error) => self.errors().push(error),
        }

        Ok(IdentTree {
            ident: path,
            mod_name: config_mod_name,
            children: config_children,
        })
    }

    fn extend_attr_meta(&mut self, attr_meta: TokenStream) -> Vec<IdentTree> {
        let ref mut input: syn_generic::ParsingTokenStream = attr_meta.into();

        let mut ident_trees = vec![];

        self.try_with(|this| {
            syn_generic::parse_comma_separated(
                input,
                syn_generic::parse_meta_simple,
                (this, &mut ident_trees),
            )?;
            input.expect_eof()
        })
        .unwrap_or_else(identity);

        ident_trees
    }

    pub fn push_top_level_attr_meta(&mut self, v: TokenStream) -> Option<IdentTree> {
        self.try_with(|this| this.try_push_top_level_attr_meta(v))
            .ok()
    }

    fn try_push_top_level_attr_meta(
        &mut self,
        attr_meta: TokenStream,
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

        let after_path = syn_generic::parse_meta_after_path(input);

        match after_path {
            syn_generic::MetaAfterPath::Empty => {}
            syn_generic::MetaAfterPath::Group(group) => {
                sub_ident_trees = self.extend_attr_meta(group.stream())
            }
            syn_generic::MetaAfterPath::Eq {
                eq,
                before_comma_or_eof: _,
            } => {
                self.errors()
                    .push(syn_generic::ParseError::custom("expect `()`", eq.span()));
            }
        }

        if let Err(err) = input.expect_eof() {
            self.errors().push(err)
        }

        Ok(IdentTree {
            ident: cjson,
            mod_name: "",
            children: sub_ident_trees,
        })
    }
    pub fn errors(&mut self) -> &mut ErrorCollector {
        self.errors
    }
}

#[proc_macro_derive(ToJson, attributes(cjson))]
pub fn derive_to_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let lit = proc_macro::Literal::string(&input.to_string());
    // let lit_debug = proc_macro::Literal::string(&format!("{input:?}"));

    let mut errors = ErrorCollector::default();
    let mut input = input.into();

    let mut item_attrs = ItemAttrsParser::new(&mut errors);

    let mut config_ident_trees: Vec<IdentTree> = vec![];

    let ParseItemStart {
        vis: _,
        first_ident,
    } = match syn_generic::parse_item_start(&mut input, |_, attr_body| match attr_body {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            if let Some(config_ident_tree) = item_attrs.push_top_level_attr_meta(group.stream()) {
                config_ident_trees.push(config_ident_tree);
            }
        }
        _ => item_attrs.errors().push(syn_generic::ParseError::custom(
            "expect `[`",
            attr_body.span(),
        )),
    }) {
        Ok(v) => v,
        Err(e) => {
            return e
                .join(errors)
                .into_item(None::<typed_quote::Never>, Span::call_site())
                .into_token_stream();
        }
    };

    let crate_path = match item_attrs.crate_path.as_ref() {
        None::<_> => typed_quote::Either::A(quote! { ::cjson }),
        Some(v) => typed_quote::Either::B(v),
    };

    let default_span = first_ident.span();

    let use_item_attrs = {
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
        .into_tokens()
    };

    let item = to_json::ToJson {
        input: &mut input,
        first_ident,
        append_where_clause: item_attrs.r#where.map(|v| (v.where_span, v.bound)),
    }
    .try_parse(&mut errors);
    let item = match item {
        Ok(item) => Some(item),
        Err(error) => {
            errors.push(error);
            None
        }
    };

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
