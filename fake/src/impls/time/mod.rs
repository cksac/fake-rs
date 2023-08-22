use crate::{Dummy, Fake, Faker};
use rand::Rng;
use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time};

const YEAR_MAG: i32 = 3_000i32;
const MIN_NANOS: i128 = -377_705_116_800_000_000_000;
const MAX_NANOS: i128 = 253_402_300_799_000_000_000;

fn is_leap(year: i32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else {
        year % 4 == 0
    }
}

impl Dummy<Faker> for Duration {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Duration::nanoseconds(Faker.fake_with_rng(rng))
    }
}

impl Dummy<Faker> for OffsetDateTime {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let nanos = (MIN_NANOS..MAX_NANOS).fake_with_rng(rng);
        OffsetDateTime::from_unix_timestamp_nanos(nanos).unwrap()
    }
}

impl Dummy<Faker> for Date {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let year: i32 = (0..YEAR_MAG).fake_with_rng(rng);
        let end = if is_leap(year) { 366 } else { 365 };
        let day_ord: u16 = (1..end).fake_with_rng(rng);
        Date::from_ordinal_date(year, day_ord).unwrap()
    }
}

impl Dummy<Faker> for Time {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let hour = (0..24).fake_with_rng(rng);
        let min = (0..60).fake_with_rng(rng);
        let sec = (0..60).fake_with_rng(rng);
        Time::from_hms(hour, min, sec).unwrap()
    }
}

impl Dummy<Faker> for PrimitiveDateTime {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let date = Faker.fake_with_rng(rng);
        let time = Faker.fake_with_rng(rng);
        PrimitiveDateTime::new(date, time)
    }
}

pub struct Precision<const N: usize>;

trait AllowedPrecision {
    const SCALE: i128;

    fn to_scale(nanos: i128) -> i128 {
        if nanos != 0 {
            nanos / Self::SCALE * Self::SCALE
        } else {
            nanos
        }
    }
}
macro_rules! allow_precision {
    ($($precision:expr),*) => {
        $(impl AllowedPrecision for Precision<$precision> {
            const SCALE: i128 = 10i128.pow(9 - $precision);
        })*
    };
}
allow_precision!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

impl<const N: usize> Dummy<Precision<N>> for Time
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Precision<N>, rng: &mut R) -> Self {
        let hour = (0..24).fake_with_rng(rng);
        let min = (0..60).fake_with_rng(rng);
        let sec = (0..60).fake_with_rng(rng);
        let nanos: i128 = (0..1_000_000_000).fake_with_rng(rng);
        let nanos = Precision::<N>::to_scale(nanos) as u32;
        Time::from_hms_nano(hour, min, sec, nanos).expect("failed to create time")
    }
}

impl<const N: usize> Dummy<Precision<N>> for PrimitiveDateTime
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Precision<N>, rng: &mut R) -> Self {
        let date = Faker.fake_with_rng(rng);
        let time = Precision::<N>.fake_with_rng(rng);
        PrimitiveDateTime::new(date, time)
    }
}

impl<const N: usize> Dummy<Precision<N>> for OffsetDateTime
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Precision<N>, rng: &mut R) -> Self {
        let nanos = (MIN_NANOS..MAX_NANOS).fake_with_rng(rng);
        let nanos = Precision::<N>::to_scale(nanos);
        OffsetDateTime::from_unix_timestamp_nanos(nanos).unwrap()
    }
}
