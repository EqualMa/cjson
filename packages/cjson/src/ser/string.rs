use crate::ser::{ToJson, texts};

impl ToJson for str {
    type ToJson<'a>
        = texts::QuotedJsonStringFragment<texts::StrToJsonStringFragment<'a>>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        texts::QuotedJsonStringFragment(texts::StrToJsonStringFragment(self))
    }
}

#[cfg(feature = "alloc")]
mod alloc;

mod r#const;
