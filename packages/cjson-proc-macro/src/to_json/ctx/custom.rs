use proc_macro::{Span, TokenTree};

use crate::expand_props::{self, TokensCollector};

use super::{CustomTokens, CustomTokensExpandError, CustomTokensExpandErrorOr};

pub struct PropDefaultCustom<D> {
    cache_for_default: Option<D>,
    custom: PropDefaultCustomCustom,
}

impl<D> PropDefaultCustom<D> {
    pub fn new(tokens: Option<CustomTokens>) -> Self {
        Self {
            cache_for_default: None,
            custom: match tokens {
                Some(tokens) => PropDefaultCustomCustom::Custom(PropCustom::new(tokens)),
                None => PropDefaultCustomCustom::Unspecified,
            },
        }
    }
}

impl<D> PropDefaultCustom<D> {
    fn use_default<'a, Ctx, R>(
        ctx: &'a mut Ctx,
        mut get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        calc: impl FnOnce(&mut Ctx) -> D,
        f: impl FnOnce(&mut D) -> R,
    ) -> R
    where
        D: 'a,
    {
        match &mut get_prop(ctx).cache_for_default {
            Some(v) => f(v),
            None => {
                let value = calc(ctx);
                let v = get_prop(ctx).cache_for_default.insert(value);
                f(v)
            }
        }
    }

    fn use_custom_or_default<'a, Ctx, R>(
        ctx: &'a mut Ctx,
        mut get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        custom_dummy: impl FnOnce(Span) -> Vec<TokenTree>,
        calc_default: impl FnOnce(&mut Ctx) -> D,
        f: impl FnOnce(Result<&mut CustomTokensExpanded, &mut D>) -> R,
    ) -> R
    where
        D: 'a,
        Ctx: expand_props::Context,
    {
        match &mut get_prop(ctx).custom {
            PropDefaultCustomCustom::Custom(PropCustom {
                tokens,
                accessed,
                cache,
            }) => {
                let v = match cache {
                    Some(cache) => cache,
                    None => {
                        *accessed = true;
                        let value = tokens
                            .take_for_calculating()
                            .expand(ctx, custom_dummy, || ());
                        let v = match &mut get_prop(ctx).custom {
                            PropDefaultCustomCustom::Custom(PropCustom { cache, .. }) => {
                                cache.insert(value)
                            }
                            PropDefaultCustomCustom::Unspecified => unreachable!(),
                        };
                        v
                    }
                };
                f(Ok(v))
            }
            PropDefaultCustomCustom::Unspecified => {
                Self::use_default(ctx, get_prop, calc_default, |d| f(Err(d)))
            }
        }
    }
}

impl<DE> PropDefaultCustom<TokensExpanded<DE>> {
    fn use_custom_or_default_map_err_cloned<'a, Ctx, R>(
        ctx: &'a mut Ctx,
        get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        calc_default: impl FnOnce(&mut Ctx) -> TokensExpanded<DE>,
        f: impl FnOnce(&mut Vec<TokenTree>, Result<(), CustomTokensExpandErrorOr<DE>>) -> R,
    ) -> R
    where
        DE: 'a,
        Ctx: expand_props::Context,
        DE: super::HasConstCircularRefMsg + Clone,
    {
        Self::use_custom_or_default(
            ctx,
            get_prop,
            DE::default_for_circular_ref,
            calc_default,
            |v| match v {
                Ok((ts, e)) => f(
                    ts,
                    e.clone()
                        .map_err(|e| e.map_circular_ref(|()| DE::CIRCULAR_REF_MSG))
                        .map_err(CustomTokensExpandErrorOr::Custom),
                ),
                Err((ts, e)) => f(ts, e.clone().map_err(CustomTokensExpandErrorOr::Other)),
            },
        )
    }

    pub fn expand_default<'a, Ctx>(
        ctx: &'a mut Ctx,
        get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        calc_default: impl FnOnce(&mut Ctx) -> TokensExpanded<DE>,
        mut out: TokensCollector<'_>,
    ) -> Result<(), DE>
    where
        DE: 'a + Clone,
    {
        Self::use_default(ctx, get_prop, calc_default, |(ts, res)| {
            out.extend_from_slice(ts);
            res.clone()
        })
    }

    pub fn expand_custom_or_default<'a, Ctx>(
        ctx: &'a mut Ctx,
        get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        calc_default: impl FnOnce(&mut Ctx) -> TokensExpanded<DE>,
        mut out: TokensCollector<'_>,
    ) -> Result<(), CustomTokensExpandErrorOr<DE>>
    where
        DE: 'a,
        Ctx: expand_props::Context,
        DE: super::HasConstCircularRefMsg + Clone,
    {
        Self::use_custom_or_default_map_err_cloned(ctx, get_prop, calc_default, |ts, res| {
            out.extend_from_slice(ts);
            res
        })
    }
}

pub type TokensExpanded<E> = (Vec<TokenTree>, Result<(), E>);
pub type CustomTokensExpanded = TokensExpanded<CustomTokensExpandError<()>>;

pub struct PropCustom {
    tokens: CustomTokens,
    accessed: bool,
    cache: Option<CustomTokensExpanded>,
}

impl PropCustom {
    pub fn new(tokens: CustomTokens) -> Self {
        Self {
            tokens,
            accessed: false,
            cache: None,
        }
    }

    pub fn cached(&mut self) -> Option<&mut CustomTokensExpanded> {
        self.cache.as_mut()
    }

    pub fn use_expanded<Ctx: expand_props::Context, R>(
        ctx: &mut Ctx,
        mut get_prop: impl FnMut(&mut Ctx) -> &mut Self,
        dummy: impl FnOnce(Span) -> Vec<TokenTree>,
        f: impl FnOnce(&mut CustomTokensExpanded) -> R,
    ) -> R {
        let this = get_prop(ctx);
        let v = match &mut this.cache {
            Some(cache) => cache,
            None => {
                this.accessed = true;
                let value = this.tokens.take_for_calculating().expand(ctx, dummy, || ());
                let v = get_prop(ctx).cache.insert(value);
                v
            }
        };
        f(v)
    }
}

enum PropDefaultCustomCustom {
    Custom(PropCustom),
    Unspecified,
}
