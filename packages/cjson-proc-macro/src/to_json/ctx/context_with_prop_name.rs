use proc_macro::{Ident, Span, TokenTree};

use crate::{expand_props, syn_generic::parse_meta_utils::MetaPathSpanWith, to_json::item::Rename};

use super::make_fn_clone_and_set_span;

pub trait ContextWithPropName {
    fn cache_for_name(&mut self) -> &mut Option<Vec<TokenTree>>;

    fn to_calc_name(&mut self) -> CalcName<'_>;

    fn expand_name(&mut self, mut out: expand_props::TokensCollector<'_>, span: Span) {
        let expanded_name = match self.cache_for_name() {
            Some(expanded_name) => expanded_name,
            None => {
                let ts = self.to_calc_name().calc();
                self.cache_for_name().insert(ts)
            }
        };

        out.extend(expanded_name.iter().map(make_fn_clone_and_set_span(span)));
    }
}

pub struct CalcName<'a> {
    pub options: &'a super::Options,
    pub rename: Option<&'a MetaPathSpanWith<Rename>>,
    pub name: &'a Ident,
}

impl<'a> CalcName<'a> {
    fn calc(self) -> Vec<TokenTree> {
        let Self {
            options,
            rename,
            name,
        } = self;
        if let Some(MetaPathSpanWith(rename_span, rename)) = rename {
            rename.to_tokens_as_json_object_key(
                //
                &options.crate_path,
                *rename_span,
                name,
            )
        } else {
            let lit = crate::utils::ident_to_literal_string(name);

            vec![lit.into()]
        }
    }
}
