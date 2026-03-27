use std::vec;

use proc_macro::{Ident, Span, TokenTree};

use crate::{ErrorCollector, IdentTree};

use super::{ConsumedTokens, MetaAfterPath, ParseError, PunctEq};

pub struct MetaToParse<'a, 's> {
    path: &'a Ident,
    pub after_path: MetaAfterPath<ConsumedTokens<'s>>,
}

impl<'a, 's> MetaToParse<'a, 's> {
    pub fn path_span(&self) -> Span {
        self.path.span()
    }

    pub fn from_ident(path: &'a Ident, after_path: MetaAfterPath<ConsumedTokens<'s>>) -> Self {
        Self { path, after_path }
    }
}

pub trait ParseMeta<'s>: Sized {
    fn parse_meta(
        input: MetaToParse<'_, 's>,
        errors: &mut ErrorCollector,
        ident_trees: IdentTreeCollector<'_>,
    ) -> Result<Self, ParseError>;
}

pub struct IdentTreeCollector<'a>(&'a mut Vec<IdentTree>);

impl<'a> From<&'a mut Vec<IdentTree>> for IdentTreeCollector<'a> {
    fn from(value: &'a mut Vec<IdentTree>) -> Self {
        Self(value)
    }
}

impl<'a> IdentTreeCollector<'a> {
    pub fn push(&mut self, it: IdentTree) {
        self.0.push(it)
    }
}
