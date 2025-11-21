use super::ToJson;

impl ToJson for bool {
    type ToJson<'a>
        = super::texts::Boolean
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        super::texts::Boolean(*self)
    }
}

mod r#const {
    use crate::{r#const::ConstIntoJson, ser::texts};

    impl ConstIntoJson<bool> {
        pub const fn const_into_json(self) -> texts::Boolean {
            texts::Boolean(self.0)
        }
    }
}
