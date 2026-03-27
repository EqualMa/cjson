use proc_macro::{Delimiter, Group, Ident, Literal, Spacing, Span, TokenStream, TokenTree};

use crate::{ErrorCollector, TokenTreeExt, syn_generic::ParseError};

pub enum Prop {
    Ident(Ident),
    Literal(Literal),
}
impl Prop {
    pub fn span(&self) -> Span {
        match self {
            Prop::Ident(this) => this.span(),
            Prop::Literal(this) => this.span(),
        }
    }
}

pub struct PropPath(pub Prop, pub Vec<Prop>);

pub trait Context {
    fn at_bracket_star<'a>(
        &'a mut self,
        star_span: Span,
        errors: &mut ErrorCollector,
    ) -> impl use<'a, Self> + ContextAtBracketStar;

    fn at_bracket_question<'a>(
        &'a mut self,
        question_span: Span,
        errors: &mut ErrorCollector,
    ) -> Option<impl use<'a, Self> + Context>;

    fn expand_prop(
        &mut self,
        prop: PropPath,
        out: TokensCollector<'_>,
        errors: &mut ErrorCollector,
    );
}

pub struct TokensCollector<'a>(&'a mut Vec<TokenTree>);

impl<'a> From<&'a mut Vec<TokenTree>> for TokensCollector<'a> {
    fn from(value: &'a mut Vec<TokenTree>) -> Self {
        Self(value)
    }
}

impl<'a> TokensCollector<'a> {
    pub fn as_mut(&mut self) -> TokensCollector<'_> {
        TokensCollector(self.0)
    }

    pub fn push(&mut self, tt: TokenTree) {
        self.0.push(tt);
    }

    pub fn extend_from_slice(&mut self, other: &[TokenTree]) {
        self.0.extend_from_slice(other)
    }
}
impl<'a> Extend<TokenTree> for TokensCollector<'a> {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

pub trait ContextAtBracketStar: Context {
    fn has_current(&self) -> bool;
    fn next(&mut self);
}

pub fn expand_ts(
    original_ts: TokenStream,
    ctx: &mut impl Context,
    errors: &mut ErrorCollector,
) -> ExpandTs {
    match expand_ts_impl(original_ts.clone().into_iter(), ctx, errors) {
        MaybeIntact::Intact(_) => MaybeIntact::Intact(original_ts),
        MaybeIntact::Expanded(ts) => MaybeIntact::Expanded(ts),
    }
}

type TokenStreamIntoIter = proc_macro::token_stream::IntoIter;

fn expand_ts_impl(
    ts: TokenStreamIntoIter,
    ctx: &mut impl Context,
    errors: &mut ErrorCollector,
) -> MaybeIntact<Vec<TokenTree>, Vec<TokenTree>> {
    let mut out = Vec::with_capacity(ts.size_hint().0);

    match expand_ts_impl_mut(&mut out, ts, ctx, errors) {
        MaybeIntact::Intact(()) => MaybeIntact::Intact(out),
        MaybeIntact::Expanded(()) => MaybeIntact::Expanded(out),
    }
}

fn expand_ts_impl_mut(
    out: &mut Vec<TokenTree>,
    ts: TokenStreamIntoIter,
    ctx: &mut impl Context,
    errors: &mut ErrorCollector,
) -> MaybeIntact<(), ()> {
    expand_ts_iter_to(From::from(out), ts, ctx, errors)
}

pub fn expand_ts_iter_to(
    mut out: TokensCollector<'_>,
    mut ts: TokenStreamIntoIter,
    ctx: &mut impl Context,
    errors: &mut ErrorCollector,
) -> MaybeIntact<(), ()> {
    let mut any_is_expanded = false;

    while let Some(tt) = ts.next() {
        let mut out = out.as_mut();
        match tt {
            TokenTree::Group(group) => {
                out.push(
                    match expand_group(group, ctx, errors) {
                        MaybeIntact::Intact(g) => g,
                        MaybeIntact::Expanded(g) => {
                            any_is_expanded = true;
                            g.into_group()
                        }
                    }
                    .into(),
                );
            }
            TokenTree::Punct(at) if at == '@' => match ts.next() {
                Some(TokenTree::Ident(at_ident)) => {
                    any_is_expanded = true;

                    ctx.expand_prop(PropPath(Prop::Ident(at_ident), vec![]), out, errors);
                }
                Some(TokenTree::Group(at_group)) => match at_group.delimiter() {
                    Delimiter::Parenthesis => 'paren: {
                        any_is_expanded = true;
                        let mut grouped_ts = at_group.stream().into_iter();

                        let next_prop = |grouped_ts: &mut TokenStreamIntoIter,
                                         errors: &mut ErrorCollector|
                         -> Result<Prop, ()> {
                            let err_span = match grouped_ts.next() {
                                Some(TokenTree::Ident(ident)) => return Ok(Prop::Ident(ident)),
                                Some(TokenTree::Literal(lit)) => return Ok(Prop::Literal(lit)),
                                Some(tt) => tt.span_open_or_entire(),
                                None => at_group.span_close(),
                            };

                            errors.push(ParseError::custom("expect ident or literal", err_span));
                            Err(())
                        };

                        let Ok(first_prop) = next_prop(&mut grouped_ts, errors) else {
                            break 'paren;
                        };

                        let mut paths = Vec::with_capacity(grouped_ts.size_hint().0 / 2);
                        'paths: while let Some(tt) = grouped_ts.next() {
                            match tt {
                                TokenTree::Punct(punct) if punct == '.' => {
                                    let Ok(prop) = next_prop(&mut grouped_ts, errors) else {
                                        break 'paths;
                                    };
                                    paths.push(prop);
                                }
                                tt => {
                                    errors.push(ParseError::custom(
                                        "expect `.`",
                                        tt.span_open_or_entire(),
                                    ));
                                    break 'paths;
                                }
                            }
                        }

                        ctx.expand_prop(
                            //
                            PropPath(first_prop, paths),
                            out,
                            errors,
                        );
                    }
                    Delimiter::Bracket => 'bracket: {
                        let err_span;
                        let after_bracket: Option<TokenTree>;
                        match ts.next() {
                            Some(TokenTree::Punct(p)) if p.spacing() == Spacing::Alone => {
                                match p.as_char() {
                                    '*' => {
                                        any_is_expanded = true;

                                        let mut ctx = ctx.at_bracket_star(p.span(), errors);

                                        let grouped_ts = at_group.stream().into_iter();
                                        while ctx.has_current() {
                                            _ = expand_ts_iter_to(
                                                out.as_mut(),
                                                grouped_ts.clone(),
                                                &mut ctx,
                                                errors,
                                            );
                                            ctx.next();
                                        }

                                        break 'bracket;
                                    }
                                    '?' => {
                                        any_is_expanded = true;

                                        let ctx = ctx.at_bracket_question(p.span(), errors);

                                        if let Some(mut ctx) = ctx {
                                            let grouped_ts = at_group.stream().into_iter();
                                            _ = expand_ts_iter_to(
                                                out, grouped_ts, &mut ctx, errors,
                                            );
                                        }

                                        break 'bracket;
                                    }
                                    _ => {
                                        err_span = p.span();
                                        after_bracket = Some(p.into());
                                    }
                                }
                            }
                            Some(tt) => {
                                err_span = tt.span_open_or_entire();
                                after_bracket = Some(tt);
                            }
                            None => {
                                err_span = at_group.span_close();
                                after_bracket = None;
                            }
                        }

                        out.push(at.into());
                        out.push(at_group.into());
                        if let Some(after_bracket) = after_bracket {
                            out.push(after_bracket);
                        }

                        errors.push(ParseError::custom(
                            "expect `?` or `*` after `@[...]`",
                            err_span,
                        ));
                    }
                    Delimiter::Brace | Delimiter::None => {
                        errors.push(ParseError::custom(
                            "unexpected group after `@`",
                            at_group.span_open(),
                        ));

                        out.push(at.into());
                        out.push(at_group.into());
                    }
                },
                Some(tt) => {
                    out.push(at.into());
                    out.push(tt);
                }
                None => out.push(at.into()),
            },
            tt => out.push(tt),
        }
    }

    if any_is_expanded {
        MaybeIntact::Expanded(())
    } else {
        MaybeIntact::Intact(())
    }
}

pub type ExpandTs = MaybeIntact<TokenStream, Vec<TokenTree>>;
pub type ExpandGroup = MaybeIntact<Group, ExpandedGroup>;

pub fn expand_group(
    group: Group,
    ctx: &mut impl Context,
    errors: &mut ErrorCollector,
) -> ExpandGroup {
    match expand_ts_impl(group.stream().into_iter(), ctx, errors) {
        MaybeIntact::Intact(_) => MaybeIntact::Intact(group),
        MaybeIntact::Expanded(expanded) => MaybeIntact::Expanded(
            //
            ExpandedGroup::from_original_and_expanded(&group, expanded),
        ),
    }
}

pub enum MaybeIntact<I, E> {
    Intact(I),
    Expanded(E),
}

pub struct ExpandedGroup {
    delimiter: Delimiter,
    span: Span,
    stream: Vec<TokenTree>,
}

impl ExpandedGroup {
    pub fn from_original_and_expanded(original: &Group, expanded: Vec<TokenTree>) -> Self {
        Self {
            delimiter: original.delimiter(),
            span: original.span(),
            stream: expanded,
        }
    }

    pub fn into_group(self) -> Group {
        let stream = FromIterator::from_iter(self.stream);
        let mut g = Group::new(self.delimiter, stream);
        g.set_span(self.span);
        g
    }
}

pub struct ErroredContext;
pub enum NeverContext {}

impl Context for ErroredContext {
    fn at_bracket_star<'a>(
        &'a mut self,
        _: Span,
        _: &mut ErrorCollector,
    ) -> impl use<'a> + ContextAtBracketStar {
        ErroredContext
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        _: Span,
        _: &mut ErrorCollector,
    ) -> Option<impl use<'a> + Context> {
        None::<NeverContext>
    }

    fn expand_prop(&mut self, _: PropPath, _: TokensCollector<'_>, _: &mut ErrorCollector) {}
}

impl ContextAtBracketStar for ErroredContext {
    fn has_current(&self) -> bool {
        false
    }

    fn next(&mut self) {}
}

impl Context for NeverContext {
    fn at_bracket_star<'a>(
        &'a mut self,
        _: Span,
        _: &mut ErrorCollector,
    ) -> impl use<'a> + ContextAtBracketStar {
        (match *self {}) as ErroredContext
    }

    fn at_bracket_question<'a>(
        &'a mut self,
        _: Span,
        _: &mut ErrorCollector,
    ) -> Option<impl use<'a> + Context> {
        (match *self {}) as Option<Self>
    }

    fn expand_prop(&mut self, _: PropPath, _: TokensCollector<'_>, _: &mut ErrorCollector) {
        match *self {}
    }
}
