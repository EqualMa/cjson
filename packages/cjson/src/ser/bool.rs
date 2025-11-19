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
