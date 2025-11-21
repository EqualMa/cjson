use core::marker::PhantomData;

pub struct Len<T>(pub T);

pub struct Empty;

impl Len<Empty> {}
impl Empty {}

struct Value<T>(T);

macro_rules! json {
    ($($t:tt)*) => {};
}

const _: () = {
    json!(0);
    json!(1.2);
    json!("hello");

    json!([
        false,
        true,
        0,
        1.2,
        -3,
        "4",
        const { 2 + 3 },
        const { V },
        ..const { Some(7) },
        const {
            if USE_EIGHT {
                Either::Left(8)
            } else {
                Either::Right(0)
            }
        },
    ]);

    const V: u8 = 6;

    const USE_EIGHT: bool = true;
};

enum EitherValue<A, B> {
    A(A),
    B(B),
}

trait HasConstValue {
    type ConstValue;
    const CONST_VALUE: Self::ConstValue;
}

trait JsonStrLen {
    type JsonStrLen<T: ?Sized + HasConstValue<ConstValue = Self>>: HasConstValue<ConstValue = usize>;
}

enum Never {}

impl HasConstValue for Never {
    type ConstValue = usize;

    const CONST_VALUE: Self::ConstValue = 1;
}

impl JsonStrLen for i8 {
    type JsonStrLen<T: ?Sized + HasConstValue<ConstValue = Self>> = Never;
}

impl<A: JsonStrLen + Copy, B: JsonStrLen + Copy> JsonStrLen for EitherValue<A, B> {
    type JsonStrLen<T: ?Sized + HasConstValue<ConstValue = Self>> = EitherJsonStrLen<T, A, B>;
}

struct EitherJsonStrLen<T: ?Sized + HasConstValue<ConstValue = EitherValue<A, B>>, A, B>(
    PhantomData<T>,
    PhantomData<A>,
    PhantomData<B>,
);

impl<
    T: ?Sized + HasConstValue<ConstValue = EitherValue<A, B>>,
    A: JsonStrLen + Copy,
    B: JsonStrLen + Copy,
> HasConstValue for EitherJsonStrLen<T, A, B>
{
    type ConstValue = usize;
    const CONST_VALUE: Self::ConstValue = {
        struct TA<T: ?Sized + HasConstValue<ConstValue = EitherValue<A, B>>, A: JsonStrLen, B: JsonStrLen>(
            EitherJsonStrLen<T, A, B>,
        );
        struct TB<T: ?Sized + HasConstValue<ConstValue = EitherValue<A, B>>, A: JsonStrLen, B: JsonStrLen>(
            EitherJsonStrLen<T, A, B>,
        );

        impl<
            T: ?Sized + HasConstValue<ConstValue = EitherValue<A, B>>,
            A: JsonStrLen + Copy,
            B: JsonStrLen + Copy,
        > HasConstValue for TA<T, A, B>
        {
            type ConstValue = A;

            const CONST_VALUE: Self::ConstValue = match T::CONST_VALUE {
                EitherValue::A(this) => this,
                EitherValue::B(_) => panic!(),
            };
        }

        impl<
            T: ?Sized + HasConstValue<ConstValue = EitherValue<A, B>>,
            A: JsonStrLen + Copy,
            B: JsonStrLen + Copy,
        > HasConstValue for TB<T, A, B>
        {
            type ConstValue = B;

            const CONST_VALUE: Self::ConstValue = match T::CONST_VALUE {
                EitherValue::A(_) => panic!(),
                EitherValue::B(this) => this,
            };
        }

        match T::CONST_VALUE {
            EitherValue::A(_) => <A::JsonStrLen<TA<T, A, B>> as HasConstValue>::CONST_VALUE,
            EitherValue::B(_) => <B::JsonStrLen<TB<T, A, B>> as HasConstValue>::CONST_VALUE,
        }
    };
}

const _: () = {
    type Ty = EitherValue<i8, i8>;
    const V: Ty = Ty::A(0);

    enum T {}
    impl HasConstValue for T {
        type ConstValue = Ty;

        const CONST_VALUE: Self::ConstValue = V;
    }

    let s = <<Ty as JsonStrLen>::JsonStrLen<T> as HasConstValue>::CONST_VALUE;

    assert!(s == 1);
};
