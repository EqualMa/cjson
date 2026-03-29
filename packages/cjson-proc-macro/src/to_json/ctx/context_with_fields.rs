use proc_macro::Span;

use crate::{
    ErrorCollector,
    expand_props::{self, TokensCollector},
    to_json::ctx::field::ContextSupportsField,
};

use super::{StructField, field::ContextOfField};

pub trait ContextWithFields: ContextSupportsField {
    fn fields(&self) -> &[StructField];
    fn fields_ident_to_index(&self) -> Option<&std::collections::HashMap<String, usize>>;

    fn for_each_non_skip_field(
        &mut self,
        span: Span,
        mut f: impl FnMut(ContextOfField<&mut Self>),
    ) {
        let mut cur = self.fields().iter().position(|f| f.not_skipped());

        while let Some(index_field) = cur {
            let ctx = ContextOfField {
                ctx_struct: &mut *self,
                index_field,
                span,
                span_self: None,
            };
            f(ctx);

            let next = index_field + 1;
            cur = match self.fields()[next..].iter().position(|f| f.not_skipped()) {
                Some(pos) => Some(next + pos),
                None => None,
            };
        }
    }

    fn expand_self(
        &mut self,
        span_self: Span,
        rest_prop: Vec<expand_props::Prop>,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let mut rest_prop = rest_prop.into_iter();

        let Some(field_name_or_index) = rest_prop.next() else {
            errors.push_custom("@self cannot expand to tokens", span_self);
            return;
        };

        let span = field_name_or_index.span();

        let index = match &field_name_or_index {
            expand_props::Prop::Ident(ident) => {
                self.fields_ident_to_index()
                    .and_then(|fields_ident_to_index| {
                        fields_ident_to_index.get(&ident.to_string()).copied()
                    })
            }
            expand_props::Prop::Literal(literal) => usize::from_str_radix(&literal.to_string(), 10)
                .ok()
                .and_then(|i| {
                    if i < self.fields().len() {
                        Some(i)
                    } else {
                        None
                    }
                }),
        };

        let Some(index) = index else {
            errors.push_custom("field doesn't exist in struct", span);
            return;
        };

        ContextOfField {
            ctx_struct: self,
            index_field: index,
            span,
            span_self: Some(span_self),
        }
        .expand_field_props_maybe_empty(rest_prop, out, errors)
    }
}

impl<Ctx: ?Sized + ContextWithFields> ContextWithFields for &mut Ctx {
    fn fields(&self) -> &[StructField] {
        Ctx::fields(self)
    }

    fn fields_ident_to_index(&self) -> Option<&std::collections::HashMap<String, usize>> {
        Ctx::fields_ident_to_index(self)
    }
}
