#![cfg(feature = "proc-macro")]

// #[derive(cjson::ToJson)]
pub enum PubEnum {
    A,
}

::cjson::impl_json!(|self: PubEnum| #[json_x]
match self {
    Self::A => json_x!("A"),
});

#[expect(unused)]
enum PrivateEnum {
    A,
}

::cjson::impl_json!(|self: PrivateEnum| match self {
    Self::A => json!("A"),
});

#[test]
fn compile_only() {}
