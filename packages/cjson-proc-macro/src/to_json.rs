use proc_macro::{Ident, TokenStream};
use typed_quote::{IntoTokens, quote, tokens::IterTokens};

use crate::{
    ErrorCollector, ident_match,
    syn_generic::{self, ParseError, ParseGenericsOutput, SomeVisibility, StructData, WhereClause},
};

pub struct ToJson<'a> {
    pub input: &'a mut syn_generic::ParsingTokenStream,
    pub first_ident: proc_macro::Ident,
}

impl<'a> ToJson<'a> {
    pub fn try_parse(self, errors: &mut ErrorCollector) -> Result<ToJsonItem, ParseError> {
        let Self { input, first_ident } = self;

        enum Kind {
            Struct,
            Enum,
        }

        let kind = ident_match!(match first_ident {
            b"struct" => Kind::Struct,
            b"enum" => Kind::Enum,
            _ =>
                return Err(ParseError::custom(
                    "expect `struct` or `enum`",
                    first_ident.span()
                )),
        });

        let item_name = input.parse_ident()?;

        let ParseGenericsOutput {
            impl_generics,
            ty_generics,
        } = match input.parse_generics() {
            Ok(v) => v,
            Err(e) => {
                errors.push(e);
                Default::default()
            }
        };

        let where_clause;

        let data = match kind {
            Kind::Struct => {
                let struct_data;
                (where_clause, struct_data) = input.parse_struct_after_generics()?;

                ToJsonItemData::Struct(struct_data)
            }
            Kind::Enum => {
                todo!()
            }
        };

        if let Err(e) = input.expect_eof() {
            errors.push(e);
        }

        Ok(ToJsonItem {
            name: item_name,
            impl_generics,
            ty_generics,
            where_clause,
            data,
        })
    }
}

pub struct ToJsonItem {
    name: Ident,
    impl_generics: TokenStream,
    ty_generics: TokenStream,
    where_clause: Option<WhereClause>,
    data: ToJsonItemData,
}
impl ToJsonItem {
    pub fn into_tokens(self, crate_path: impl IntoTokens) -> impl IntoTokens {
        let Self {
            name,
            impl_generics,
            ty_generics,
            where_clause,
            data,
        } = self;

        let where_clause = where_clause.map(
            |WhereClause {
                 r#where,
                 predicates,
             }| {
                let r#where: Ident = r#where.into();
                let predicates = IterTokens(predicates);
                quote!(
                    #r#where
                    #predicates
                )
            },
        );

        let data = data.into_tokens();

        quote!(
            #crate_path ::__private_proc_macro_to_json! {
                (#data)
                {
                    impl_generics[#impl_generics]
                    ty_generics[#ty_generics]
                    where_clause[#where_clause]
                    item_name(#name)
                }
            }
        )
    }
}

enum ToJsonItemData {
    Struct(StructData),
}

impl ToJsonItemData {
    fn into_tokens(self) -> impl IntoTokens {
        match self {
            ToJsonItemData::Struct(struct_data) => {
                let struct_data = struct_data.into_token_stream();
                quote!(struct #struct_data)
            }
        }
    }
}
