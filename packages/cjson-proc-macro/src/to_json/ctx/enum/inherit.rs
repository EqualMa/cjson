use proc_macro::{Span, TokenTree};

use crate::expand_props::Context;

use super::{
    super::{CustomTokens, custom::CustomTokensExpanded},
    CustomTokensExpandError, CustomTokensNotCalculating,
};

pub struct InheritedProp<D> {
    cache_for_unspecified: Option<D>,
    cache_for_custom: InheritedPropCustom,
}

impl<D> Default for InheritedProp<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D> InheritedProp<D> {
    pub fn new() -> Self {
        Self {
            cache_for_unspecified: None,
            cache_for_custom: InheritedPropCustom::NotExpanded {
                is_calculating_inherited: None,
            },
        }
    }

    pub fn cache_for_unspecified(&mut self) -> &mut Option<D> {
        &mut self.cache_for_unspecified
    }

    pub fn custom_or_unspecified<'a, Ctx: Context>(
        ctx: &'a mut Ctx,
        mut get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        mut ctx_inherited: impl FnMut(&mut Ctx) -> super::ContextOfEnumItemTo<'_>,
        get_specified_or_inherited: impl FnOnce(&mut Ctx) -> Option<SpecifiedOrInherited>,
        calc_unspecified: impl FnOnce(&mut Ctx) -> D,
        dummy: impl FnOnce(Span) -> Vec<TokenTree>,
        // f: impl FnOnce(Result<(), &mut D>) -> R,
    ) -> Result<&mut CustomTokensExpanded, &mut D> {
        enum Calculated {
            InheritedCircularRef,
            Unspecified,
        }

        if let InheritedPropCustom::NotExpanded {
            is_calculating_inherited,
        } = &mut get_prop(ctx).cache_for_custom
        {
            let v = 'calc: {
                if let Some(span) = is_calculating_inherited {
                    break 'calc InheritedPropCustom::SpecifiedOrInherited {
                        cached: (
                            dummy(*span),
                            Err(CustomTokensExpandError::CircularRef { msg: () }),
                        ),
                    };
                }
                let v = get_specified_or_inherited(ctx);
                match v {
                    Some(v) => InheritedPropCustom::SpecifiedOrInherited {
                        cached: match v {
                            SpecifiedOrInherited::Specified(v) => v.expand(ctx, dummy, || ()),
                            SpecifiedOrInherited::Inherited(v) => {
                                match &mut get_prop(ctx).cache_for_custom {
                                    InheritedPropCustom::NotExpanded {
                                        is_calculating_inherited,
                                    } => *is_calculating_inherited = Some(v.span),
                                    _ => unreachable!(),
                                }

                                let (ts, res) = CustomTokens::from(v).expand(
                                    &mut ctx_inherited(ctx),
                                    dummy,
                                    || (),
                                );
                                (
                                    ts,
                                    res.inspect_err(|e| {
                                        if let CustomTokensExpandError::CircularRef { msg: () } = e
                                        {
                                            unreachable!()
                                        }
                                    }),
                                )
                            }
                        },
                    },

                    None => InheritedPropCustom::Unspecified,
                }
            };
            get_prop(ctx).cache_for_custom = v;
        }

        let prop = get_prop(ctx);

        if matches!(prop.cache_for_custom, InheritedPropCustom::Unspecified)
            && prop.cache_for_unspecified.is_none()
        {
            let value = calc_unspecified(ctx);
            get_prop(ctx).cache_for_unspecified = Some(value);
        }

        let prop = get_prop(ctx);

        match &mut prop.cache_for_custom {
            InheritedPropCustom::NotExpanded { .. } => unreachable!(),
            InheritedPropCustom::SpecifiedOrInherited { cached } => Ok(cached),
            InheritedPropCustom::Unspecified => Err(match &mut prop.cache_for_unspecified {
                Some(v) => v,
                None => unreachable!(),
            }),
        }
    }

    pub fn unspecified<'a, Ctx>(
        ctx: &'a mut Ctx,
        mut get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        calc_unspecified: impl FnOnce(&mut Ctx) -> D,
    ) -> &mut D {
        let prop = get_prop(ctx);
        if prop.cache_for_unspecified.is_none() {
            let value = calc_unspecified(ctx);
            get_prop(ctx).cache_for_unspecified = Some(value);
        }

        let prop = get_prop(ctx);

        prop.cache_for_unspecified.as_mut().unwrap()
    }
}

enum InheritedPropCustom {
    NotExpanded {
        is_calculating_inherited: Option<Span>,
    },
    SpecifiedOrInherited {
        cached: CustomTokensExpanded,
    },
    Unspecified,
}

pub enum SpecifiedOrInherited {
    Specified(CustomTokens),
    Inherited(CustomTokensNotCalculating),
}
