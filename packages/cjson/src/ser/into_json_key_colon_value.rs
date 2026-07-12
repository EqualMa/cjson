use super::{IntoJson, IntoJsonKeyColonValue, json_kinds};

pub trait Sealed {
    type IntoJsonKey: IntoJson<JsonKind = json_kinds::JsonString>;
    type IntoJsonValue: IntoJson;
    fn into_json_key_value(self) -> (Self::IntoJsonKey, Self::IntoJsonValue);
}

impl<K: IntoJson<JsonKind = json_kinds::JsonString>, V: IntoJson> Sealed for (K, V) {
    type IntoJsonKey = K;
    type IntoJsonValue = V;

    fn into_json_key_value(self) -> (Self::IntoJsonKey, Self::IntoJsonValue) {
        self
    }
}
impl<K: IntoJson<JsonKind = json_kinds::JsonString>, V: IntoJson> IntoJsonKeyColonValue for (K, V) {}
