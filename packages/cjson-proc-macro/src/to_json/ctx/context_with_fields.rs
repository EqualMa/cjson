use proc_macro::Span;

use crate::to_json::ctx::field::ContextSupportsField;

use super::{StructField, field::ContextOfField};

pub trait ContextWithFields: ContextSupportsField {
    fn fields(&self) -> &[StructField];

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
}

impl<Ctx: ?Sized + ContextWithFields> ContextWithFields for &mut Ctx {
    fn fields(&self) -> &[StructField] {
        Ctx::fields(self)
    }
}
