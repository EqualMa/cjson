use std::collections::HashMap;

use proc_macro::{Ident, Literal, Span, TokenStream, TokenTree};

use crate::{
    ErrorCollector, ident_match,
    syn_generic::{
        GroupParen, ParseError,
        parse_meta_utils::{FlagPresent, MetaPathSpanWith},
    },
    to_json::item::Rename,
};

trait Expand {
    fn expand(&self) -> Result<(), ()>;
}

enum KnownProperties<'a> {
    Errored(ErroredProperties),
    ItemStructAtBracket(ItemStructAtBracket<'a>),
}

pub struct ErroredProperties;
pub struct ItemStructAtBracket<'a>(&'a ItemStruct<'a>);

trait Properties {
    fn get_prop<'a, 'p>(
        &'a mut self,
        prop: &'p TokenTree,
        errors: &mut ErrorCollector,
    ) -> impl Properties + Expand + Into<KnownProperties<'a>>;

    fn at_bracket<'a>(&'a mut self) -> impl Properties + Expand;
}

struct ItemStructProps {
    struct_name: TokenStream,
    fields: Vec<ItemStructFieldProps>,
    field_name_to_index: Option<HashMap<String, usize>>,
}

struct ItemStructPropsThenSelf {}
struct ItemStructPropsThenField {}

enum ItemStructPropsThen {
    Self_(ItemStructPropsThenSelf),
    Field(ItemStructPropsThenField),
    Errored(ErroredProperties),
}

enum PropKey {
    Ident(Ident),
    Literal(Literal),
}

impl Properties for ItemStructProps {
    fn get_prop<'a, 'p>(
        &'a mut self,
        prop: &'p TokenTree,
        errors: &mut ErrorCollector,
    ) -> ItemStructPropsThen {
        enum Then {
            Self_,
            Field,
        }

        let then = match prop {
            TokenTree::Ident(ident) => {
                ident_match!(match ident {
                    b"self" => Ok(Then::Self_),
                    b"field" => Ok(Then::Field),
                    _ => Err("property not defined"),
                })
            }
            _ => Err("invalid property"),
        };

        let then = match then {
            Ok(then) => then,
            Err(e) => {
                errors.push(ParseError::custom(e, prop.span()));
                return ErroredProperties.into();
            }
        };

        match then {
            Then::Self_ => ItemStructPropsThenSelf {}.into(),
            Then::Field => ItemStructPropsThenField {}.into(),
        }
    }

    fn at_bracket<'a>(&'a mut self) -> impl Properties + Expand {
        todo!()
    }
}

struct ItemStructFieldProps {
    name: TokenStream,
    to: TokenStream,
    index_to_str: TokenStream,
    name_or_index_to_str: TokenStream,
    expr: TokenStream,
    type_: TokenStream,
    to_kvs: TokenStream,
    to_items: TokenStream,
}

enum MyTokens {
    NotResolved(),
    Resolving(Resolving),
    AllResolved(AllResolved),
}

struct Resolving(Vec<TokenTree>);

enum MaybeResolvedTokenTrees {
    NotResolved(NotResolvedTokenTrees),
    AllResolved(TokenTree, Vec<TokenTree>),
}

enum NotResolvedTokenTrees {
    RefProp,
    Group,
}

enum AllResolved {
    Resolved(Vec<TokenTree>),
    Verbatim(TokenStream),
}

struct ItemStruct<'a> {
    name: &'a Ident,
    fields: ItemStructFields,
    rename_fields: Option<MetaPathSpanWith<GroupParen>>,
}

enum ItemStructFields {
    Named(HashMap<FieldName, Field>),
    Unnamed(Vec<Field>),
}

struct FieldName(String, Ident);

impl std::hash::Hash for FieldName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Eq for FieldName {}
impl PartialEq for FieldName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

struct Field {
    skip: Option<FlagPresent>,
    rename: Option<Rename>,

    to: PropMaybeAccessed,
    to_kvs: PropMaybeAccessed,
    to_items: PropMaybeAccessed,

    index: usize,

    ty: TokenStream,
}

struct ItemEnum {}

struct Prop<T> {
    value: T,
    accessed: bool,
    cached: TokenStream,
}

enum PropMaybeAccessed {
    NotAccessed(Option<MetaPathSpanWith<GroupParen>>),
    Accessed(TokenStream),
}
