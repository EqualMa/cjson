use crate::{
    r#const::{JsonStringFragmentAsStr, LastChunkOfJsonStringAsStr},
    ser::{IntoJson, json_kinds, traits::ConsumeTextChunk},
};

use super::{ConsumeJson, Consumed};

pub trait ConsumeFragmentInString {
    fn consume_fragment_as_str(
        &mut self,
        w: &mut impl ConsumeTextChunk,
        v: JsonStringFragmentAsStr<'_>,
    );
    fn consume_fragment(
        &mut self,
        w: &mut impl ConsumeTextChunk,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    );
}

pub trait ConsumeFragmentInStringNormally {}

impl<T: ?Sized + ConsumeFragmentInStringNormally> ConsumeFragmentInString for T {
    fn consume_fragment_as_str(
        &mut self,
        w: &mut impl ConsumeTextChunk,
        v: JsonStringFragmentAsStr<'_>,
    ) {
        w.consume_text_chunk(v.as_str())
    }
    fn consume_fragment(
        &mut self,
        w: &mut impl ConsumeTextChunk,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) {
        let Consumed { .. } = v.json_provide_into(super::consume_content::ConsumeStringFragment(
            w.as_mut_consume_text_chunk(),
        ));
    }
}

pub trait EndJsonString: ConsumeFragmentInString {
    fn end_with_last_chunk(
        //
        self,
        w: impl ConsumeTextChunk,
        v: LastChunkOfJsonStringAsStr<'_>,
    );
    fn end_with(
        self,
        w: impl ConsumeTextChunk,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    );
}

pub(crate) trait HasConstDefault {
    const DEFAULT: Self;
}

pub struct ConsumeInJsonString<E: EndJsonString, InitialConsumer: ?Sized + ConsumeJson> {
    end: E,
    writer: InitialConsumer::Writer,
}

impl<E: EndJsonString, InitialConsumer: ?Sized + ConsumeJson>
    ConsumeInJsonString<E, InitialConsumer>
{
    pub(super) const fn new_full(end: E, writer: InitialConsumer::Writer) -> Self {
        Self { end, writer }
    }
    pub(super) const fn new(writer: InitialConsumer::Writer) -> Self
    where
        E: HasConstDefault,
    {
        Self::new_full(E::DEFAULT, writer)
    }
}

impl<E: EndJsonString, InitialConsumer: ?Sized + ConsumeJson>
    ConsumeInJsonString<E, InitialConsumer>
{
    pub fn consume_fragment_as_str(&mut self, v: JsonStringFragmentAsStr<'_>) {
        self.end.consume_fragment_as_str(&mut self.writer, v)
    }
    pub fn consume_fragment(&mut self, v: impl IntoJson<JsonKind = json_kinds::JsonString>) {
        self.end.consume_fragment(&mut self.writer, v)
    }

    pub fn end_with_last_chunk(
        self,
        v: LastChunkOfJsonStringAsStr<'_>,
    ) -> Consumed<json_kinds::JsonString, InitialConsumer> {
        self.end.end_with_last_chunk(self.writer, v);
        Consumed::ASSERT_STRING
    }
    pub fn end_with(
        self,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, InitialConsumer> {
        self.end.end_with(self.writer, v);
        Consumed::ASSERT_STRING
    }
}

pub enum NeverEndJsonString {}
pub struct EndJsonStringWithClose;
pub struct EndJsonStringWithNothing;

impl ConsumeFragmentInString for NeverEndJsonString {
    fn consume_fragment_as_str(
        &mut self,
        _: &mut impl ConsumeTextChunk,
        _: JsonStringFragmentAsStr<'_>,
    ) {
        match *self {}
    }

    fn consume_fragment(
        &mut self,
        _: &mut impl ConsumeTextChunk,
        _: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) {
        match *self {}
    }
}
impl EndJsonString for NeverEndJsonString {
    fn end_with_last_chunk(
        //
        self,
        _: impl ConsumeTextChunk,
        _: LastChunkOfJsonStringAsStr<'_>,
    ) {
        match self {}
    }

    fn end_with(
        self,
        _: impl ConsumeTextChunk,
        _: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) {
        match self {}
    }
}

impl ConsumeFragmentInStringNormally for EndJsonStringWithClose {}
impl EndJsonString for EndJsonStringWithClose {
    fn end_with_last_chunk(
        //
        self,
        mut w: impl ConsumeTextChunk,
        v: LastChunkOfJsonStringAsStr<'_>,
    ) {
        w.consume_text_chunk(v.as_str())
    }

    fn end_with(
        self,
        w: impl ConsumeTextChunk,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) {
        let Consumed { .. } =
            v.json_provide_into(super::consume_content_close::ConsumeStringFragmentClose(w));
    }
}

impl ConsumeFragmentInStringNormally for EndJsonStringWithNothing {}
impl EndJsonString for EndJsonStringWithNothing {
    fn end_with_last_chunk(
        //
        self,
        mut w: impl ConsumeTextChunk,
        v: LastChunkOfJsonStringAsStr<'_>,
    ) {
        w.consume_text_chunk(v.fragment())
    }

    fn end_with(
        self,
        w: impl ConsumeTextChunk,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) {
        let Consumed { .. } = v.json_provide_into(super::consume_content::ConsumeStringFragment(w));
    }
}

impl HasConstDefault for EndJsonStringWithClose {
    const DEFAULT: Self = Self;
}

impl HasConstDefault for EndJsonStringWithNothing {
    const DEFAULT: Self = Self;
}

pub struct EndJsonStringOpenFragmentIfNotEmpty<'a> {
    pub(crate) started: &'a mut bool,
}

impl ConsumeFragmentInString for EndJsonStringOpenFragmentIfNotEmpty<'_> {
    fn consume_fragment_as_str(
        &mut self,
        w: &mut impl ConsumeTextChunk,
        v: JsonStringFragmentAsStr<'_>,
    ) {
        let Some(non_empty_fragment) = v.non_empty_fragment() else {
            return;
        };
        if *self.started {
            w.consume_text_chunk(non_empty_fragment)
        } else {
            *self.started = true;
            w.consume_2_text_chunks("\"", non_empty_fragment);
        }
    }

    fn consume_fragment(
        &mut self,
        w: &mut impl ConsumeTextChunk,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) {
        EndJsonStringOpenFragmentIfNotEmpty {
            started: self.started,
        }
        .end_with(w.as_mut_consume_text_chunk(), v)
    }
}
impl<'a> EndJsonString for EndJsonStringOpenFragmentIfNotEmpty<'a> {
    fn end_with_last_chunk(
        //
        self,
        mut w: impl ConsumeTextChunk,
        v: LastChunkOfJsonStringAsStr<'_>,
    ) {
        let Some(non_empty_fragment) = v.non_empty_fragment() else {
            return;
        };

        if *self.started {
            w.consume_text_chunk(non_empty_fragment)
        } else {
            *self.started = true;
            w.consume_2_text_chunks("\"", non_empty_fragment);
        }
    }

    fn end_with(
        self,
        w: impl ConsumeTextChunk,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) {
        if *self.started {
            let Consumed { .. } =
                v.json_provide_into(super::consume_content::ConsumeStringFragment(w));
        } else {
            let Consumed { .. } = v.json_provide_into(
                super::consume_open_content::ConsumeStringOpenFragmentIfNotEmpty::new(
                    w,
                    self.started,
                ),
            );
        }
    }
}
