use std::{
    borrow::{Borrow, Cow},
    convert::Infallible,
    iter,
    mem::ManuallyDrop,
    ops::{self, Add, Deref},
    process::Output,
    vec,
};

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use typed_quote::{Either, IntoTokens, ToTokens, WithSpan, quote};

use crate::{ident_match, syn_generic};

use self::ident_eq::ident_matches;

pub(crate) mod ident_eq;

pub mod parse_meta;
pub mod parse_meta_utils;

pub trait Parseable {}

pub trait Parse<IN>: Sized + Parseable {
    type ErrorPayload: TryUnwrapPayload<Self>;
    type Error: Into<ParseError>;
    type ErrorWithPayload: Into<ErrorWithPayload<Self::Error, Self::ErrorPayload>>;
    fn parse(input: IN) -> Result<Self, Self::ErrorWithPayload>;

    fn parse_and_report(input: IN, errors: &mut ErrorCollector) -> Result<Self, Self::Error> {
        let err = match Self::parse(input) {
            Ok(v) => return Ok(v),
            Err(err) => err,
        };
        let ErrorWithPayload { error, payload } = err.into();

        if let Some(payload) = payload.try_into_payload() {
            errors.push(error.into());
            Ok(payload)
        } else {
            Err(error)
        }
    }
}

pub trait ResultExt: Into<Result<Self::Ok, Self::Err>> {
    type Ok;
    type Err;
}

pub trait ResultWithPayload:
    ResultExt<Err: Into<ErrorWithPayload<Self::Error, Self::ErrorPayload>>>
{
    type Error;
    type ErrorPayload;
}

pub struct ErrorWithPayload<E, P> {
    pub error: E,
    pub payload: P,
}

pub struct NoPayload;

impl From<Infallible> for NoPayload {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

pub trait TryUnwrapPayload<P: Parseable> {
    type HasPayload: Into<HasPayload<P>>;
    type NoPayload: Into<NoPayload>;
    fn try_unwrap_payload(self) -> Result<Self::HasPayload, Self::NoPayload>;
    fn try_into_payload(self) -> Option<P>;
}

pub struct HasPayload<T>(pub T);

impl<T> From<Infallible> for HasPayload<T> {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl<T: Parseable> TryUnwrapPayload<T> for NoPayload {
    type HasPayload = Infallible;
    type NoPayload = NoPayload;

    fn try_unwrap_payload(self) -> Result<Self::HasPayload, Self::NoPayload> {
        Err(self)
    }
    fn try_into_payload(self) -> Option<T> {
        None
    }
}

impl<T: Parseable> TryUnwrapPayload<T> for T {
    type HasPayload = HasPayload<T>;
    type NoPayload = Infallible;

    fn try_unwrap_payload(self) -> Result<Self::HasPayload, Self::NoPayload> {
        Ok(HasPayload(self))
    }
    fn try_into_payload(self) -> Option<T> {
        Some(self)
    }
}

impl<T: Parseable> TryUnwrapPayload<T> for Option<T> {
    type HasPayload = HasPayload<T>;
    type NoPayload = NoPayload;
    fn try_unwrap_payload(self) -> Result<Self::HasPayload, Self::NoPayload> {
        match self {
            Some(this) => Ok(HasPayload(this)),
            None => Err(NoPayload),
        }
    }
    fn try_into_payload(self) -> Option<T> {
        self
    }
}

impl<E> From<E> for ErrorWithPayload<E, NoPayload> {
    fn from(error: E) -> Self {
        Self {
            error,
            payload: NoPayload,
        }
    }
}

/// Expects eof.
impl<T: for<'a> Parse<&'a mut ParsingTokenStream>> Parse<ParsingTokenStream> for T {
    type ErrorPayload = Option<T>;
    type Error = ParseError;
    type ErrorWithPayload = ErrorWithPayload<Self::Error, Self::ErrorPayload>;

    fn parse(mut input: ParsingTokenStream) -> Result<Self, Self::ErrorWithPayload> {
        let payload = T::parse(&mut input).map_err(|error| {
            let ErrorWithPayload { error, payload } = error.into();
            ErrorWithPayload {
                error: error.into(),
                payload: payload.try_into_payload(),
            }
        })?;

        match input.expect_eof() {
            Ok(()) => Ok(payload),
            Err(error) => Err(ErrorWithPayload {
                error,
                payload: Some(payload),
            }),
        }
    }
}

/// Expects eof.
impl<T: for<'a> Parse<&'a mut ParsingTokenStream>> Parse<Vec<TokenTree>> for T {
    type ErrorPayload = <Self as Parse<ParsingTokenStream>>::ErrorPayload;
    type Error = <Self as Parse<ParsingTokenStream>>::Error;
    type ErrorWithPayload = <Self as Parse<ParsingTokenStream>>::ErrorWithPayload;

    fn parse(input: Vec<TokenTree>) -> Result<Self, Self::ErrorWithPayload> {
        <Self as Parse<ParsingTokenStream>>::parse(input.into())
    }
}

/// Expects eof.
impl<T: for<'a> Parse<&'a mut ParsingTokenStream>> Parse<TokenStream> for T {
    type ErrorPayload = <Self as Parse<ParsingTokenStream>>::ErrorPayload;
    type Error = <Self as Parse<ParsingTokenStream>>::Error;
    type ErrorWithPayload = <Self as Parse<ParsingTokenStream>>::ErrorWithPayload;

    fn parse(input: TokenStream) -> Result<Self, Self::ErrorWithPayload> {
        <Self as Parse<ParsingTokenStream>>::parse(input.into())
    }
}

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

#[derive(Debug)]
pub struct ParsingTokenStream<S = vec::IntoIter<TokenTree>> {
    s: S,
    prev_span: Option<Span>,
}

pub type ParsingTokenStreamCow<'a> = ParsingTokenStream<TokenStreamCow<'a>>;

pub enum TokenStreamCow<'a> {
    Borrowed(&'a [TokenTree]),
    Owned(vec::IntoIter<TokenTree>),
}

impl TokenStreamCow<'_> {
    fn into_vec(self) -> Vec<TokenTree> {
        match self {
            TokenStreamCow::Borrowed(ts) => ts.into(),
            TokenStreamCow::Owned(ts) => FromIterator::from_iter(ts),
        }
    }

    pub fn as_slice(&self) -> &[TokenTree] {
        match self {
            TokenStreamCow::Borrowed(ts) => *ts,
            TokenStreamCow::Owned(ts) => ts.as_slice(),
        }
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn into_vec_iter(self) -> vec::IntoIter<TokenTree> {
        match self {
            TokenStreamCow::Borrowed(ts) => ts.to_vec().into_iter(),
            TokenStreamCow::Owned(ts) => ts,
        }
    }
}

pub trait IterTokenTreeCow {}

impl From<TokenStream> for ParsingTokenStream {
    fn from(value: TokenStream) -> Self {
        value.into_iter().collect::<Vec<_>>().into()
    }
}

impl From<Vec<TokenTree>> for ParsingTokenStream {
    fn from(value: Vec<TokenTree>) -> Self {
        Self {
            s: value.into_iter(),
            prev_span: None,
        }
    }
}

pub fn with_trailing_punct_if_not_empty(mut ts: Vec<TokenTree>, punct: char) -> Vec<TokenTree> {
    if ts
        .last()
        .is_some_and(|tt| !matches!(tt, TokenTree::Punct(p) if *p == punct))
    {
        ts.push(TokenTree::Punct(Punct::new(punct, Spacing::Alone)));
    }

    ts
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

    fn len(&self) -> usize {
        self.s.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn take_all(&mut self) -> vec::IntoIter<TokenTree> {
        if let Some(prev_span) = self
            .s
            .as_slice()
            .last()
            .map(TokenTreeExt::span_close_or_entire)
        {
            self.prev_span = Some(prev_span);
        }

        let ts = std::mem::take(&mut self.s);

        ts
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
                match self.first() {
                    Some(TokenTree::Punct(p)) if *p == ':' => {}
                    _ => break 'bounds,
                }

                let bounds = self.parse_in_generics_before_punct(|ch| matches!(ch, '=' | ','));

                if bounds.len() > 0 {
                    out.impl_generics.extend(bounds.into_vec_iter());
                }
            }

            let eq_default = self.parse_in_generics_before_punct(|ch| matches!(ch, ','));
            drop(eq_default);

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
    ) -> Result<(Option<WhereClause<ConsumedTokens<'_>>>, StructData), ParseError> {
        let (mut r#where, predicates) = self.parse_where_clause_impl()?;

        enum Out {
            Paren(GroupParen),
            Brace(GroupBrace),
            Semi(PunctSemi),
        }

        let mut parsing_after_where_clause = ParsingAfterConsumedTokens(predicates);

        let out = next_if!(match parsing_after_where_clause {
            #[next]
            Some(TokenTree::Group(g))
                if g.delimiter() == Delimiter::Parenthesis && r#where.is_none() =>
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
            _ =>
                return Err(ParseError::custom(
                    "unexpected token",
                    parsing_after_where_clause.span_open_or_prev()
                )),
        });

        let data = match out {
            Out::Paren(paren) => {
                let this = parsing_after_where_clause
                    .0
                    .try_unwrap_rest()
                    .ok()
                    .expect("no where clause");

                let predicates;
                (r#where, predicates) = this.parse_where_clause_impl()?;
                parsing_after_where_clause = ParsingAfterConsumedTokens(predicates);

                let semi = next_if!(match parsing_after_where_clause {
                    #[next]
                    Some(TokenTree::Punct(p)) if *p == ';' => PunctSemi(p),
                    #[skip]
                    _ =>
                        return Err(ParseError::custom(
                            "expect `;`",
                            parsing_after_where_clause.span_open_or_prev()
                        )),
                });

                StructData::Paren { paren, semi }
            }
            Out::Brace(g) => StructData::Brace(g),
            Out::Semi(semi) => StructData::Semi(semi),
        };

        let where_clause = match r#where {
            Some(r#where) => Some(WhereClause {
                r#where,
                predicates: parsing_after_where_clause.0,
            }),
            None => None,
        };

        Ok((where_clause, data))
    }

    pub fn parse_enum_after_generics(
        &mut self,
    ) -> Result<(Option<WhereClause<ConsumedTokens<'_>>>, GroupBrace), ParseError> {
        let (r#where, predicates) = self.parse_where_clause_impl()?;

        let mut parsing_after_where_clause = ParsingAfterConsumedTokens(predicates);

        let out = next_if!(match parsing_after_where_clause {
            #[next]
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => GroupBrace(g),
            #[skip]
            _ =>
                return Err(ParseError::custom(
                    "expect `{...}`",
                    parsing_after_where_clause.span_open_or_prev()
                )),
        });

        let where_clause = match r#where {
            Some(r#where) => Some(WhereClause {
                r#where,
                predicates: parsing_after_where_clause.0,
            }),
            None => None,
        };

        Ok((where_clause, out))
    }

    pub fn parse_where_clause(
        &mut self,
    ) -> Result<Option<WhereClause<ConsumedTokens<'_>>>, ParseError> {
        let (r#where, predicates) = self.parse_where_clause_impl()?;
        Ok(match r#where {
            Some(r#where) => Some(WhereClause {
                r#where,
                predicates,
            }),
            None => None,
        })
    }

    fn parse_where_clause_impl(
        &mut self,
    ) -> Result<(Option<IdentWhere>, ConsumedTokens<'_>), ParseError> {
        let r#where = next_if!(match self {
            #[next]
            Some(TokenTree::Ident(id)) if ident_matches!(id, b"where") => {
                IdentWhere(id)
            }
            #[skip]
            _ => return Ok((None, ConsumedTokens::from(self))),
        });

        let mut nested = 0usize;

        enum Error {
            UnexpectedGt,
            UnexpectedSemiInNested,
        }

        let mut error = None::<(Error, Span)>;
        let predicates = self.consume_before_or_all(|tt| {
            match tt {
                TokenTree::Group(group) if nested == 0 && group.delimiter() == Delimiter::Brace => {
                    return true;
                }
                TokenTree::Punct(p) => match p.as_char() {
                    '<' => nested += 1,
                    '>' => {
                        if nested == 0 {
                            error = Some((Error::UnexpectedGt, p.span()));
                            return true;
                        } else {
                            nested -= 1;
                        }
                    }
                    ';' => {
                        if nested > 0 {
                            error = Some((Error::UnexpectedSemiInNested, p.span()));
                        }
                        return true;
                    }
                    _ => {}
                },
                _ => {}
            }
            false
        });

        let predicates = if let Some(error) = error {
            Err(error)
        } else {
            Ok(predicates)
        };

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

        Ok((Some(r#where), predicates))
    }

    fn parse_in_generics_before_punct(
        &mut self,
        mut f: impl FnMut(char) -> bool,
    ) -> ConsumedTokens<'_> {
        let mut nested = 0usize;
        let check = |tt: &_| {
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

        self.consume_before_or_all(check)
    }

    fn consume_before_or_all(
        &mut self,
        mut f: impl FnMut(&TokenTree) -> bool,
    ) -> ConsumedTokens<'_> {
        let pos = self.s.as_slice().iter().position(|tt| f(tt));

        ConsumedTokens {
            prev_span: self.prev_span,
            pos: match pos {
                Some(pos) => {
                    if pos > 0 {
                        let span = self.s.as_slice()[pos - 1].span_close_or_entire();
                        self.prev_span = Some(span);
                    }
                    ConsumedTokensPos::Partial {
                        consumed_len: pos,
                        rest_start_from: pos,
                    }
                }
                None => {
                    if let Some(last) = self
                        .s
                        .as_slice()
                        .last()
                        .map(TokenTreeExt::span_close_or_entire)
                    {
                        self.prev_span = Some(last);
                    }
                    ConsumedTokensPos::All
                }
            },
            ts: Some(self),
        }
    }

    pub fn parse_into_unnamed_fields<Attrs, R>(
        self,
        resource: &mut R,
        new_attrs: impl FnMut(&mut R) -> Attrs,
        push_outer_attr: impl FnMut(&mut Attrs, &mut R, PunctPound, TokenTree),
        mut push_field: impl FnMut(
            &mut R,
            Attrs,
            Option<SomeVisibility>,
            TokenStreamCow<'_>,
            Option<PunctComma>,
        ),
    ) -> Result<(), ParseError> {
        self.parse_into_field(
            resource,
            new_attrs,
            push_outer_attr,
            |_, _| Ok(()),
            |resource, attrs, vis, (), ty, comma| push_field(resource, attrs, vis, ty, comma),
        )
    }

    pub fn parse_into_named_fields<Attrs, R>(
        self,
        resource: &mut R,
        new_attrs: impl FnMut(&mut R) -> Attrs,
        push_outer_attr: impl FnMut(&mut Attrs, &mut R, PunctPound, TokenTree),
        mut push_field: impl FnMut(
            &mut R,
            Attrs,
            Option<SomeVisibility>,
            Ident,
            PunctColon,
            TokenStreamCow<'_>,
            Option<PunctComma>,
        ),
    ) -> Result<(), ParseError> {
        self.parse_into_field(
            resource,
            new_attrs,
            push_outer_attr,
            |input, _| {
                let name = input.parse_ident()?;

                let colon = match input.next() {
                    Some(TokenTree::Punct(p)) if p.spacing() == Spacing::Alone && p == ':' => {
                        PunctColon(p)
                    }
                    tt => {
                        return Err(ParseError::custom(
                            "expect `:`",
                            tt.map(|tt| tt.span_open_or_entire()).unwrap_or(name.span()),
                        ));
                    }
                };

                Ok((name, colon))
            },
            |resource, attrs, vis, (name, colon), ty, comma| {
                push_field(resource, attrs, vis, name, colon, ty, comma)
            },
        )
    }

    fn parse_into_field<Attrs, T, R>(
        self,
        mut resource: &mut R,
        mut new_attrs: impl FnMut(&mut R) -> Attrs,
        mut push_outer_attr: impl FnMut(&mut Attrs, &mut R, PunctPound, TokenTree),
        mut parse_after_vis: impl FnMut(&mut Self, &mut R) -> Result<T, ParseError>,
        mut push: impl FnMut(
            &mut R,
            Attrs,
            Option<SomeVisibility>,
            T,
            TokenStreamCow<'_>,
            Option<PunctComma>,
        ),
    ) -> Result<(), ParseError> {
        let mut input = self;

        while !input.is_empty() {
            let mut attrs = new_attrs(&mut resource);
            loop {
                let pound = next_if!(match input {
                    #[next]
                    Some(TokenTree::Punct(punct)) if *punct == '#' => PunctPound(punct),
                    #[skip]
                    _ => break,
                });

                let bracket_meta = input.next_or_error()?;

                push_outer_attr(&mut attrs, &mut resource, pound, bracket_meta);
            }

            let vis = input.parse_vis();

            let after_vis = parse_after_vis(&mut input, &mut resource)?;

            let ty = input.parse_in_generics_before_punct(|ch| matches!(ch, ','));

            let mut after_ty = ParsingAfterConsumedTokens(ty);
            let mut res = Ok(());
            let comma = next_if!(match after_ty {
                #[next]
                Some(TokenTree::Punct(punct)) if *punct == ',' => Some(PunctComma(punct)),
                #[skip]
                Some(tt) => {
                    res = Err(ParseError::custom(
                        "expect eof or `,`",
                        tt.span_open_or_entire(),
                    ));
                    None
                }
                #[next]
                None::<_> => None,
            });

            let ty = after_ty.0;

            () = ty.use_tokens(|ty| push(resource, attrs, vis, after_vis, ty, comma));

            res?;
        }

        input.expect_eof()
    }

    pub fn parse_into_variants<Attrs, R>(
        self,
        mut resource: &mut R,
        mut new_attrs: impl FnMut(&mut R) -> Attrs,
        mut push_outer_attr: impl FnMut(&mut Attrs, &mut R, PunctPound, TokenTree),
        mut push: impl FnMut(&mut R, Attrs, EnumVariant, Option<PunctComma>),
    ) -> Result<(), ParseError> {
        let mut input = self;

        while !input.is_empty() {
            let mut attrs = new_attrs(&mut resource);
            loop {
                let pound = next_if!(match input {
                    #[next]
                    Some(TokenTree::Punct(punct)) if *punct == '#' => PunctPound(punct),
                    #[skip]
                    _ => break,
                });

                let bracket_meta = input.next_or_error()?;

                push_outer_attr(&mut attrs, &mut resource, pound, bracket_meta);
            }

            let vis = input.parse_vis();

            let name = input.parse_ident()?;

            let body = next_if!(match input {
                #[next]
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                    EnumVariantBody::Paren(GroupParen(g))
                }
                #[next]
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                    EnumVariantBody::Brace(GroupBrace(g))
                }
                #[skip]
                _ => EnumVariantBody::Unit,
            });

            let discriminant = next_if!(match input {
                #[next]
                Some(TokenTree::Punct(eq)) if *eq == '=' => Some({
                    let eq = PunctEq(eq);
                    let discriminant = input
                        .consume_before_or_all(|tt| matches!(tt,TokenTree::Punct(p) if *p == ','));

                    let discriminant = discriminant.into_vec_iter();

                    parse_meta_utils::EqValue {
                        eq,
                        value: discriminant,
                    }
                }),
                #[skip]
                _ => None,
            });

            let mut res = Ok(());
            let comma = next_if!(match input {
                #[next]
                Some(TokenTree::Punct(punct)) if *punct == ',' => Some(PunctComma(punct)),
                #[skip]
                Some(tt) => {
                    res = Err(ParseError::custom(
                        "expect eof or `,`",
                        tt.span_open_or_entire(),
                    ));
                    None
                }
                #[next]
                None::<_> => None,
            });

            () = push(
                resource,
                attrs,
                EnumVariant {
                    vis,
                    name,
                    body,
                    discriminant,
                },
                comma,
            );

            res?;
        }

        input.expect_eof()
    }

    fn parse_vis(&mut self) -> Option<SomeVisibility> {
        let r#pub = next_if!(match self {
            #[next]
            Some(TokenTree::Ident(ident)) if ident_matches!(ident, b"pub") => IdentPub(ident),
            #[skip]
            _ => return None,
        });

        let kind;
        let paren = next_if!(match self {
            #[next]
            Some(TokenTree::Group(g))
                if {
                    kind = SomeVisibilityParenKind::try_parse(g);
                    kind.is_some()
                } =>
                Some({
                    let group = GroupParen(g);
                    let kind = kind.unwrap();
                    SomeVisibilityParen { group, kind: kind }
                }),
            #[skip]
            _ => None,
        });

        Some(SomeVisibility { r#pub, paren })
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

    let vis = input.parse_vis();

    let first_ident = input.parse_ident()?;

    Ok(ParseItemStart { vis, first_ident })
}

pub struct ParseItemStart {
    pub vis: Option<SomeVisibility>,
    pub first_ident: Ident,
}

pub fn parse_meta_simple(
    input: &mut ParsingTokenStream,
) -> Result<MetaSimple<ConsumedTokens<'_>>, ParseError> {
    let path = input.parse_ident()?;

    let after_path = parse_meta_after_path(input);

    Ok(MetaSimple { path, after_path })
}

pub struct MetaSimple<AfterEq> {
    pub path: Ident,
    pub after_path: MetaAfterPath<AfterEq>,
}

pub fn parse_meta_after_path(input: &mut ParsingTokenStream) -> MetaAfterPath<ConsumedTokens<'_>> {
    next_if!(match input {
        #[next]
        Some(TokenTree::Group(g)) => MetaAfterPath::Group(g),
        #[next]
        Some(TokenTree::Punct(p)) if *p == '=' => {
            MetaAfterPath::Eq {
                eq: PunctEq(p),
                before_comma_or_eof: input
                    .consume_before_or_all(|tt| matches!(tt, TokenTree::Punct(p) if *p == ',')),
            }
        }
        #[skip]
        _ => MetaAfterPath::Empty,
    })
}

pub enum MetaAfterPath<AfterEq> {
    Empty,
    Group(Group),
    Eq {
        eq: PunctEq,
        before_comma_or_eof: AfterEq,
    },
}

pub trait CollectSeparated<T, P> {
    fn push_pair(&mut self, item: T, punct: P);

    type Collect;

    fn collect_with_last(self, last: T) -> Self::Collect;
    fn collect(self) -> Self::Collect;
}

impl<P> CollectSeparated<(), P> for () {
    fn push_pair(&mut self, (): (), _: P) {}

    type Collect = ();

    fn collect_with_last(self, (): ()) -> Self::Collect {}

    fn collect(self) -> Self::Collect {}
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
    paren: Option<SomeVisibilityParen>,
}

pub struct SomeVisibilityParen {
    group: GroupParen,
    kind: SomeVisibilityParenKind,
}

impl SomeVisibilityParenKind {
    fn try_parse(g: &Group) -> Option<Self> {
        if g.delimiter() != Delimiter::Parenthesis {
            return None;
        }

        let mut ts = g.stream().into_iter();

        let Some(TokenTree::Ident(ident)) = ts.next() else {
            return None;
        };

        let kind = ident_match!(match ident {
            b"crate" => Self::Crate(IdentCrate(ident)),
            b"self" => Self::Self_(IdentSelf(ident)),
            b"super" => Self::Super(IdentSuper(ident)),
            b"in" => return Some(Self::In(IdentIn(ident), ts)),
            _ => return None,
        });

        if ts.next().is_some() {
            return None;
        }

        Some(kind)
    }
}

enum SomeVisibilityParenKind {
    Crate(IdentCrate),
    Self_(IdentSelf),
    Super(IdentSuper),
    In(IdentIn, proc_macro::token_stream::IntoIter),
}

#[derive(Clone)]
pub struct ParseError(ParseErrorKind, Vec<ParseErrorKind>);

impl ParseError {
    fn into_iter(self) -> impl Iterator<Item = ParseErrorKind> {
        iter::once(self.0).chain(self.1)
    }

    pub fn push(&mut self, e: ParseError) {
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

#[derive(Clone)]
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
impl GroupParen {
    pub fn new(stream: TokenStream) -> Self {
        Self(Group::new(Delimiter::Parenthesis, stream))
    }
    pub fn with_delimiter_span(mut self, span: Span) -> Self {
        self.0.set_span(span);
        self
    }
}

impl Deref for GroupParen {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<Group> for GroupParen {
    type Error = Group;

    fn try_from(g: Group) -> Result<Self, Self::Error> {
        if g.delimiter() == Delimiter::Parenthesis {
            Ok(Self(g))
        } else {
            Err(g)
        }
    }
}
pub struct GroupBrace(Group);

impl Deref for GroupBrace {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<GroupBrace> for Group {
    fn from(value: GroupBrace) -> Self {
        value.0
    }
}

pub struct GroupBracket(Group);

impl Deref for GroupBracket {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GroupBracket {
    pub fn parse_from_token_tree(tt: TokenTree) -> Result<Self, ParseError> {
        match tt {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => Ok(Self(group)),
            _ => Err(syn_generic::ParseError::custom(
                "expect `[`",
                tt.span_open_or_entire(),
            )),
        }
    }
}

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

impl PunctSemi {
    pub fn span(&self) -> Span {
        self.0.span()
    }
}
/// `<`
pub struct PunctLt(Punct);
/// `>`
pub struct PunctGt(Punct);

impl PunctEq {
    pub fn span(&self) -> Span {
        self.0.span()
    }
}

impl Deref for PunctComma {
    type Target = Punct;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `pub`
pub struct IdentPub(Ident);
/// `crate`
pub struct IdentCrate(Ident);
/// `self`
pub struct IdentSelf(Ident);
/// `super`
pub struct IdentSuper(Ident);
/// `in`
pub struct IdentIn(Ident);

/// `where`
pub struct IdentWhere(Ident);

impl From<Span> for IdentWhere {
    fn from(span: Span) -> Self {
        IdentWhere(Ident::new("where", span))
    }
}

impl From<IdentWhere> for Ident {
    fn from(value: IdentWhere) -> Self {
        value.0
    }
}

#[derive(Default, Clone)]
pub struct ErrorCollector(Option<ParseError>);

impl ErrorCollector {
    pub fn push(&mut self, e: ParseError) {
        match &mut self.0 {
            Some(errors) => errors.push(e),
            None => self.0 = Some(e),
        }
    }

    pub fn push_custom(
        &mut self,
        msg: impl Into<std::borrow::Cow<'static, str>>,
        span: impl Into<Option<Span>>,
    ) {
        self.push(ParseError::custom(msg, span));
    }

    pub fn ok(self) -> Result<(), ParseError> {
        match self.0 {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}

pub struct WhereClause<Predicates> {
    pub r#where: IdentWhere,
    pub predicates: Predicates,
}

pub struct EnumVariant {
    pub vis: Option<SomeVisibility>,
    pub name: Ident,
    pub body: EnumVariantBody,
    pub discriminant: Option<parse_meta_utils::EqValue<vec::IntoIter<TokenTree>>>,
}

pub enum EnumVariantBody {
    Unit,
    Paren(GroupParen),
    Brace(GroupBrace),
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

#[derive(Debug)]
pub struct ConsumedTokens<'a> {
    // ts.s is mutated when `ConsumedTokens` is dropped
    // ts.prev_span is always up-to-date
    ts: Option<&'a mut ParsingTokenStream>,
    prev_span: Option<Span>,
    pos: ConsumedTokensPos,
}

impl<'a> From<&'a mut ParsingTokenStream> for ConsumedTokens<'a> {
    fn from(ts: &'a mut ParsingTokenStream) -> Self {
        Self {
            prev_span: ts.prev_span,
            ts: Some(ts),
            pos: ConsumedTokensPos::Partial {
                consumed_len: 0,
                rest_start_from: 0,
            },
        }
    }
}

#[derive(Debug)]
enum ConsumedTokensPos {
    All,
    Partial {
        consumed_len: usize,
        rest_start_from: usize,
    },
}

fn dummy_tt() -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::None, TokenStream::new()))
}

impl<'a> ConsumedTokens<'a> {
    pub fn into_vec_iter(self) -> vec::IntoIter<TokenTree> {
        self.use_tokens(|v| v.into_vec_iter())
    }

    pub fn into_vec(self) -> Vec<TokenTree> {
        self.use_tokens(|v| v.into_vec())
    }

    pub fn use_tokens<T>(self, output: impl FnOnce(TokenStreamCow<'_>) -> T) -> T {
        match self.pos {
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from: _,
            } => {
                if consumed_len == 0 {
                    output(TokenStreamCow::Owned(Default::default()))
                } else {
                    output(TokenStreamCow::Borrowed(
                        &self.ts.as_ref().unwrap().s.as_slice()[..consumed_len],
                    ))
                }
            }
            ConsumedTokensPos::All => {
                let mut this = self;
                let ts = this.ts.take().unwrap();
                this.pos = ConsumedTokensPos::Partial {
                    consumed_len: 0,
                    rest_start_from: 0,
                };

                // ts.prev_span is already kept updated
                let ts = std::mem::take(&mut ts.s);
                output(TokenStreamCow::Owned(ts))
            }
        }
    }

    fn rest_as_slice(&self) -> &[TokenTree] {
        match self.pos {
            ConsumedTokensPos::All => &[],
            ConsumedTokensPos::Partial {
                consumed_len: _,
                rest_start_from,
            } => {
                self.ts
                    .as_ref()
                    .unwrap()
                    .s
                    .as_slice()
                    .split_at(rest_start_from)
                    .1
            }
        }
    }

    fn rest_next(&mut self) -> Option<TokenTree> {
        let ts = self.ts.as_mut().unwrap();
        match &mut self.pos {
            ConsumedTokensPos::All => None,
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from: 0,
            } => {
                debug_assert_eq!(*consumed_len, 0);
                ts.next()
            }
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from,
            } => {
                debug_assert!(*consumed_len <= *rest_start_from);

                let tt = ts.s.as_mut_slice().get_mut(*rest_start_from);

                match tt {
                    Some(tt) => Some({
                        *rest_start_from += 1;
                        ts.prev_span = Some(tt.span_close_or_entire());
                        let tt = std::mem::replace(tt, dummy_tt());
                        tt
                    }),
                    None => None,
                }
            }
        }
    }

    /// Returns Ok(_) if self is empty
    pub fn try_unwrap_rest(mut self) -> Result<&'a mut ParsingTokenStream, Self> {
        match self.pos {
            ConsumedTokensPos::All if self.ts.as_ref().unwrap().is_empty() => {
                self.pos = ConsumedTokensPos::Partial {
                    consumed_len: 0,
                    rest_start_from: 0,
                };

                Ok(self.ts.take().unwrap())
            }
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from: 0,
            } => {
                debug_assert_eq!(consumed_len, 0);
                Ok(self.ts.take().unwrap())
            }
            _ => Err(self),
        }
    }

    pub fn len(&self) -> usize {
        match self.pos {
            ConsumedTokensPos::All => self.ts.as_ref().unwrap().len(),
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from,
            } => {
                debug_assert!(consumed_len <= rest_start_from);
                consumed_len
            }
        }
    }

    pub fn parse(self) -> ParsingConsumedTokens<'a> {
        ParsingConsumedTokens(match self.pos {
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from,
            } => {
                debug_assert!(consumed_len <= rest_start_from);

                let mut this = self;

                let ts = std::mem::take(&mut this.ts).unwrap();

                this.pos = ConsumedTokensPos::Partial {
                    consumed_len: 0,
                    rest_start_from: 0,
                };

                ParsingConsumedTokensInner::Partial {
                    ts,
                    prev_span: this.prev_span,
                    start_from: 0,
                    len: consumed_len,
                    rest_start_from,
                }
            }
            ConsumedTokensPos::All => {
                let mut this = self;

                let ts = std::mem::take(&mut this.ts).unwrap();

                ts.prev_span = this.prev_span;

                this.pos = ConsumedTokensPos::Partial {
                    consumed_len: 0,
                    rest_start_from: 0,
                };

                ParsingConsumedTokensInner::All(ts)
            }
        })
    }
}

impl<'a> Drop for ConsumedTokens<'a> {
    fn drop(&mut self) {
        match self.pos {
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from: 0,
            } => {
                debug_assert_eq!(consumed_len, 0);
            }
            ConsumedTokensPos::Partial {
                consumed_len,
                rest_start_from,
            } => {
                let ts = self.ts.as_mut().unwrap();
                debug_assert!(consumed_len <= rest_start_from);

                let _: TokenTree = ts.s.nth(rest_start_from - 1).unwrap();
            }
            ConsumedTokensPos::All => {
                _ = {
                    let ts = self.ts.as_mut().unwrap();
                    ts.take_all()
                }
            }
        }
    }
}

pub struct ParsingAfterConsumedTokens<'a>(pub ConsumedTokens<'a>);

impl<'a> ParsingAfterConsumedTokens<'a> {
    pub fn first(&self) -> Option<&TokenTree> {
        self.0.rest_as_slice().first()
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        self.0.rest_next()
    }

    pub fn span_open_or_prev(&self) -> Option<Span> {
        match self.first().map(TokenTreeExt::span_open_or_entire) {
            Some(v) => Some(v),
            None => self.prev_span(),
        }
    }

    fn prev_span(&self) -> Option<Span> {
        self.0.ts.as_ref().unwrap().prev_span
    }

    pub fn expect_eof(&self) -> Result<(), ParseError> {
        if self.first().is_none() {
            Ok(())
        } else {
            Err(ParseError::custom("expect eof", self.prev_span()))
        }
    }
}

pub struct ParsingConsumedTokens<'a>(ParsingConsumedTokensInner<'a>);

enum ParsingConsumedTokensInner<'a> {
    All(&'a mut ParsingTokenStream),
    Partial {
        ts: &'a mut ParsingTokenStream,

        prev_span: Option<Span>,
        start_from: usize,
        len: usize,

        rest_start_from: usize,
    },
}

impl<'a> Drop for ParsingConsumedTokensInner<'a> {
    fn drop(&mut self) {
        match self {
            ParsingConsumedTokensInner::All(ts) => _ = ts.take_all(),
            ParsingConsumedTokensInner::Partial {
                ts,
                prev_span: _,
                start_from: _,
                len: _,
                rest_start_from,
            } => {
                let rest_start_from = *rest_start_from;
                if rest_start_from > 0 {
                    _ = ts.s.nth(rest_start_from - 1);
                }
            }
        }
    }
}

impl<'a> ParsingConsumedTokens<'a> {
    pub fn next(&mut self) -> Option<TokenTree> {
        match &mut self.0 {
            ParsingConsumedTokensInner::All(ts) => ts.next(),
            ParsingConsumedTokensInner::Partial {
                ts,
                prev_span,
                start_from,
                len,
                rest_start_from: _,
            } => {
                let all = ts.s.as_mut_slice().split_at_mut(*len).0;

                match all.get_mut(*start_from) {
                    Some(tt) => Some({
                        *start_from += 1;
                        *prev_span = Some(tt.span_close_or_entire());

                        std::mem::replace(tt, dummy_tt())
                    }),
                    None => None,
                }
            }
        }
    }

    pub fn first(&self) -> Option<&TokenTree> {
        match self.0 {
            ParsingConsumedTokensInner::All(ref ts) => ts.first(),
            ParsingConsumedTokensInner::Partial {
                ref ts,
                prev_span: _,
                start_from,
                len,
                rest_start_from: _,
            } => {
                let all = ts.s.as_slice().split_at(len).0;
                all.get(start_from)
            }
        }
    }

    pub fn prev_span(&self) -> Option<Span> {
        match self.0 {
            ParsingConsumedTokensInner::All(ref ts) => ts.prev_span,
            ParsingConsumedTokensInner::Partial { prev_span, .. } => prev_span,
        }
    }

    pub fn expect_eof(&self) -> Result<(), ParseError> {
        if self.first().is_none() {
            Ok(())
        } else {
            Err(ParseError::custom("expect eof", self.prev_span()))
        }
    }
}

impl Parseable for vec::IntoIter<TokenTree> {}
impl Parse<ConsumedTokens<'_>> for vec::IntoIter<TokenTree> {
    type ErrorPayload = NoPayload;
    type Error = Infallible;
    type ErrorWithPayload = Infallible;

    fn parse(input: ConsumedTokens<'_>) -> Result<Self, Self::ErrorWithPayload> {
        Ok(input.into_vec_iter())
    }
}

impl From<Infallible> for ParseError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

pub struct Type();
