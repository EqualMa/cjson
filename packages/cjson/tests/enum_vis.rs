#![cfg(feature = "proc-macro")]

// #[derive(cjson::ToJson)]
pub enum PubEnum {
    A,
}

::cjson::impl_json!(
    impl_generics![],
    where_clause![],
    |self: PubEnum| match self {
        Self::A => json!("A"),
    }
);

#[expect(unused)]
enum PrivateWithPubImpl {
    A,
}

::cjson::impl_json!(
    impl_generics![],
    where_clause![],
    |self: PrivateWithPubImpl| match self {
        Self::A => json!("A"),
    }
);
