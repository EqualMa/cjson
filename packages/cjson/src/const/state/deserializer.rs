use super::{AfterEndArrayOrObject, Intermediate, IntermediateState, Stack, StateInner};

macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err(err) => return ::core::result::Result::Err(err),
        }
    };
}

pub(super) struct Deserializer<'a>(&'a [u8]);

impl<'a> Deserializer<'a> {
    pub(super) const fn new(s: &'a str) -> Self {
        Self(s.as_bytes())
    }

    pub const fn peek(&self) -> Option<u8> {
        self.0.first().copied()
    }

    pub const fn parse_till_eof_with_state(
        mut self,
        state: StateInner,
    ) -> Result<StateInner, &'static str> {
        match state {
            StateInner::Init => match tri!(self.parse_json_value_no_nesting()) {
                ParseJsonValueNoNestingOutput::JsonValue => {
                    tri!(self.expect_eof_after_value());
                    Ok(StateInner::Eof)
                }
                ParseJsonValueNoNestingOutput::EofInString => {
                    Ok(StateInner::Intermediate(Intermediate {
                        stack: Stack::INIT,
                        state: IntermediateState::InString,
                    }))
                }
                ParseJsonValueNoNestingOutput::ArrayStart => self
                    .parse_till_eof_with_intermediate_state(Intermediate {
                        stack: Stack::INIT.start_array(),
                        state: IntermediateState::AfterArrayStart,
                    }),
                ParseJsonValueNoNestingOutput::ObjectStart => self
                    .parse_till_eof_with_intermediate_state(Intermediate {
                        stack: Stack::INIT.start_object(),
                        state: IntermediateState::AfterObjectStart,
                    }),
            },
            StateInner::Intermediate(inter) => self.parse_till_eof_with_intermediate_state(inter),
            StateInner::Eof => {
                tri!(self.expect_eof_after_value());
                Ok(StateInner::Eof)
            }
        }
    }

    const fn expect_eof_after_value(&self) -> Result<(), &'static str> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err("expect EOF after json value")
        }
    }

    const fn parse_till_eof_with_intermediate_state(
        mut self,
        mut inter_state: Intermediate,
    ) -> Result<StateInner, &'static str> {
        macro_rules! parse_value {
            ($after_value:expr) => {
                match tri!(self.parse_json_value_no_nesting()) {
                    ParseJsonValueNoNestingOutput::JsonValue => inter_state.state = $after_value,
                    ParseJsonValueNoNestingOutput::EofInString => {
                        return Ok(StateInner::Intermediate(Intermediate {
                            stack: inter_state.stack,
                            state: IntermediateState::InString,
                        }));
                    }
                    ParseJsonValueNoNestingOutput::ArrayStart => {
                        inter_state = Intermediate {
                            stack: inter_state.stack.start_array(),
                            state: IntermediateState::AfterArrayStart,
                        }
                    }
                    ParseJsonValueNoNestingOutput::ObjectStart => {
                        inter_state = Intermediate {
                            stack: inter_state.stack.start_object(),
                            state: IntermediateState::AfterObjectStart,
                        }
                    }
                }
            };
        }

        macro_rules! next_byte_or_return_current {
            ($this:expr) => {
                match $this.next_byte() {
                    Some(b) => b,
                    None::<_> => return Ok(StateInner::Intermediate(inter_state)),
                }
            };
        }

        loop {
            match inter_state.state {
                IntermediateState::InString => match tri!(self.parse_in_string()) {
                    ParseInString::StringEnded => match inter_state.stack.is_in_array_or_object() {
                        Some(true) => inter_state.state = IntermediateState::AfterArrayItem,
                        Some(false) => inter_state.state = IntermediateState::AfterObjectFieldValue,
                        None => {
                            return {
                                tri!(self.expect_eof_after_value());
                                Ok(StateInner::Eof)
                            };
                        }
                    },
                    ParseInString::EofInString => {
                        return Ok(StateInner::Intermediate(Intermediate {
                            stack: inter_state.stack,
                            state: IntermediateState::InString,
                        }));
                    }
                },
                IntermediateState::AfterArrayStart => {
                    if matches!(self.peek(), Some(b']')) {
                        self.eat_byte();
                        match inter_state.stack.end_array() {
                            AfterEndArrayOrObject::Intermediate(new_state) => {
                                inter_state = new_state
                            }
                            AfterEndArrayOrObject::Eof => {
                                return {
                                    tri!(self.expect_eof_after_value());
                                    Ok(StateInner::Eof)
                                };
                            }
                        }
                    } else {
                        parse_value!(IntermediateState::AfterArrayItem)
                    }
                }
                IntermediateState::AfterArrayItem => match next_byte_or_return_current!(self) {
                    b',' => inter_state.state = IntermediateState::AfterArrayComma,
                    b']' => match inter_state.stack.end_array() {
                        AfterEndArrayOrObject::Intermediate(new_state) => inter_state = new_state,
                        AfterEndArrayOrObject::Eof => {
                            return {
                                tri!(self.expect_eof_after_value());
                                Ok(StateInner::Eof)
                            };
                        }
                    },
                    _ => return Err("unexpect chars after array item"),
                },
                IntermediateState::AfterArrayStartOrItem => {
                    match next_byte_or_return_current!(self) {
                        b']' => match inter_state.stack.end_array() {
                            AfterEndArrayOrObject::Intermediate(new_state) => {
                                inter_state = new_state
                            }
                            AfterEndArrayOrObject::Eof => {
                                return {
                                    tri!(self.expect_eof_after_value());
                                    Ok(StateInner::Eof)
                                };
                            }
                        },
                        _ => return Err("expect `]` after array start or item"),
                    }
                }

                // note that for AfterArrayStartOrComma: (']' | value) & value = value
                IntermediateState::AfterArrayComma | IntermediateState::AfterArrayStartOrComma => {
                    if self.0.is_empty() {
                        return Ok(StateInner::Intermediate(inter_state));
                    } else {
                        parse_value!(IntermediateState::AfterArrayItem)
                    }
                }
                IntermediateState::AfterObjectStart => match next_byte_or_return_current!(self) {
                    b'}' => match inter_state.stack.end_object() {
                        AfterEndArrayOrObject::Intermediate(new_state) => inter_state = new_state,
                        AfterEndArrayOrObject::Eof => {
                            return {
                                tri!(self.expect_eof_after_value());
                                Ok(StateInner::Eof)
                            };
                        }
                    },
                    b'"' => inter_state.state = IntermediateState::InObjectFieldName,
                    _ => return Err("unexpect chars after object start"),
                },
                IntermediateState::InObjectFieldName => match tri!(self.parse_in_string()) {
                    ParseInString::StringEnded => {
                        inter_state.state = IntermediateState::AfterObjectFieldName
                    }
                    ParseInString::EofInString => {
                        return Ok(StateInner::Intermediate(Intermediate {
                            stack: inter_state.stack,
                            state: IntermediateState::InObjectFieldName,
                        }));
                    }
                },
                IntermediateState::AfterObjectFieldName => {
                    match next_byte_or_return_current!(self) {
                        b':' => inter_state.state = IntermediateState::AfterObjectFieldColon,
                        _ => return Err("unexpect chars after object start"),
                    }
                }
                IntermediateState::AfterObjectFieldColon => {
                    parse_value!(IntermediateState::AfterObjectFieldValue)
                }
                IntermediateState::AfterObjectFieldValue => {
                    match next_byte_or_return_current!(self) {
                        b',' => inter_state.state = IntermediateState::AfterObjectComma,
                        b'}' => match inter_state.stack.end_object() {
                            AfterEndArrayOrObject::Intermediate(new_state) => {
                                inter_state = new_state
                            }
                            AfterEndArrayOrObject::Eof => {
                                return {
                                    tri!(self.expect_eof_after_value());
                                    Ok(StateInner::Eof)
                                };
                            }
                        },
                        _ => return Err("unexpect chars after object field"),
                    }
                }
                IntermediateState::AfterObjectComma => match next_byte_or_return_current!(self) {
                    b'"' => inter_state.state = IntermediateState::InObjectFieldName,
                    _ => return Err("unexpected chars after comma in object"),
                },
                IntermediateState::AfterObjectStartOrComma => {
                    // note that ('"' | '}') & '"' = '"'
                    match next_byte_or_return_current!(self) {
                        b'"' => inter_state.state = IntermediateState::InObjectFieldName,
                        _ => return Err("unexpected chars after object after or comma"),
                    }
                }
                IntermediateState::AfterObjectStartOrFieldValue => {
                    // note that ('"' | '}') & (',' | '}') = '}'
                    match next_byte_or_return_current!(self) {
                        b'}' => match inter_state.stack.end_object() {
                            AfterEndArrayOrObject::Intermediate(new_state) => {
                                inter_state = new_state
                            }
                            AfterEndArrayOrObject::Eof => {
                                return {
                                    tri!(self.expect_eof_after_value());
                                    Ok(StateInner::Eof)
                                };
                            }
                        },
                        _ => return Err("unexpect chars after object start or field value"),
                    }
                }
            }
        }
    }

    #[cfg(todo)]
    const fn parse_json_value(&mut self) -> Result<ParseJsonValueOutput, &'static str> {
        match tri!(self.parse_json_value_no_nesting()) {
            ParseJsonValueNoNestingOutput::JsonValue => Ok(ParseJsonValueOutput::JsonValue),
            ParseJsonValueNoNestingOutput::EofInString => Ok(ParseJsonValueOutput::EofInString),
            ParseJsonValueNoNestingOutput::ArrayStart => match self.peek() {
                Some(b']') => {
                    self.eat_char();
                    Ok(ParseJsonValueOutput::JsonValue)
                }
                Some(_) => {
                    let mut stack = Stack::INIT.start_array();

                    loop {
                        match tri!(self.parse_json_value()) {
                            ParseJsonValueOutput::EofAfterArrayStart => todo!(),
                            ParseJsonValueOutput::JsonValue => match self.peek() {
                                Some(b',') => {
                                    self.eat_char();
                                    // parse next item
                                }
                                Some(b']') => {
                                    self.eat_char();
                                    stack.end_array();

                                    if stack.is_in_array_or_object().is_none() {}
                                }
                                None => {
                                    return Ok(ParseJsonValueOutput::EofAfterArrayItem { stack });
                                }
                            },
                            ParseJsonValueOutput::EofInString => todo!(),
                        }
                    }
                }
                None => Ok(ParseJsonValueOutput::EofAfterArrayStart),
            },
            ParseJsonValueNoNestingOutput::ObjectStart => {}
        }
    }

    const fn parse_in_string(&mut self) -> Result<ParseInString, &'static str> {
        loop {
            match self.next_byte() {
                Some(b'"') => return Ok(ParseInString::StringEnded),
                None::<_> => return Ok(ParseInString::EofInString),
                Some(b'\\') => {
                    //   %x22 /          ; "    quotation mark  U+0022
                    //   %x5C /          ; \    reverse solidus U+005C
                    //   %x2F /          ; /    solidus         U+002F
                    //   %x62 /          ; b    backspace       U+0008
                    //   %x66 /          ; f    form feed       U+000C
                    //   %x6E /          ; n    line feed       U+000A
                    //   %x72 /          ; r    carriage return U+000D
                    //   %x74 /          ; t    tab             U+0009
                    //   %x75 4HEXDIG )  ; uXXXX                U+XXXX
                    match self.next_byte() {
                        Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => {
                            // simple escape
                        }
                        Some(b'u') => {
                            macro_rules! HEXDIG {
                                () => [ b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f' ];
                            }
                            match self.next_n::<4>() {
                                Some([HEXDIG!(), HEXDIG!(), HEXDIG!(), HEXDIG!()]) => {
                                    // unicode escape
                                }
                                Some(_) => {
                                    return Err("unexpected char in string unicode escape");
                                }
                                None => return Err("unexpected EOF in string unicode escape"),
                            }
                        }
                        Some(_) => return Err("unexpected byte in string escape"),
                        None => return Err("unexpected EOF in string escape"),
                    }
                }
                Some(0x0..=0x1F) => return Err("unexpected ASCII control character in string"),
                Some(_) => {
                    // skip
                }
            }
        }
    }

    const fn parse_json_value_no_nesting(
        &mut self,
    ) -> Result<ParseJsonValueNoNestingOutput, &'static str> {
        let Some(peek) = self.peek() else {
            return Err("unexpected EOF");
        };

        let out = match peek {
            b'n' => {
                self.eat_byte();

                match self.next_n() {
                    Some(b"ull") => {}
                    _ => return Err("unexpect ident starting with n"),
                }

                ParseJsonValueNoNestingOutput::JsonValue
            }
            b't' => {
                self.eat_byte();
                match self.next_n() {
                    Some(b"rue") => {}
                    _ => return Err("unexpect ident starting with t"),
                }
                ParseJsonValueNoNestingOutput::JsonValue
            }
            b'f' => {
                self.eat_byte();
                match self.next_n() {
                    Some(b"alse") => {}
                    _ => return Err("unexpect ident starting with t"),
                }
                ParseJsonValueNoNestingOutput::JsonValue
            }
            b'-' => {
                self.eat_byte();
                tri!(self.parse_positive_number());
                ParseJsonValueNoNestingOutput::JsonValue
            }
            b'0'..=b'9' => {
                tri!(self.parse_positive_number());
                ParseJsonValueNoNestingOutput::JsonValue
            }
            b'"' => {
                self.eat_byte();
                match tri!(self.parse_in_string()) {
                    ParseInString::StringEnded => ParseJsonValueNoNestingOutput::JsonValue,
                    ParseInString::EofInString => ParseJsonValueNoNestingOutput::EofInString,
                }
            }
            b'[' => {
                self.eat_byte();
                ParseJsonValueNoNestingOutput::ArrayStart
            }
            b'{' => {
                self.eat_byte();
                ParseJsonValueNoNestingOutput::ObjectStart
            }
            _ => return Err("expect json value"),
        };

        Ok(out)
    }

    const fn parse_positive_number(&mut self) -> Result<(), &'static str> {
        // int [ frac ] [ exp ]
        match self.next_byte() {
            Some(b'0') => {}
            Some(b'1'..=b'9') => {
                while let Some(b'0'..=b'9') = self.peek() {
                    self.eat_byte();
                }
            }
            _ => return Err("unexpected when parsing positive number"),
        }

        if matches!(self.peek(), Some(b'.')) {
            self.eat_byte();

            let Some(()) = self.parse_one_or_more_digits() else {
                return Err("expect digits after dot");
            };
        }

        if matches!(self.peek(), Some(b'e')) {
            self.eat_byte();

            if let Some(b'-' | b'+') = self.peek() {
                self.eat_byte();
            }

            let Some(()) = self.parse_one_or_more_digits() else {
                return Err("expect digits in exp");
            };
        }

        Ok(())
    }

    const fn parse_one_or_more_digits(&mut self) -> Option<()> {
        let Some(b'0'..=b'9') = self.peek() else {
            return None;
        };
        self.eat_byte();
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.eat_byte();
        }

        Some(())
    }

    const fn eat_byte(&mut self) {
        _ = self.next_byte().expect("unexpected EOF")
    }

    const fn next_byte(&mut self) -> Option<u8> {
        match self.0.split_first() {
            Some((b, rest)) => {
                self.0 = rest;
                Some(*b)
            }
            None => None,
        }
    }

    const fn next_n<const N: usize>(&mut self) -> Option<&'a [u8; N]> {
        match self.0.split_first_chunk::<N>() {
            Some((chunk, rest)) => {
                self.0 = rest;
                Some(chunk)
            }
            None => None,
        }
    }
}

enum ParseInString {
    StringEnded,
    EofInString,
}

enum ParseJsonValueNoNestingOutput {
    JsonValue,
    EofInString,
    ArrayStart,
    ObjectStart,
}

pub(super) enum ParseJsonValueOutput {
    EofAfterArrayStart,
    JsonValue,
    EofInString,
}
