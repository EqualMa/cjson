use super::traits;

mod literal_names;

/// Json array.
#[derive(Debug, Clone, Copy)]
pub struct Array<Values: traits::EmptyOrCommaSeparatedElements>(pub Values);
mod array;

/// Json array.
#[derive(Debug, Clone, Copy)]
pub struct ArrayOfIter<I: Iterator<Item: traits::Text>>(pub I);
mod array_of_iter;

macro_rules! define_refined_type {
    (
        #[assert_refined($($Trait:ident),*)]
        $(#$attr:tt)*
        $vis:vis $struct:ident $Type:ident<$T:ident: $Bounds:path> $body:tt ;
    ) => {
        $(#$attr)*
        $vis $struct $Type<$T: $Bounds> $body;

        impl<T: $Bounds> $Type<T> {
            pub(crate) fn new_without_validation(chunks: T) -> Self {
                Self(chunks)
            }
        }

        impl<T: $Bounds> traits::IntoTextChunks for $Type<T> {
            type IntoTextChunks = T::IntoTextChunks;

            fn into_text_chunks(self) -> Self::IntoTextChunks {
                self.0.into_text_chunks()
            }
        }

        $(
            impl<T: $Bounds> traits::sealed::$Trait for $Type<T> {}
            impl<T: $Bounds> traits::$Trait for $Type<T> {}
        )*
    };
}

define_refined_type!(
    #[assert_refined(Text, Value)]
    /// Json number.
    #[derive(Debug, Clone, Copy)]
    pub struct Number<T: traits::IntoTextChunks>(T);
);

define_refined_type!(
    #[assert_refined(Text, Value)]
    /// Json string.
    #[derive(Debug, Clone, Copy)]
    pub struct JsonString<T: traits::IntoTextChunks>(T);
);

define_refined_type!(
    #[assert_refined(JsonStringFragment)]
    /// Json string.
    #[derive(Debug, Clone, Copy)]
    pub struct JsonStringFragment<T: traits::IntoTextChunks>(T);
);

#[derive(Debug, Clone, Copy)]
pub struct StrToJsonStringFragment<'a>(pub &'a str);
mod str_to_json_string_fragment;

#[derive(Debug, Clone, Copy)]
pub struct QuotedJsonStringFragment<T: traits::JsonStringFragment>(pub T);
mod quoted_json_string_fragment;

#[derive(Debug, Clone, Copy)]
pub struct Chain<A, B>(pub A, pub B);
mod chain;

#[derive(Debug, Clone, Copy)]
pub struct CommaSeparated<A, B>(pub A, pub B);
mod comma_separated;
