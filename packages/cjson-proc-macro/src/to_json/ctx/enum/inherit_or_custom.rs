use proc_macro::TokenTree;

use super::{
    super::{
        CustomTokens, CustomTokensExpandErrorOr, HasConstCircularRefMsg, custom::TokensExpanded,
    },
    ContextOfEnumVariant, MaybeAccessed,
    inherit::{InheritedProp, SpecifiedOrInherited},
};

pub struct InheritOrCustom<E> {
    custom: MaybeAccessed<Option<CustomTokens>>,
    cache: InheritedProp<TokensExpanded<E>>,
}

impl<E> InheritOrCustom<E> {
    pub fn new(custom: Option<CustomTokens>) -> Self {
        Self {
            custom: MaybeAccessed::new(custom),
            cache: Default::default(),
        }
    }

    pub fn is_specified(&self) -> bool {
        self.custom.value.is_some()
    }
}

impl ContextOfEnumVariant<'_> {
    pub fn try_to_unspecified_prop<E: Clone>(
        &mut self,
        get_prop: impl for<'ctx> Fn(&'ctx mut ContextOfEnumVariant<'_>) -> &'ctx mut InheritOrCustom<E>,
        calc_unspecified: impl FnOnce(&mut ContextOfEnumVariant<'_>) -> TokensExpanded<E>,
    ) -> &mut TokensExpanded<E> {
        InheritedProp::unspecified(
            //
            self,
            |this| &mut get_prop(this).cache,
            calc_unspecified,
        )
    }

    pub fn try_to_inherit_or_custom<E: HasConstCircularRefMsg + Clone + 'static>(
        &mut self,
        get_prop: impl for<'ctx> Fn(&'ctx mut ContextOfEnumVariant<'_>) -> &'ctx mut InheritOrCustom<E>,
        get_inherited: impl Fn(&mut super::ContextOfEnum) -> Option<super::CustomTokensNotCalculating>,
        calc_unspecified: impl FnOnce(&mut ContextOfEnumVariant<'_>) -> TokensExpanded<E>,
    ) -> (
        &mut Vec<TokenTree>,
        Result<(), CustomTokensExpandErrorOr<E>>,
    ) {
        match InheritedProp::custom_or_unspecified(
            self,
            |this| &mut get_prop(this).cache,
            |this| super::ContextOfEnumItemTo(this.as_mut()),
            |this| match get_prop(this).custom.access_mut() {
                Some(specified) => Some(SpecifiedOrInherited::Specified(
                    specified.take_for_calculating(),
                )),
                None => get_inherited(&mut this.ctx_enum).map(SpecifiedOrInherited::Inherited),
            },
            calc_unspecified,
            E::default_for_circular_ref,
        ) {
            Ok((ts, res)) => (
                ts,
                res.clone()
                    .map_err(|e| e.map_circular_ref(|()| E::CIRCULAR_REF_MSG))
                    .map_err(CustomTokensExpandErrorOr::Custom),
            ),
            Err((ts, res)) => (
                //
                ts,
                res.clone().map_err(CustomTokensExpandErrorOr::Other),
            ),
        }
    }
}
