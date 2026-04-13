use super::traits;

mod never;

mod either;

#[derive(Debug, Clone, Copy)]
pub struct Empty;
mod empty;

pub(crate) mod literal_names;

#[derive(Debug, Clone, Copy)]
pub struct Boolean(pub bool);
mod boolean;

/// Json array.
#[derive(Debug, Clone, Copy)]
pub struct Bracketed<Values: traits::EmptyOrCommaSeparatedElements>(pub Values);
mod bracketed;

/// Json object.
#[derive(Debug, Clone, Copy)]
pub struct Braced<Values: traits::Kvs>(pub Values);
mod braced;

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
            /// - `T` MUST satisfy the constraints of this refined type.
            /// - `T` MUST NOT have inner mutability which means `T: core::marker::Freeze`
            ///   because of public api [`Self::inner`].
            pub(crate) const fn new_without_validation(chunks: T) -> Self {
                Self(chunks)
            }

            pub const fn inner(&self) -> &T {
                &self.0
            }

            pub fn into_inner(self) -> T {
                self.0
            }
        }

        impl<T: $Bounds> traits::IntoTextChunks for $Type<T> {
            type IntoTextChunks = T::IntoTextChunks;

            fn into_text_chunks(self) -> Self::IntoTextChunks {
                self.0.into_text_chunks()
            }

            // TODO:
            // fn _private_into_text_chunks_vec(self)
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
    pub struct Text<T: traits::IntoTextChunks>(T);
);

define_refined_type!(
    #[assert_refined(NonEmptyCommaSeparatedElements)]
    #[derive(Debug, Clone, Copy)]
    pub struct NonEmptyCommaSeparatedItems<T: traits::IntoTextChunks>(T);
);

mod non_empty_comma_separated_items;

define_refined_type!(
    #[assert_refined(Text, Value)]
    /// Json number.
    #[derive(Debug, Clone, Copy)]
    pub struct Value<T: traits::IntoTextChunks>(T);
);

mod value_const;

define_refined_type!(
    #[assert_refined(Text, Value)]
    /// Json number.
    #[derive(Debug, Clone, Copy)]
    pub struct Number<T: traits::IntoTextChunks>(T);
);

mod number_const;

define_refined_type!(
    #[assert_refined(Text, Value)]
    /// Json string.
    #[derive(Debug, Clone, Copy)]
    pub struct JsonString<T: traits::IntoTextChunks>(T);
);

define_refined_type!(
    #[assert_refined(JsonStringFragment)]
    /// Json string fragment.
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
pub struct Comma;
mod comma;

define_refined_type!(
    #[assert_refined(NonEmptyKvs)]
    #[derive(Debug, Clone, Copy)]
    pub struct NonEmptyKvs<T: traits::IntoTextChunks>(T);
);

mod non_empty_kvs;
