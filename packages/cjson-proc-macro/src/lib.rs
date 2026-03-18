use std::convert::identity;

use proc_macro::{Delimiter, Group, Ident, Punct, Span, TokenStream, TokenTree};
use typed_quote::prelude::*;

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
    crate_path: Option<Config<TokenStream>>,

    cjson_spans: Vec<Span>,
    errors: &'a mut ErrorCollector,
}

struct Config<V> {
    name: Ident,
    value: V,
}

impl<P> syn_generic::CollectSeparated<MetaSimple, P> for &mut ItemAttrsParser<'_> {
    fn push_pair(&mut self, item: MetaSimple, _: P) {
        self.extend_one_attr_meta(item)
    }

    type Collect = ();

    fn collect_with_last(self, last: MetaSimple) -> Self::Collect {
        self.extend_one_attr_meta(last);
    }

    fn collect(self) -> Self::Collect {}
}

impl<'a> ItemAttrsParser<'a> {
    pub fn new(errors: &'a mut ErrorCollector) -> Self {
        Self {
            crate_path: Default::default(),
            cjson_spans: Default::default(),
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

    fn extend_one_attr_meta(&mut self, v: MetaSimple) {
        self.try_with(|this| this.try_extend_one_attr_meta(v))
            .unwrap_or_else(identity)
    }

    fn try_extend_one_attr_meta(
        &mut self,
        MetaSimple { path, after_path }: MetaSimple,
    ) -> Result<(), syn_generic::ParseError> {
        ident_match!(match path {
            b"crate" => {
                let value = match after_path {
                    syn_generic::MetaAfterPath::Empty => {
                        return Err(syn_generic::ParseError::custom(
                            "expect `crate_path(::path::to::crate_cjson)`",
                            path.span(),
                        ));
                    }
                    syn_generic::MetaAfterPath::Group(group) => {
                        if self.crate_path.is_some() {
                            return Err(syn_generic::ParseError::custom(
                                "expect `crate_path(::path::to::crate_cjson)`",
                                path.span(),
                            ));
                        }
                        group.stream()
                    }
                    syn_generic::MetaAfterPath::Eq {
                        eq,
                        before_comma_or_eof: _,
                    } => {
                        return Err(syn_generic::ParseError::custom(
                            "expect `crate_path(::path::to::crate_cjson)`",
                            eq.span(),
                        ));
                    }
                };

                self.crate_path = Some(Config { name: path, value })
            }
            _ =>
                return Err(syn_generic::ParseError::custom(
                    "unknown attribute",
                    path.span(),
                )),
        });

        Ok(())
    }

    fn extend_attr_meta(&mut self, attr_meta: TokenStream) {
        let ref mut input: syn_generic::ParsingTokenStream = attr_meta.into();

        self.try_with(|this| {
            syn_generic::parse_comma_separated(input, syn_generic::parse_meta_simple, this)?;
            input.expect_eof()
        })
        .unwrap_or_else(identity);
    }

    pub fn push_top_level_attr_meta(&mut self, v: TokenStream) {
        self.try_with(|this| this.try_push_top_level_attr_meta(v))
            .unwrap_or_else(identity)
    }

    fn try_push_top_level_attr_meta(
        &mut self,
        attr_meta: TokenStream,
    ) -> Result<(), syn_generic::ParseError> {
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

        self.cjson_spans.push(cjson.span());

        let after_path = syn_generic::parse_meta_after_path(input);

        match after_path {
            syn_generic::MetaAfterPath::Empty => {}
            syn_generic::MetaAfterPath::Group(group) => self.extend_attr_meta(group.stream()),
            syn_generic::MetaAfterPath::Eq {
                eq,
                before_comma_or_eof: _,
            } => {
                self.errors()
                    .push(syn_generic::ParseError::custom("expect `()`", eq.span()));
            }
        }

        input.expect_eof()
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

    let ParseItemStart {
        vis: _,
        first_ident,
    } = match syn_generic::parse_item_start(&mut input, |_, attr_body| match attr_body {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            item_attrs.push_top_level_attr_meta(group.stream())
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

    let crate_path = match item_attrs.crate_path.as_ref().map(|v| &v.value) {
        None::<_> => typed_quote::Either::B(quote! { ::cjson }),
        Some(v) => typed_quote::Either::A(v),
    };

    let default_span = first_ident.span();

    let item = match (to_json::ToJson {
        input: &mut input,
        first_ident,
    }
    .try_parse(&mut errors))
    {
        Ok(item) => item,
        Err(error) => {
            return error
                .join(errors)
                .into_item(Some(crate_path), default_span)
                .into_token_stream();
        }
    };

    let ts = item.into_tokens(crate_path);

    let errors = errors
        .ok()
        .err()
        .map(|v| v.into_item(Some(crate_path), default_span));

    typed_quote!(
        #ts
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

    let fields = typed_quote::tokens::IterTokens(
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
