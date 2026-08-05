use proc_macro::{Span, TokenStream, TokenTree};

use crate::syn_generic::{
    ErrorCollector, MetaAfterPath, ParseError, TokenTreeExt,
    parse_meta::{IdentTreeCollector, MetaToParse, ParseMeta},
};

pub(crate) struct ItemAttrWhere {
    pub where_span: Span,
    pub bound: TokenStream,
}

impl ItemAttrWhere {
    pub(crate) fn parse_meta_impl(
        meta: MetaToParse<'_, '_>,
        errors: &mut ErrorCollector,
    ) -> Result<Self, ParseError> {
        let path_span = meta.path_span();
        let after_path = meta.after_path;

        let value = match after_path {
            MetaAfterPath::Empty => Err(path_span),
            MetaAfterPath::Group(group) => Err(group.span_open()),
            MetaAfterPath::Eq {
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
                return Err(ParseError::custom("expect `where = (Bounds:)`", span));
            }
        };

        Ok(ItemAttrWhere {
            where_span: path_span,
            bound: value,
        })
    }
}

impl ParseMeta<'_> for ItemAttrWhere {
    fn parse_meta(
        meta: MetaToParse<'_, '_>,
        errors: &mut ErrorCollector,
        _: IdentTreeCollector<'_>,
    ) -> Result<Self, ParseError> {
        Self::parse_meta_impl(meta, errors)
    }
}
