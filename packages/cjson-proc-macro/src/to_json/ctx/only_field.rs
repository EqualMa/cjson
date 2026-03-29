use proc_macro::Span;

use crate::{ErrorCollector, expand_props};

use super::{
    OnlyFieldError, OnlyFieldResult, StructField, bracket_star,
    context_with_fields::ContextWithFields, field::ContextOfField,
};

pub trait ContextSupportsOnlyField: ContextWithFields {
    fn cache_for_only_field_index(&mut self) -> &mut Option<OnlyFieldResult<usize>>;

    fn only_field_index(&mut self) -> OnlyFieldResult<usize> {
        let out = match self.cache_for_only_field_index() {
            Some(v) => v,
            None => {
                let v = calc_only_field_index(self.fields());
                self.cache_for_only_field_index().insert(v)
            }
        };

        *out
    }

    fn context_of_only_field(
        &mut self,
        span: Span,
        span_self: Option<Span>,
    ) -> OnlyFieldResult<ContextOfField<&'_ mut Self>>
    where
        Self: super::field::ContextSupportsField,
    {
        self.only_field_index().map(|index| ContextOfField {
            ctx_struct: self,
            index_field: index,
            span,
            span_self,
        })
    }

    fn expand_only_field(
        &mut self,
        first_ident_span: Span,
        rest_prop: Vec<expand_props::Prop>,
        out: expand_props::TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) where
        Self: super::field::ContextSupportsField,
    {
        let Some(mut ctx) = self
            .context_of_only_field(first_ident_span, None)
            .report(errors, first_ident_span)
        else {
            return;
        };
        ctx.expand_field_props_maybe_empty(rest_prop.into_iter(), out, errors)
    }

    fn expand_only_field_or_non_field<'a>(
        &'a mut self,
        prop: expand_props::PropPath,
        out: expand_props::TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) where
        Self: super::non_field::ContextSupportsNonFieldProp,
    {
        match bracket_star::field_or(prop) {
            Ok((field_ident, rest_prop)) => {
                self.expand_only_field(field_ident.span(), rest_prop, out, errors)
            }
            Err(prop) => self.expand_non_field_prop(prop, out, errors),
        }
    }
}

fn calc_only_field_index(fields: &[StructField]) -> OnlyFieldResult<usize> {
    let mut idx = None;
    let mut too_many = false;
    fields.iter().enumerate().for_each(|(i, field)| {
        if field.skipped() {
            return;
        }

        if idx.is_none() {
            idx = Some(i)
        } else {
            too_many = true;
        }
    });

    match idx {
        Some(idx) => OnlyFieldResult::Existing(
            idx,
            if too_many {
                Some(OnlyFieldError(
                    "`@field` is ambiguous when there are more than one fields without `#[cjson(skip)]`",
                ))
            } else {
                None
            },
        ),
        None => {
            let error =
                OnlyFieldError("`@field` requires exactly one field without `#[cjson(skip)]`");
            if fields.len() == 0 {
                OnlyFieldResult::EmptyFields(error)
            } else {
                OnlyFieldResult::Existing(0, Some(error))
            }
        }
    }
}
