use std::{
    borrow::{Borrow, Cow},
    iter,
};

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use typed_quote::{Either, IntoTokens, ToTokens, WithSpan, quote};

use crate::syn_generic;

use self::ident_eq::ident_matches;

pub(crate) mod ident_eq;

mod token_tree_ext {
    pub trait Sealed {}
}

pub trait TokenTreeExt: token_tree_ext::Sealed + Borrow<TokenTree> {
    fn span_open_or_entire(&self) -> Span {
        match self.borrow() {
            TokenTree::Group(group) => group.span_open(),
            TokenTree::Ident(tt) => tt.span(),
            TokenTree::Punct(tt) => tt.span(),
            TokenTree::Literal(tt) => tt.span(),
        }
    }

    fn span_close_or_entire(&self) -> Span {
        match self.borrow() {
            TokenTree::Group(group) => group.span_close(),
            TokenTree::Ident(tt) => tt.span(),
            TokenTree::Punct(tt) => tt.span(),
            TokenTree::Literal(tt) => tt.span(),
        }
    }
}

impl token_tree_ext::Sealed for TokenTree {}
impl TokenTreeExt for TokenTree {}

macro_rules! next_if {
    (
        match $input:ident {
            $($pats:tt)*
        }
    ) => {
        __next_if_parse_pats! {
            (
                input($input)
            )
            []
            {
                $($pats)*
            }
        }
    };
}

macro_rules! __next_if_expand_value {
    (skip $skip:ident ($input:expr) [$pat:pat] ($e:expr)) => {
        $skip!($e)
    };
    (next $next:ident ($input:expr) [$pat:pat] ($e:expr)) => {
        match $input.$next() {
            $pat => $e,
            _ => unreachable!(),
        }
    };
}

macro_rules! skip {
    ($e:expr) => {
        $e
    };
}

macro_rules! __next_if_parse_pats {
    (
        $data:tt
        [$($parsed:tt)*]
        {
            #[$skip_or_next:ident]
            $p:pat $(if $condition:expr)? => $e:expr
            $(, $($rest:tt)*)?
        }
    ) => {
        __next_if_parse_pats! {
            $data
            [
                $($parsed)*
                {
                    $skip_or_next
                    [$p][$(if $condition)?][$e]
                }
            ]{
                $($($rest)*)?
            }
        }
    };
    (
        $data:tt
        [$($parsed:tt)*]
        {
            #[$skip_or_next:ident]
            $p:pat $(if $condition:expr)? => $e:block
            $($rest:tt)*
        }
    ) => {
        __next_if_parse_pats! {
            $data
            [
                $($parsed)*
                {
                    $skip_or_next
                    [$p][$(if $condition)?][$e]
                }
            ]{
                $($rest)*
            }
        }
    };
    (
        (
            input($input:expr)
        )
        [$({
            $kind:ident
            [$pat:pat][$($if_condition:tt)*][$e:expr]
        })*]
        {}
    ) => {
        match $input.first() {
            $(
                $pat $($if_condition)* => {
                    __next_if_expand_value! {
                        $kind
                        $kind
                        ($input)
                        [$pat]
                        ($e)
                    }
                }
            )*
        }
    };
}
//

pub struct ParsingTokenStream {
    s: std::vec::IntoIter<TokenTree>,
    prev_span: Option<Span>,
}

impl From<TokenStream> for ParsingTokenStream {
    fn from(value: TokenStream) -> Self {
        Self {
            s: value.into_iter().collect::<Vec<_>>().into_iter(),
            prev_span: None,
        }
    }
}

impl ParsingTokenStream {
    pub fn first(&self) -> Option<&TokenTree> {
        self.s.as_slice().first()
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        let tt = self.s.next();

        if let Some(tt) = &tt {
            self.prev_span = Some(tt.span_close_or_entire());
        }

        tt
    }

    pub fn next_or_error(&mut self) -> Result<TokenTree, ParseError> {
        let tt = self.next();

        match tt {
            Some(tt) => Ok(tt),
            None => Err(ParseError(
                ParseErrorKind::UnexpectedEof {
                    after: self.prev_span,
                },
                vec![],
            )),
        }
    }

    pub fn parse_ident(&mut self) -> Result<Ident, ErrorUnexpectedTokenOrEof> {
        match self.next() {
            Some(TokenTree::Ident(ident)) => Ok(ident),
            unexpected_tt => Err(match unexpected_tt {
                Some(unexpected_tt) => ErrorUnexpectedTokenOrEof::Token(unexpected_tt),
                None => ErrorUnexpectedTokenOrEof::Eof {
                    after: self.prev_span,
                },
            }),
        }
    }

    fn make_expect_err(
        &self,
        unexpected_tt: Option<&TokenTree>,
        expect_msg: impl Into<Cow<'static, str>>,
    ) -> ParseError {
        match unexpected_tt {
            Some(unexpected_tt) => ParseError::custom(expect_msg, unexpected_tt.span()),
            None => ParseErrorKind::UnexpectedEof {
                after: self.prev_span,
            }
            .into(),
        }
    }

    pub fn expect_eof(&self) -> Result<(), ParseError> {
        if self.first().is_none() {
            Ok(())
        } else {
            Err(ParseError::custom("expect eof", self.prev_span))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.s.len() == 0
    }

    fn consume_before_comma_or_eof(&mut self) -> Self {
        let pos = self
            .s
            .as_slice()
            .iter()
            .position(|tt| matches!(tt, TokenTree::Punct(p) if *p == ','));

        if let Some(pos) = pos {
            if pos == 0 {
                Self {
                    s: Default::default(),
                    prev_span: self.prev_span,
                }
            } else {
                self.prev_span = Some(self.s.as_slice()[pos - 1].span_close_or_entire());

                Self {
                    s: self.s.by_ref().take(pos).collect::<Vec<_>>().into_iter(),
                    prev_span: self.prev_span,
                }
            }
        } else {
            if let Some(span) = self.s.as_slice().last().map(|v| v.span_close_or_entire()) {
                self.prev_span = Some(span);
            }

            let s = core::mem::take(&mut self.s);

            Self {
                s,
                prev_span: self.prev_span,
            }
        }
    }

    pub fn parse_generics(&mut self) -> Result<ParseGenericsOutput, ParseError> {
        let mut out = ParseGenericsOutput::default();
        let PunctLt(_lt) = next_if!(match self {
            #[next]
            Some(TokenTree::Punct(p)) if *p == '<' => PunctLt(p),
            #[skip]
            _ => return Ok(out),
        });

        enum GenericParamName {
            Lifetime { single_quote: Punct, name: Ident },
            Type { name: Ident },
            Const { r#const: Ident, name: Ident },
        }

        let PunctGt(_gt) = loop {
            let name = match self.next() {
                Some(TokenTree::Punct(p)) if p == '>' => break PunctGt(p),
                Some(TokenTree::Punct(p)) if p.spacing() == Spacing::Joint && p == '\'' => {
                    GenericParamName::Lifetime {
                        single_quote: p,
                        name: self.parse_ident()?,
                    }
                }
                Some(TokenTree::Ident(id)) => {
                    if ident_matches!(id, b"const") {
                        GenericParamName::Const {
                            r#const: id,
                            name: self.parse_ident()?,
                        }
                    } else {
                        GenericParamName::Type { name: id }
                    }
                }
                tt => {
                    return Err(match tt {
                        Some(tt) => ErrorUnexpectedTokenOrEof::Token(tt),
                        None => ErrorUnexpectedTokenOrEof::Eof {
                            after: self.prev_span,
                        },
                    }
                    .into());
                }
            };

            match name {
                GenericParamName::Lifetime { single_quote, name } => {
                    let lt = [TokenTree::from(single_quote), name.into()];
                    out.impl_generics.extend(lt.clone());
                    out.ty_generics.extend(lt);
                }
                GenericParamName::Type { name } => {
                    out.impl_generics.extend(Some(name.clone()));
                    out.ty_generics.extend(Some(name));
                }
                GenericParamName::Const { r#const, name } => {
                    out.impl_generics.extend([r#const, name.clone()]);
                    out.ty_generics.extend(Some(name));
                }
            }

            'bounds: {
                let colon = next_if!(match self {
                    #[next]
                    Some(TokenTree::Punct(p)) if *p == ':' => PunctColon(p),
                    #[skip]
                    _ => break 'bounds,
                });

                () = self.parse_in_generics_before_punct(
                    |ch| matches!(ch, '=' | ','),
                    |ts| {
                        if ts.len() > 0 {
                            out.impl_generics
                                .extend(iter::once(TokenTree::from(colon.0)).chain(ts.to_vec()));
                        }
                    },
                );
            }

            () = self.parse_in_generics_before_punct(
                |ch| matches!(ch, ','),
                |_eq_default| {
                    //
                },
            );

            let comma = next_if!(match self {
                #[next]
                Some(TokenTree::Punct(p)) if *p == ',' => {
                    PunctComma(p)
                }
                #[skip]
                _ => {
                    PunctComma(Punct::new(',', Spacing::Alone))
                }
            });

            out.impl_generics.extend(Some(comma.0.clone()));
            out.ty_generics.extend(Some(comma.0));
        };

        Ok(out)
    }

    pub fn parse_struct_after_generics(
        &mut self,
    ) -> Result<(Option<WhereClause>, StructData), ParseError> {
        let where_clause = self.parse_where_clause()?;

        enum Out {
            Paren(GroupParen),
            Brace(GroupBrace),
            Semi(PunctSemi),
        }

        let out = next_if!(match self {
            #[next]
            Some(TokenTree::Group(g))
                if g.delimiter() == Delimiter::Parenthesis && where_clause.is_none() =>
            {
                Out::Paren(GroupParen(g))
            }
            #[next]
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                Out::Brace(GroupBrace(g))
            }
            #[next]
            Some(TokenTree::Punct(p)) if *p == ';' => {
                Out::Semi(PunctSemi(p))
            }
            #[skip]
            tt => return Err(self.make_expect_err(tt, "unexpected token")),
        });

        match out {
            Out::Paren(paren) => {
                let where_clause = self.parse_where_clause()?;

                let semi = next_if!(match self {
                    #[next]
                    Some(TokenTree::Punct(p)) if *p == ';' => PunctSemi(p),
                    #[skip]
                    tt => return Err(self.make_expect_err(tt, "expect `;`")),
                });

                Ok((where_clause, StructData::Paren { paren, semi }))
            }
            Out::Brace(g) => Ok((where_clause, StructData::Brace(g))),
            Out::Semi(semi) => Ok((where_clause, StructData::Semi(semi))),
        }
    }

    pub fn parse_where_clause(&mut self) -> Result<Option<WhereClause>, ParseError> {
        let r#where = next_if!(match self {
            #[next]
            Some(TokenTree::Ident(id)) if ident_matches!(id, b"where") => {
                IdentWhere(id)
            }
            #[skip]
            _ => return Ok(None),
        });

        let mut nested = 0usize;

        enum Error {
            UnexpectedGt,
            UnexpectedSemiInNested,
        }

        let predicates = self.consume_before_or_all(
            |tt, error: &mut _| {
                match tt {
                    TokenTree::Group(group)
                        if nested == 0 && group.delimiter() == Delimiter::Brace =>
                    {
                        return true;
                    }
                    TokenTree::Punct(p) => match p.as_char() {
                        '<' => nested += 1,
                        '>' => {
                            if nested == 0 {
                                *error = Some((Error::UnexpectedGt, p.span()));
                                return true;
                            } else {
                                nested -= 1;
                            }
                        }
                        ';' => {
                            if nested > 0 {
                                *error = Some((Error::UnexpectedSemiInNested, p.span()));
                            }
                            return true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
                false
            },
            |ts, error| {
                if let Some(error) = error {
                    return Err(error);
                }

                let mut ts = ts.to_vec();

                if ts
                    .last()
                    .is_some_and(|tt| !matches!(tt, TokenTree::Punct(p) if *p == ','))
                {
                    ts.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                }

                Ok(ts)
            },
            None::<(Error, Span)>,
        );

        let predicates = match predicates {
            Ok(predicates) => predicates,
            Err((error, span)) => {
                return Err(ParseError::custom(
                    match error {
                        Error::UnexpectedGt => "unexpected `>`",
                        Error::UnexpectedSemiInNested => "unexpected `;` in nested type path",
                    },
                    span,
                ));
            }
        };

        Ok(Some(WhereClause {
            r#where,
            predicates,
        }))
    }

    fn parse_in_generics_before_punct<T>(
        &mut self,
        mut f: impl FnMut(char) -> bool,
        output: impl FnOnce(&[TokenTree]) -> T,
    ) -> T {
        let mut nested = 0usize;
        let check = |tt: &_, (): &mut ()| {
            if let TokenTree::Punct(p) = tt {
                match p.as_char() {
                    '<' => nested += 1,
                    '>' => {
                        if nested == 0 {
                            return true;
                        } else {
                            nested -= 1;
                        }
                    }
                    p if nested == 0 && f(p) => return true,
                    _ => {}
                }
            }

            false
        };

        self.consume_before_or_all(check, |ts, ()| output(ts), ())
    }

    fn consume_before_or_all<T, R>(
        &mut self,
        mut f: impl FnMut(&TokenTree, &mut R) -> bool,
        output: impl FnOnce(&[TokenTree], R) -> T,
        mut resource: R,
    ) -> T {
        let pos = self.s.as_slice().iter().position(|tt| f(tt, &mut resource));

        match pos {
            Some(pos) => {
                if pos == 0 {
                    output(&[], resource)
                } else {
                    let out = output(&self.s.as_slice()[..pos], resource);

                    let prev_span = self.s.as_slice()[pos - 1].span_close_or_entire();

                    self.prev_span = Some(prev_span);

                    () = self.s.by_ref().take(pos).for_each(drop);

                    out
                }
            }
            None => {
                let out = output(self.s.as_slice(), resource);

                if let Some(prev_span) = self
                    .s
                    .as_slice()
                    .last()
                    .map(TokenTreeExt::span_close_or_entire)
                {
                    self.prev_span = Some(prev_span);
                }

                self.s = Default::default();

                out
            }
        }
    }
}

/// Each field is without `<>` but with a trailing comma if not empty
#[derive(Default)]
pub struct ParseGenericsOutput {
    pub impl_generics: TokenStream,
    pub ty_generics: TokenStream,
}
// pub where_clause: TokenStream,// TODO:

pub enum ErrorUnexpectedTokenOrEof {
    Token(TokenTree),
    Eof { after: Option<Span> },
}

pub fn parse_item_start(
    input: &mut ParsingTokenStream,
    mut parse_attr: impl FnMut(PunctPound, TokenTree),
) -> Result<ParseItemStart, ParseError> {
    loop {
        let pound = next_if!(match input {
            #[next]
            Some(TokenTree::Punct(punct)) if *punct == '#' => PunctPound(punct),
            #[skip]
            _ => break,
        });

        let bracket_meta = input.next_or_error()?;

        parse_attr(pound, bracket_meta);
    }

    let vis = 'vis: {
        let ident_pub = next_if!(match input {
            #[next]
            Some(TokenTree::Ident(ident)) if ident_matches!(ident, b"pub") => IdentPub(ident),
            #[skip]
            _ => break 'vis None,
        });

        let paren = match input.first() {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                let g = match input.next() {
                    Some(TokenTree::Group(g)) => g,
                    _ => unreachable!(),
                };
                Some(g)
            }
            _ => None,
        };

        Some(SomeVisibility {
            r#pub: ident_pub,
            paren,
        })
    };

    let first_ident = input.parse_ident()?;

    Ok(ParseItemStart { vis, first_ident })
}

pub struct ParseItemStart {
    pub vis: Option<SomeVisibility>,
    pub first_ident: Ident,
}

pub fn parse_meta_simple(input: &mut ParsingTokenStream) -> Result<MetaSimple, ParseError> {
    let path = input.parse_ident()?;

    let after_path = parse_meta_after_path(input);

    Ok(MetaSimple { path, after_path })
}

pub struct MetaSimple {
    pub path: Ident,
    pub after_path: MetaAfterPath,
}

pub fn parse_meta_after_path(input: &mut ParsingTokenStream) -> MetaAfterPath {
    next_if!(match input {
        #[next]
        Some(TokenTree::Group(g)) => MetaAfterPath::Group(g),
        #[next]
        Some(TokenTree::Punct(p)) if *p == '=' => {
            MetaAfterPath::Eq {
                eq: PunctEq(p),
                before_comma_or_eof: input.consume_before_comma_or_eof(),
            }
        }
        #[skip]
        _ => MetaAfterPath::Empty,
    })
}

pub enum MetaAfterPath {
    Empty,
    Group(Group),
    Eq {
        eq: PunctEq,
        before_comma_or_eof: ParsingTokenStream,
    },
}

pub trait CollectSeparated<T, P> {
    fn push_pair(&mut self, item: T, punct: P);

    type Collect;

    fn collect_with_last(self, last: T) -> Self::Collect;
    fn collect(self) -> Self::Collect;
}

pub fn parse_comma_separated<T, R>(
    input: &mut ParsingTokenStream,
    mut p: impl FnMut(&mut ParsingTokenStream) -> Result<T, ParseError>,
    mut co: impl CollectSeparated<T, PunctComma, Collect = R>,
) -> Result<R, ParseError> {
    let mut last = None;
    while !input.is_empty() {
        let item = p(input)?;
        next_if!(match input {
            #[next]
            Some(TokenTree::Punct(p)) if *p == ',' => {
                co.push_pair(item, PunctComma(p));
            }
            #[skip]
            _ => {
                last = Some(item);
                break;
            }
        });
    }

    Ok(if let Some(last) = last {
        co.collect_with_last(last)
    } else {
        co.collect()
    })
}

pub struct SomeVisibility {
    r#pub: IdentPub,
    paren: Option<Group>,
}

pub struct ParseError(ParseErrorKind, Vec<ParseErrorKind>);

impl ParseError {
    fn into_iter(self) -> impl Iterator<Item = ParseErrorKind> {
        iter::once(self.0).chain(self.1)
    }

    fn push(&mut self, e: ParseError) {
        self.1.extend(e.into_iter());
    }

    pub fn join(mut self, other: ErrorCollector) -> Self {
        if let Some(other) = other.0 {
            self.push(other);
        }

        self
    }

    pub fn into_item(
        self,
        crate_path: Option<impl IntoTokens>,
        default_span: Span,
    ) -> impl IntoTokens {
        let errors = typed_quote::tokens::IterTokens(
            self.into_iter().map(move |v| v.into_stmt(default_span)),
        );

        let path_prefix = match crate_path {
            Some(crate_path) => Either::A(quote!(#crate_path ::__private::proc_macro)),
            None => Either::B(quote!(::core)),
        };

        quote! {
            const _: () = {
                use #path_prefix::compile_error;
                #errors
            };
        }
    }
}

impl From<ErrorUnexpectedTokenOrEof> for ParseError {
    fn from(value: ErrorUnexpectedTokenOrEof) -> Self {
        Self(
            match value {
                ErrorUnexpectedTokenOrEof::Token(tt) => ParseErrorKind::UnexpectedToken {
                    span: tt.span_open_or_entire(),
                },
                ErrorUnexpectedTokenOrEof::Eof { after } => ParseErrorKind::UnexpectedEof { after },
            },
            vec![],
        )
    }
}

impl From<ParseErrorKind> for ParseError {
    fn from(value: ParseErrorKind) -> Self {
        Self(value, vec![])
    }
}

enum ParseErrorKind {
    UnexpectedToken {
        span: Span,
    },
    UnexpectedEof {
        after: Option<Span>,
    },
    Custom {
        msg: std::borrow::Cow<'static, str>,
        span: Option<Span>,
    },
}

impl ParseErrorKind {
    fn message(&self) -> (&str, Option<Span>) {
        match *self {
            ParseErrorKind::UnexpectedToken { span } => ("unexpected token", Some(span)),
            ParseErrorKind::UnexpectedEof { after } => ("unexpected eof", after),
            ParseErrorKind::Custom { ref msg, span } => (msg, span),
        }
    }

    fn into_stmt(self, default_span: Span) -> impl IntoTokens {
        let (msg, span) = self.message();
        let msg = Literal::string(msg);
        quote!(compile_error! { #msg }).with_replaced_span(span.unwrap_or(default_span))
    }
}

impl ParseError {
    fn unexpected_eof_after(after: Span) -> Self {
        Self(ParseErrorKind::UnexpectedEof { after: Some(after) }, vec![])
    }

    pub fn custom(
        msg: impl Into<std::borrow::Cow<'static, str>>,
        span: impl Into<Option<Span>>,
    ) -> Self {
        Self(
            ParseErrorKind::Custom {
                msg: msg.into(),
                span: span.into(),
            },
            vec![],
        )
    }
}

pub struct GroupParen(Group);
pub struct GroupBrace(Group);

/// `#`
pub struct PunctPound(Punct);
/// `,`
pub struct PunctComma(Punct);
/// `=`
pub struct PunctEq(Punct);
/// `:`
pub struct PunctColon(Punct);
/// `;`
pub struct PunctSemi(Punct);
/// `<`
pub struct PunctLt(Punct);
/// `>`
pub struct PunctGt(Punct);

impl PunctEq {
    pub fn span(&self) -> Span {
        self.0.span()
    }
}

/// `pub`
pub struct IdentPub(Ident);

/// `where`
pub struct IdentWhere(Ident);

impl From<IdentWhere> for Ident {
    fn from(value: IdentWhere) -> Self {
        value.0
    }
}

#[derive(Default)]
pub struct ErrorCollector(Option<ParseError>);

impl ErrorCollector {
    pub fn push(&mut self, e: ParseError) {
        match &mut self.0 {
            Some(errors) => errors.push(e),
            None => self.0 = Some(e),
        }
    }

    pub fn ok(self) -> Result<(), ParseError> {
        match self.0 {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}

pub struct WhereClause {
    pub r#where: IdentWhere,
    pub predicates: Vec<TokenTree>,
}

pub enum StructData {
    Paren { paren: GroupParen, semi: PunctSemi },
    Brace(GroupBrace),
    Semi(PunctSemi),
}

impl StructData {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            StructData::Paren {
                paren: GroupParen(paren),
                semi: PunctSemi(semi),
            } => quote!(#paren #semi).into_token_stream(),
            StructData::Brace(group_brace) => group_brace.0.into_token_stream(),
            StructData::Semi(punct_semi) => punct_semi.0.into_token_stream(),
        }
    }
}
