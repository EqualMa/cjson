macro_rules! assert_json_eq {
    ($v:expr, $eq:expr) => {
        assert_eq!(::cjson::ser::ToJsonExt::to_json_as_string(&$v), $eq);
        assert_eq!(
            ::cjson::ser::ToJsonExt::to_json_as_try::<::cjson::ser::IoWrite<Vec<u8>>>(&$v)
                .unwrap()
                .0,
            $eq.as_bytes()
        );
        // TODO: test async try
        assert_eq!(::cjson::ser::IntoJsonExt::into_json_as_string($v), $eq);
        assert_eq!(
            ::cjson::ser::IntoJsonExt::into_json_as_try::<::cjson::ser::IoWrite<Vec<u8>>>($v)
                .unwrap()
                .0,
            $eq.as_bytes()
        );
    };
}

#[test]
fn tuples() {
    assert_json_eq!((1,), "[1]");
    assert_json_eq!(
        (
            cjson::values::Null,
            true,
            2,
            3,
            4,
            5,
            "6",
            7,
            8,
            9,
            10,
            "11"
        ),
        r#"[null,true,2,3,4,5,"6",7,8,9,10,"11"]"#
    );
}
