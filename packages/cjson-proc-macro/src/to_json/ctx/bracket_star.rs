use proc_macro::{Ident, Span};

use crate::{
    ErrorCollector,
    expand_props::{self, Context, ContextAtBracketStar, Prop, PropPath, TokensCollector},
    ident_matches,
};

use super::field::ContextSupportsField;

pub trait Field {
    fn not_skipped(&self) -> bool;
    fn skipped(&self) -> bool;
}

pub trait ContextSupportsAtBracketStar: ContextSupportsField {
    const MSG_CANNOT_NEST_BRACKET_STAR: &'static str;
    type Field: Field;
    fn fields(&self) -> &[Self::Field];

    fn should_expand_bracket_question(&self, field_index: usize) -> Result<bool, &'static str>;

    fn expand_non_field_prop(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    );
}

impl<Ctx: ContextSupportsAtBracketStar> ContextSupportsAtBracketStar for &mut Ctx {
    const MSG_CANNOT_NEST_BRACKET_STAR: &'static str = Ctx::MSG_CANNOT_NEST_BRACKET_STAR;
    type Field = Ctx::Field;

    fn fields(&self) -> &[Self::Field] {
        Ctx::fields(self)
    }

    fn should_expand_bracket_question(&self, field_index: usize) -> Result<bool, &'static str> {
        Ctx::should_expand_bracket_question(self, field_index)
    }

    fn expand_non_field_prop(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        Ctx::expand_non_field_prop(self, prop, out, errors)
    }
}

pub struct ContextAtBracketStarOf<Ctx: ContextSupportsAtBracketStar> {
    ctx: Ctx,
    #[expect(unused)]
    star_span: Span,

    // asserts `ctx.fields()[index].not_skipped()` or `index == ctx.fields().len()`
    index: usize,
}

pub struct ContextAtBracketQuestionInsideStarOf<'a, Ctx: ContextSupportsAtBracketStar> {
    ctx_star: &'a mut ContextAtBracketStarOf<Ctx>,
    #[expect(unused)]
    question_span: Span,
}

impl<Ctx: ContextSupportsAtBracketStar> Drop for ContextAtBracketStarOf<Ctx> {
    fn drop(&mut self) {
        if self.index != self.ctx.fields().len() {
            panic!(
                "ContextAtBracketStarOf<{}> not fully expanded",
                std::any::type_name::<Ctx>()
            )
        }
    }
}

impl<Ctx: ContextSupportsAtBracketStar> ContextAtBracketStarOf<Ctx> {
    pub(super) fn new(ctx: Ctx, star_span: Span) -> Self {
        Self {
            index: ctx
                .fields()
                .iter()
                .position(|f| f.not_skipped())
                .unwrap_or(ctx.fields().len()),
            ctx,
            star_span,
        }
    }
}

impl<Ctx: ContextSupportsAtBracketStar> Context for ContextAtBracketStarOf<Ctx> {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        errors: &mut ErrorCollector,
    ) -> impl use<'a, Ctx> + ContextAtBracketStar {
        errors.push_custom(Ctx::MSG_CANNOT_NEST_BRACKET_STAR, star_span);
        expand_props::ErroredContext
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a, Ctx> + Context> {
        match Ctx::should_expand_bracket_question(&self.ctx, self.index) {
            Ok(true) => Some(ContextAtBracketQuestionInsideStarOf {
                ctx_star: self,
                question_span,
            }),
            Ok(false) => None,
            Err(msg) => {
                errors.push_custom(msg, question_span);
                None
            }
        }
    }

    fn expand_prop(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        let index = self.index;
        if index < self.ctx.fields().len() {
            // continue
        } else {
            errors.push_custom(
                "ContextAtBracketStarOf overflowed. \
                    Make sure to check has_current() before expand_prop().",
                prop.0.span(),
            );
            return;
        }

        match field_or(prop) {
            Ok((field_ident, rest_prop)) => {
                expand_field_prop(
                    &mut self.ctx,
                    self.index,
                    field_ident,
                    rest_prop,
                    out,
                    errors,
                );
            }
            Err(prop) => self.ctx.expand_non_field_prop(prop, out, errors),
        }
    }
}

fn expand_field_prop(
    ctx: impl ContextSupportsField,
    field_index: usize,
    field_ident: IdentField,
    rest_prop: Vec<Prop>,
    out: TokensCollector<'_>,
    errors: &mut ErrorCollector,
) {
    super::field::ContextOfField {
        ctx_struct: ctx,
        index_field: field_index,
        span: field_ident.span(),
        span_self: None,
    }
    .expand_field_props_maybe_empty(rest_prop.into_iter(), out, errors)
}

impl<Ctx: ContextSupportsAtBracketStar> ContextAtBracketStar for ContextAtBracketStarOf<Ctx> {
    fn has_current(&self) -> bool {
        self.index < self.ctx.fields().len()
    }

    fn next(&mut self) {
        if self.index < self.ctx.fields().len() {
            self.index += 1;
            match self
                .ctx
                .fields()
                .split_at(self.index)
                .1
                .iter()
                .position(|f| f.not_skipped())
            {
                Some(pos) => self.index += pos,
                None => self.index = self.ctx.fields().len(),
            }
        }
    }
}

impl<'this, Ctx: ContextSupportsAtBracketStar> Context
    for ContextAtBracketQuestionInsideStarOf<'this, Ctx>
{
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        errors: &mut ErrorCollector,
    ) -> impl use<'a, 'this, Ctx> + ContextAtBracketStar {
        errors.push_custom("`@[...]*` cannot be nested", star_span);
        expand_props::ErroredContext
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a, 'this, Ctx> + Context> {
        errors.push_custom("`@[...]?` cannot be nested", question_span);
        None::<expand_props::NeverContext>
    }

    fn expand_prop(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        self.ctx_star.expand_prop(prop, out, errors)
    }
}

pub fn field_or(prop: PropPath) -> Result<(IdentField, Vec<Prop>), PropPath> {
    match prop.0 {
        Prop::Ident(ident) if ident_matches!(ident, b"field") => Ok((IdentField(ident), prop.1)),
        _ => return Err(prop),
    }
}

pub struct IdentField(Ident);

impl std::ops::Deref for IdentField {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
