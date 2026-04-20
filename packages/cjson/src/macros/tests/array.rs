use crate::ser::{ToJson, exts::TextExt};

struct TestSimple<
    Empty: ToJson + Copy,
    Mixed: ToJson + Copy,
    Nested: ToJson + Copy,
    NegLiteral: ToJson + Copy,
> {
    empty: Empty,
    mixed: Mixed,
    nested: Nested,
    neg_literal: NegLiteral,
}

const fn test_simple()
-> TestSimple<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestSimple {
        empty: {
            let v: crate::r#const::array::EmptyArray = json!([]);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"[]"));
            v
        },
        mixed: {
            let v = json!([false, true, 1u8, 2u128, null, "", "hello", "\nworld"]);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(
                s,
                br#"[false,true,1,2,null,"","hello","\nworld"]"#
            ));
            v
        },
        nested: {
            let v = json!([[["\t", [[[]]]], false]]);
            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, br#"[[["\t",[[[]]]],false]]"#));
            v
        },
        neg_literal: {
            let v = json!([-1i8]);
            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"[-1]"));
            v
        },
    }
}

struct TestRuntime<One: ToJson + Copy, Two: ToJson + Copy, Nested: ToJson + Copy> {
    one: (One, &'static str),
    two: (Two, &'static str),
    nested: (Nested, &'static str),
}

const fn test_runtime() -> TestRuntime<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestRuntime {
        one: (json!([(1)]), "[1]"),
        two: (json!([(1), null, (3)]), "[1,null,3]"),
        nested: (
            json!([1u8, [2u8, [(3), 4u8], [(5)],], 6u8]),
            "[1,[2,[3,4],[5]],6]",
        ),
    }
}

const _: () = {
    test_simple();
};

#[cfg(feature = "alloc")]
#[test]
fn tests() {
    fn to_json_string(v: impl ToJson) -> alloc::string::String {
        v.to_json().into_string().into_inner()
    }

    let TestSimple {
        empty,
        mixed,
        nested,
        neg_literal,
    } = test_simple();

    assert_eq!(to_json_string(empty), "[]");
    assert_eq!(
        to_json_string(mixed),
        r#"[false,true,1,2,null,"","hello","\nworld"]"#
    );

    assert_eq!(to_json_string(nested), r#"[[["\t",[[[]]]],false]]"#);
    assert_eq!(to_json_string(neg_literal), "[-1]");

    let TestRuntime {
        //
        one,
        two,
        nested,
    } = test_runtime();

    assert_eq!(to_json_string(one.0), one.1);
    assert_eq!(to_json_string(two.0), two.1);
    assert_eq!(to_json_string(nested.0), nested.1);
}

#[test]
fn test_chunks() {
    use crate::ser::iter_text_chunk::IterTextChunk as _;
    use crate::ser::traits::IntoTextChunks as _;

    macro_rules! next {
        ($v:expr) => {
            $v.next_text_chunk().as_ref().map(|v| v.as_ref())
        };
    }

    {
        let mut v = json!([true, [[null,], "hello\tworld"]])
            .to_json()
            .into_text_chunks();

        assert_eq!(
            next!(v),
            Some(br#"[true,[[null],"hello\tworld"]]"#.as_slice()),
        );

        assert_eq!(next!(v), None);
    }

    {
        let v = json!([true, [(1), [(json!([null]))], ("hello\tworld")]]);
        let v = crate::r#const::array::NonEmptyArray::new(crate::r#const::value::Value::new({
            enum HasConstCompileTimeChunk {}

            impl HasConstCompileTimeChunk {
                const STATED_CHUNK_STRING: crate::r#const::StatedChunkString<
                    {
                        crate::r#const::ChunkLen::DEFAULT
                            .left_bracket()
                            .json_value(crate::__private_json_expand_token_args_for_len! {
                                json_value true
                            })
                            .comma()
                            .left_bracket()
                            .len()
                    },
                > = {
                    let mut buf =
                        crate::r#const::StatedChunkBuf::new((crate::r#const::State::INIT));
                    buf = buf.left_bracket();
                    buf = crate::r#const::ConstIntoJsonValueString(
                        crate::r#const::ConstIntoJson(true).const_into_json(),
                    )
                    .const_concat_after_stated_chunk_buf(buf);
                    buf = buf.comma();
                    buf = buf.left_bracket();
                    buf.assert()
                };
            }
            impl crate::r#const::HasConstCompileTimeChunk for HasConstCompileTimeChunk {
                const CHUNK: crate::r#const::StatedChunkStr<'static> =
                    Self::STATED_CHUNK_STRING.as_str();
            }
            let cjson_prev_compile_runtime = crate::__private::runtime_kinds::json_value(
                crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::DEFAULT,
                1,
            );
            enum PrevState {}

            impl PrevState {
                const STATE: crate::r#const::State =
                    <HasConstCompileTimeChunk as crate::r#const::HasConstCompileTimeChunk>::CHUNK
                        .next_state()
                        .json_value();
            }
            {
                enum HasConstCompileTimeChunk {}

                impl HasConstCompileTimeChunk {
                    const STATED_CHUNK_STRING: crate::r#const::StatedChunkString<
                        {
                            crate::r#const::ChunkLen::DEFAULT
                                .comma()
                                .left_bracket()
                                .len()
                        },
                    > = {
                        let mut buf = crate::r#const::StatedChunkBuf::new((PrevState::STATE));
                        buf = buf.comma();
                        buf = buf.left_bracket();
                        buf.assert()
                    };
                }
                impl crate::r#const::HasConstCompileTimeChunk for HasConstCompileTimeChunk {
                    const CHUNK: crate::r#const::StatedChunkStr<'static> =
                        Self::STATED_CHUNK_STRING.as_str();
                }
                let cjson_prev_compile_runtime = crate::r#const::ChunkConcat(
                    cjson_prev_compile_runtime,
                    crate::__private::runtime_kinds::json_value(
                        crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::DEFAULT,
                        ({
                            enum HasConstCompileTimeChunk {}

                            impl HasConstCompileTimeChunk {
                                const STATED_CHUNK_STRING: crate::r#const::StatedChunkString<
                                    {
                                        crate::r#const::ChunkLen::DEFAULT
                                    .left_bracket()
                                    .json_value(
                                        crate::__private_json_expand_token_args_for_len! {
                                            json_value crate::__private::well_known_ident::null
                                        },
                                    )
                                    .right_bracket()
                                    .len()
                                    },
                                > = {
                                    let mut buf = crate::r#const::StatedChunkBuf::new(
                                        (crate::r#const::State::INIT),
                                    );
                                    buf = buf.left_bracket();
                                    buf = crate::r#const::ConstIntoJsonValueString(
                                        crate::r#const::ConstIntoJson(
                                            (crate::__private::well_known_ident::null),
                                        )
                                        .const_into_json(),
                                    )
                                    .const_concat_after_stated_chunk_buf(buf);
                                    buf = buf.right_bracket();
                                    buf.assert()
                                };
                            }
                            impl crate::r#const::HasConstCompileTimeChunk for HasConstCompileTimeChunk {
                                const CHUNK: crate::r#const::StatedChunkStr<'static> =
                                    Self::STATED_CHUNK_STRING.as_str();
                            }
                            crate::r#const::CompileTimeChunk:: <HasConstCompileTimeChunk::< >> ::JSON_ARRAY_NON_EMPTY
                        }),
                    ),
                );
                {
                    enum PrevState {}

                    impl PrevState {
                        const STATE:crate::r#const::State =  <HasConstCompileTimeChunk as crate::r#const::HasConstCompileTimeChunk>::CHUNK.next_state().json_value();
                    }
                    {
                        enum HasConstCompileTimeChunk {}

                        impl HasConstCompileTimeChunk {
                            const STATED_CHUNK_STRING: crate::r#const::StatedChunkString<
                                {
                                    crate::r#const::ChunkLen::DEFAULT
                                        .right_bracket()
                                        .comma()
                                        .len()
                                },
                            > = {
                                let mut buf =
                                    crate::r#const::StatedChunkBuf::new((PrevState::STATE));
                                buf = buf.right_bracket();
                                buf = buf.comma();
                                buf.assert()
                            };
                        }
                        impl crate::r#const::HasConstCompileTimeChunk for HasConstCompileTimeChunk {
                            const CHUNK: crate::r#const::StatedChunkStr<'static> =
                                Self::STATED_CHUNK_STRING.as_str();
                        }
                        let cjson_prev_compile_runtime = crate::r#const::ChunkConcat(
                    cjson_prev_compile_runtime,
                    crate::__private::runtime_kinds::json_value(
                        crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::DEFAULT,
                        "hello\tworld",
                    ),
                );
                        {
                            enum PrevState {}

                            impl PrevState {
                                const STATE:crate::r#const::State =  <HasConstCompileTimeChunk as crate::r#const::HasConstCompileTimeChunk>::CHUNK.next_state().json_value();
                            }
                            crate::r#const::ChunkConcat(cjson_prev_compile_runtime, {
                                enum HasConstCompileTimeChunk {}

                                impl HasConstCompileTimeChunk {
                                    const STATED_CHUNK_STRING: crate::r#const::StatedChunkString<
                                        {
                                            crate::r#const::ChunkLen::DEFAULT
                                                .right_bracket()
                                                .right_bracket()
                                                .len()
                                        },
                                    > = {
                                        let mut buf =
                                            crate::r#const::StatedChunkBuf::new((PrevState::STATE));
                                        buf = buf.right_bracket();
                                        buf = buf.right_bracket();
                                        buf.assert()
                                    };
                                }
                                impl crate::r#const::HasConstCompileTimeChunk for HasConstCompileTimeChunk {
                                    const CHUNK: crate::r#const::StatedChunkStr<'static> =
                                        Self::STATED_CHUNK_STRING.as_str();
                                }
                                crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::DEFAULT
                            })
                        }
                    }
                }
            }
        }));
        let mut v = v.to_json().into_text_chunks();

        assert_eq!(next!(v), Some(b"[true,[".as_slice()));
        assert_eq!(next!(v), Some(b"1".as_slice()));
        assert_eq!(next!(v), Some(b",[".as_slice()));
        assert_eq!(next!(v), Some(b"[null]".as_slice()));
        assert_eq!(next!(v), Some(b"],".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b"hello".as_slice()));
        assert_eq!(next!(v), Some(b"\\t".as_slice()));
        assert_eq!(next!(v), Some(b"world".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b"]]".as_slice()));

        assert_eq!(next!(v), None);
    }
}
