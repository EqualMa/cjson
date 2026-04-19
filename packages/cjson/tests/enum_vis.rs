#![cfg(feature = "proc-macro")]

// #[derive(cjson::ToJson)]
pub enum PubEnum {
    A,
}

::cjson::impl_to_json!(
    vis![pub],
    impl_generics![],
    where_clause![],
    |self: PubEnum| match self {
        #[cjson(match_branch_name(A))]
        Self::A => json!("A"),
    }
);

#[expect(unused)]
enum PrivateWithPubImpl {
    A,
}

::cjson::impl_to_json!(
    vis![pub],
    impl_generics![],
    where_clause![],
    |self: PrivateWithPubImpl| match self {
        #[cjson(match_branch_name(A))]
        Self::A => json!("A"),
    }
);
