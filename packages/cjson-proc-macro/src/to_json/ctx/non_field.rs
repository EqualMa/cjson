use crate::{
    ErrorCollector,
    expand_props::{PropPath, TokensCollector},
};

pub trait ContextSupportsNonFieldProp {
    fn expand_non_field_prop(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    );
}

impl<Ctx: ?Sized + ContextSupportsNonFieldProp> ContextSupportsNonFieldProp for &mut Ctx {
    fn expand_non_field_prop(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    ) {
        Ctx::expand_non_field_prop(self, prop, out, errors)
    }
}
