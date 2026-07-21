use core::convert::Infallible;

use crate::r#const::{
    ContentfulFirstChunkOfArrayAsStr, ContentfulFirstChunkOfObjectAsStr,
    ContentfulLastChunkOfArrayAsStr, ContentfulLastChunkOfObjectAsStr, HasConstState,
    NonEmptyArrayAsStr, NonEmptyObjectAsStr,
};

pub struct AnyValue;
pub struct JsonString;
pub struct Array;
pub struct Object;

pub trait YesOrNo {}

impl YesOrNo for () {}
impl YesOrNo for Infallible {}

pub trait JsonKind: Sized {
    fn into_kind_of_any_value(self) -> AnyValue {
        AnyValue
    }

    type Union<Other: JsonKind>;
    fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other>;

    type UnionString;
    fn union_string(self, other: JsonString) -> Self::UnionString;

    type UnionArray;
    fn union_array(self, other: Array) -> Self::UnionArray;

    type UnionObject;
    fn union_object(self, other: Object) -> Self::UnionObject;

    type Contains<Other: JsonKind>: YesOrNo;

    type StringContainsSelf: YesOrNo;
    type ArrayContainsSelf: YesOrNo;
    type ObjectContainsSelf: YesOrNo;

    type ArrayOrObjectContainsSelf: YesOrNo;
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

    type Contains<Other: JsonKind> = ();

    type StringContainsSelf = Infallible;
    type ArrayContainsSelf = Infallible;
    type ObjectContainsSelf = Infallible;
    type ArrayOrObjectContainsSelf = Infallible;
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

    type Contains<Other: JsonKind> = Other::StringContainsSelf;

    type StringContainsSelf = ();
    type ArrayContainsSelf = Infallible;
    type ObjectContainsSelf = Infallible;
    type ArrayOrObjectContainsSelf = Infallible;
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

    type Contains<Other: JsonKind> = Other::ArrayContainsSelf;

    type StringContainsSelf = Infallible;
    type ArrayContainsSelf = ();
    type ObjectContainsSelf = Infallible;
    type ArrayOrObjectContainsSelf = ();
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

    type Contains<Other: JsonKind> = Other::ObjectContainsSelf;

    type StringContainsSelf = Infallible;
    type ArrayContainsSelf = Infallible;
    type ObjectContainsSelf = ();
    type ArrayOrObjectContainsSelf = ();
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
