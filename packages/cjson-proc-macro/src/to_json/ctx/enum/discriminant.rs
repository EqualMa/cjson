use proc_macro::{TokenStream, TokenTree};

use typed_quote::{IntoTokenTree as _, WithSpan as _, quote};

use crate::to_json::ctx::EqValue;

pub(super) struct Discriminant {
    eq_value: EqValue,
    expanded_as_json_value: Option<DiscriminantAsJsonValue>,
}

impl From<EqValue> for Discriminant {
    fn from(eq_value: EqValue) -> Self {
        Self {
            eq_value,
            expanded_as_json_value: None,
        }
    }
}

enum DiscriminantAsJsonValue {
    Literal,
    ConstBlock([TokenTree; 2]),
}

impl Discriminant {
    pub(super) fn expand_as_json_value(&mut self) -> &[TokenTree] {
        let v = self.expanded_as_json_value.get_or_insert_with(|| {
            let ts = self.eq_value.value.as_slice();
            match ts {
                [TokenTree::Literal(_)] => DiscriminantAsJsonValue::Literal,
                ts => DiscriminantAsJsonValue::ConstBlock({
                    let span = self.eq_value.eq.span();
                    let ts = TokenStream::from_iter(ts.iter().cloned());
                    [
                        quote!(const).with_default_span(span).into_token_tree(),
                        quote!({ #ts }).with_default_span(span).into_token_tree(),
                    ]
                }),
            }
        });

        match v {
            DiscriminantAsJsonValue::Literal => self.eq_value.value.as_slice(),
            DiscriminantAsJsonValue::ConstBlock(ts) => ts,
        }
    }
}
