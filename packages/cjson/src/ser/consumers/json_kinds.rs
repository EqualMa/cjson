use core::convert::Infallible;

use crate::r#const::{
    ContentfulFirstChunkOfArrayAsStr, ContentfulFirstChunkOfObjectAsStr,
    ContentfulLastChunkOfArrayAsStr, ContentfulLastChunkOfObjectAsStr, HasConstState,
    NonEmptyArrayAsStr, NonEmptyObjectAsStr,
};

use super::{help_ancestor::HelpAncestorToXConsumeChild, yes_or_no::YesOrNo};

pub(crate) use self::json_kind_contains::JsonKindContains;

mod json_kind_contains;

pub struct AnyValue;
pub struct JsonString;
pub struct Array;
pub struct Object;

pub trait JsonKind: Sized + JsonKindContains<Contains<Self> = ()> {
    type Union<Other: JsonKind>: JsonKind<Contains<Self> = (), Contains<Other> = ()>
        + HelpAncestorToXConsumeChild<Self>
        + HelpAncestorToXConsumeChild<Other>;
    fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other>;

    type UnionString: JsonKind<Contains<Self> = (), Contains<JsonString> = ()>
        + HelpAncestorToXConsumeChild<Self>
        + HelpAncestorToXConsumeChild<JsonString>;
    fn union_string(self, other: JsonString) -> Self::UnionString;

    type UnionArray: JsonKind<Contains<Self> = (), Contains<Array> = ()>
        + HelpAncestorToXConsumeChild<Self>
        + HelpAncestorToXConsumeChild<Array>;
    fn union_array(self, other: Array) -> Self::UnionArray;

    type UnionObject: JsonKind<Contains<Self> = (), Contains<Object> = ()>
        + HelpAncestorToXConsumeChild<Self>
        + HelpAncestorToXConsumeChild<Object>;
    fn union_object(self, other: Object) -> Self::UnionObject;

    type ArrayOrObjectContainsSelf: YesOrNo;

    fn upcast<A: JsonKind<Contains<Self> = ()>>(self) -> A;

    fn from_upcast_any_value(any_value: AnyValue, yes: Self::Contains<AnyValue>) -> Self;
    fn from_upcast_string(string: JsonString, yes: Self::Contains<JsonString>) -> Self;
    fn from_upcast_array(array: Array, yes: Self::Contains<Array>) -> Self;
    fn from_upcast_object(object: Object, yes: Self::Contains<Object>) -> Self;
}

impl JsonKindContains for AnyValue {
    type Contains<Other: JsonKind> = ();

    type StringContainsSelf = Infallible;
    type ArrayContainsSelf = Infallible;
    type ObjectContainsSelf = Infallible;
}

impl JsonKind for AnyValue {
    type Union<Other: JsonKind> = Self;

    fn union<Other: JsonKind>(self, _: Other) -> Self::Union<Other> {
        self
    }

    type UnionString = Self;

    fn union_string(self, _: JsonString) -> Self::UnionString {
        self
    }

    type UnionArray = Self;

    fn union_array(self, _: Array) -> Self::UnionArray {
        self
    }

    type UnionObject = Self;

    fn union_object(self, _: Object) -> Self::UnionObject {
        self
    }

    type ArrayOrObjectContainsSelf = Infallible;

    fn upcast<A: JsonKind<Contains<Self> = ()>>(self) -> A {
        A::from_upcast_any_value(self, ())
    }

    fn from_upcast_any_value(any_value: AnyValue, (): Self::Contains<AnyValue>) -> Self {
        any_value
    }

    fn from_upcast_string(JsonString: JsonString, (): Self::Contains<JsonString>) -> Self {
        Self
    }

    fn from_upcast_array(Array: Array, (): Self::Contains<Array>) -> Self {
        Self
    }

    fn from_upcast_object(Object: Object, (): Self::Contains<Object>) -> Self {
        Self
    }
}

impl JsonKindContains for JsonString {
    type Contains<Other: JsonKind> = Other::StringContainsSelf;

    type StringContainsSelf = ();
    type ArrayContainsSelf = Infallible;
    type ObjectContainsSelf = Infallible;
}
impl JsonKind for JsonString {
    type Union<Other: JsonKind> = Other::UnionString;

    fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other> {
        other.union_string(self)
    }

    type UnionString = Self;

    fn union_string(self, _: JsonString) -> Self::UnionString {
        self
    }

    type UnionArray = AnyValue;

    fn union_array(self, Array: Array) -> Self::UnionArray {
        AnyValue
    }

    type UnionObject = AnyValue;

    fn union_object(self, Object: Object) -> Self::UnionObject {
        AnyValue
    }

    type ArrayOrObjectContainsSelf = Infallible;

    fn upcast<A: JsonKind<Contains<Self> = ()>>(self) -> A {
        A::from_upcast_string(self, ())
    }

    fn from_upcast_any_value(_: AnyValue, yes: Self::Contains<AnyValue>) -> Self {
        match yes {}
    }

    fn from_upcast_string(string: JsonString, (): Self::Contains<JsonString>) -> Self {
        string
    }

    fn from_upcast_array(_: Array, yes: Self::Contains<Array>) -> Self {
        match yes {}
    }

    fn from_upcast_object(_: Object, yes: Self::Contains<Object>) -> Self {
        match yes {}
    }
}

impl JsonKindContains for Array {
    type Contains<Other: JsonKind> = Other::ArrayContainsSelf;

    type StringContainsSelf = Infallible;
    type ArrayContainsSelf = ();
    type ObjectContainsSelf = Infallible;
}
impl JsonKind for Array {
    type Union<Other: JsonKind> = Other::UnionArray;

    fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other> {
        other.union_array(self)
    }

    type UnionString = AnyValue;

    fn union_string(self, JsonString: JsonString) -> Self::UnionString {
        AnyValue
    }

    type UnionArray = Array;

    fn union_array(self, Array: Array) -> Self::UnionArray {
        Array
    }

    type UnionObject = AnyValue;

    fn union_object(self, Object: Object) -> Self::UnionObject {
        AnyValue
    }

    type ArrayOrObjectContainsSelf = ();

    fn upcast<A: JsonKind<Contains<Self> = ()>>(self) -> A {
        A::from_upcast_array(self, ())
    }

    fn from_upcast_any_value(_: AnyValue, yes: Self::Contains<AnyValue>) -> Self {
        match yes {}
    }

    fn from_upcast_string(_: JsonString, yes: Self::Contains<JsonString>) -> Self {
        match yes {}
    }

    fn from_upcast_array(array: Array, (): Self::Contains<Array>) -> Self {
        array
    }

    fn from_upcast_object(_: Object, yes: Self::Contains<Object>) -> Self {
        match yes {}
    }
}

impl JsonKindContains for Object {
    type Contains<Other: JsonKind> = Other::ObjectContainsSelf;

    type StringContainsSelf = Infallible;
    type ArrayContainsSelf = Infallible;
    type ObjectContainsSelf = ();
}
impl JsonKind for Object {
    type Union<Other: JsonKind> = Other::UnionObject;

    fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other> {
        other.union_object(self)
    }

    type UnionString = AnyValue;

    fn union_string(self, JsonString: JsonString) -> Self::UnionString {
        AnyValue
    }

    type UnionArray = AnyValue;

    fn union_array(self, Array: Array) -> Self::UnionArray {
        AnyValue
    }

    type UnionObject = Self;

    fn union_object(self, Object: Object) -> Self::UnionObject {
        self
    }

    type ArrayOrObjectContainsSelf = ();

    fn upcast<A: JsonKind<Contains<Self> = ()>>(self) -> A {
        A::from_upcast_object(self, ())
    }

    fn from_upcast_any_value(_: AnyValue, yes: Self::Contains<AnyValue>) -> Self {
        match yes {}
    }

    fn from_upcast_string(_: JsonString, yes: Self::Contains<JsonString>) -> Self {
        match yes {}
    }

    fn from_upcast_array(_: Array, yes: Self::Contains<Array>) -> Self {
        match yes {}
    }

    fn from_upcast_object(object: Object, (): Self::Contains<Object>) -> Self {
        object
    }
}

pub trait ArrayOrObject: 'static + JsonKind {
    type ContentfulFirstChunkAsStr<'a, Next: ?Sized + HasConstState>;
    type ContentfulLastChunkAsStr<'a, Prev: ?Sized + HasConstState>;
    type ContentfulFullChunkAsAtr<'a>;
}

impl ArrayOrObject for Array {
    type ContentfulFirstChunkAsStr<'a, Next: ?Sized + HasConstState> =
        ContentfulFirstChunkOfArrayAsStr<'a, Next>;
    type ContentfulLastChunkAsStr<'a, Prev: ?Sized + HasConstState> =
        ContentfulLastChunkOfArrayAsStr<'a, Prev>;
    type ContentfulFullChunkAsAtr<'a> = NonEmptyArrayAsStr<'a>;
}

impl ArrayOrObject for Object {
    type ContentfulFirstChunkAsStr<'a, Next: ?Sized + HasConstState> =
        ContentfulFirstChunkOfObjectAsStr<'a, Next>;
    type ContentfulLastChunkAsStr<'a, Prev: ?Sized + HasConstState> =
        ContentfulLastChunkOfObjectAsStr<'a, Prev>;
    type ContentfulFullChunkAsAtr<'a> = NonEmptyObjectAsStr<'a>;
}

pub trait ChainableJsonKind: JsonKind {}

impl ChainableJsonKind for JsonString {}
impl ChainableJsonKind for Array {}
impl ChainableJsonKind for Object {}
