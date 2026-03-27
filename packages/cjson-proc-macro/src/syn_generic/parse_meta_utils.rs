use std::vec;

use proc_macro::{Span, TokenTree};

use crate::ConsumedTokens;

use super::{
    ErrorCollector, GroupParen, MetaAfterPath, Parse, ParseError, PunctEq,
    parse_meta::{IdentTreeCollector, MetaToParse, ParseMeta},
};

pub struct FlagPresent(pub Span);

impl ParseMeta<'_> for FlagPresent {
    fn parse_meta(
        input: MetaToParse<'_, '_>,
        errors: &mut ErrorCollector,
        _: IdentTreeCollector<'_>,
    ) -> Result<Self, ParseError> {
        let span = input.path_span();
        'err: {
            let err_span = match input.after_path {
                MetaAfterPath::Empty => break 'err,
                MetaAfterPath::Group(group) => group.span_open(),
                MetaAfterPath::Eq {
                    eq,
                    before_comma_or_eof: _,
                } => eq.span(),
            };

            errors.push(ParseError::custom("expect eof", err_span))
        }

        Ok(Self(span))
    }
}

pub struct EqValue<V> {
    pub eq: PunctEq,
    pub value: V,
}

impl<'s, V: Parse<ConsumedTokens<'s>>> ParseMeta<'s> for EqValue<V> {
    fn parse_meta(
        input: MetaToParse<'_, 's>,
        errors: &mut ErrorCollector,
        _: IdentTreeCollector<'_>,
    ) -> Result<Self, ParseError> {
        let err_span = match input.after_path {
            MetaAfterPath::Empty => input.path_span(),
            MetaAfterPath::Group(group) => group.span_open(),
            MetaAfterPath::Eq {
                eq,
                before_comma_or_eof,
            } => {
                return Ok(Self {
                    eq,
                    value: V::parse_and_report(before_comma_or_eof, errors)
                        .map_err(Into::<ParseError>::into)?,
                });
            }
        };

        return Err(ParseError::custom("expect ``= ...`", err_span));
    }
}

impl ParseMeta<'_> for GroupParen {
    fn parse_meta(
        input: MetaToParse<'_, '_>,
        _: &mut ErrorCollector,
        _: IdentTreeCollector<'_>,
    ) -> Result<Self, ParseError> {
        let err_span = match input.after_path {
            MetaAfterPath::Empty => input.path_span(),
            MetaAfterPath::Group(group) => match GroupParen::try_from(group) {
                Ok(v) => return Ok(v),
                Err(g) => g.span_open(),
            },
            MetaAfterPath::Eq {
                eq,
                before_comma_or_eof: _,
            } => eq.span(),
        };

        Err(ParseError::custom("expect `(...)`", err_span))
    }
}

pub struct MetaPathSpanWith<V>(pub Span, pub V);

impl<'s, V: ParseMeta<'s>> ParseMeta<'s> for MetaPathSpanWith<V> {
    fn parse_meta(
        input: MetaToParse<'_, 's>,
        errors: &mut ErrorCollector,
        ident_trees: IdentTreeCollector<'_>,
    ) -> Result<Self, ParseError> {
        Ok(MetaPathSpanWith(
            input.path_span(),
            V::parse_meta(input, errors, ident_trees)?,
        ))
    }
}
