use Faker;
use faker::Boolean;
use faker::Lorem;
use faker::Number;

use std::rc::Rc;
use std::sync::Arc;

pub trait Dummy {
    fn dummy() -> Self;
}

macro_rules! impl_number {
    ($t: ident) => {
        impl_number!($t, $t);
    };
    ($t: ident, $m: ident) => {
        impl Dummy for $t {
            fn dummy() -> Self {
                <Faker as Number>::between($m::min_value() as $t, $m::max_value() as $t)
            }
        }
    };
}

impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(isize);
impl_number!(usize);
impl_number!(f32, i32);
impl_number!(f64, i64);

impl Dummy for bool {
    fn dummy() -> Self {
        <Faker as Boolean>::boolean()
    }
}

impl Dummy for String {
    fn dummy() -> Self {
        <Faker as Lorem>::sentence(3, 2)
    }
}

impl<'a> Dummy for &'a str {
    fn dummy() -> Self {
        <Faker as Lorem>::word()
    }
}

impl<T> Dummy for Vec<T>
where
    T: Dummy,
{
    fn dummy() -> Self {
        let size = <Faker as Number>::between(0, 5);
        (0..size).map(|_| T::dummy()).collect()
    }
}

impl<T> Dummy for Option<T>
where
    T: Dummy,
{
    fn dummy() -> Self {
        match <Faker as Boolean>::boolean() {
            true => Some(T::dummy()),
            false => None,
        }
    }
}

impl<T> Dummy for Box<T>
where
    T: Dummy,
{
    fn dummy() -> Self {
        Box::new(T::dummy())
    }
}

impl<T> Dummy for Rc<T>
where
    T: Dummy,
{
    fn dummy() -> Self {
        Rc::new(T::dummy())
    }
}

impl<T> Dummy for Arc<T>
where
    T: Dummy,
{
    fn dummy() -> Self {
        Arc::new(T::dummy())
    }
}
