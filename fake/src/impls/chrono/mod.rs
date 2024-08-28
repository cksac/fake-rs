#![allow(deprecated)]

use crate::{Dummy, Fake, Faker};
use chrono::{
    Date, DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use rand::Rng;

const YEAR_MAG: i32 = 3_000i32;

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

impl Dummy<Faker> for Utc {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Utc
    }
}

impl Dummy<Faker> for FixedOffset {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        if rng.gen_bool(0.5) {
            let halves: i32 = (0..=28).fake_with_rng(rng);
            FixedOffset::east_opt(halves * 30 * 60).expect("failed to create FixedOffset")
        } else {
            let halves: i32 = (0..=24).fake_with_rng(rng);
            FixedOffset::west_opt(halves * 30 * 60).expect("failed to create FixedOffset")
        }
    }
}

impl<Tz> Dummy<Faker> for DateTime<Tz>
where
    Tz: TimeZone + Dummy<Faker>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let utc: DateTime<Utc> = Utc.timestamp_nanos(Faker.fake_with_rng(rng));
        let tz: Tz = Faker.fake_with_rng(rng);
        utc.with_timezone(&tz)
    }
}

impl Dummy<Faker> for Date<Utc> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let year: i32 = (1..YEAR_MAG).fake_with_rng(rng);
        let end = if is_leap(year) { 366 } else { 365 };
        let day_ord: u32 = (1..end).fake_with_rng(rng);
        Utc.yo(year, day_ord)
    }
}

impl Dummy<Faker> for NaiveTime {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let hour = (0..24).fake_with_rng(rng);
        let min = (0..60).fake_with_rng(rng);
        let sec = (0..60).fake_with_rng(rng);
        NaiveTime::from_hms(hour, min, sec)
    }
}

impl Dummy<Faker> for NaiveDate {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let year: i32 = (1..YEAR_MAG).fake_with_rng(rng);
        let end = if is_leap(year) { 366 } else { 365 };
        let day_ord: u32 = (1..end).fake_with_rng(rng);
        NaiveDate::from_yo(year, day_ord)
    }
}

impl Dummy<Faker> for NaiveDateTime {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let date = Faker.fake_with_rng(rng);
        let time = Faker.fake_with_rng(rng);
        NaiveDateTime::new(date, time)
    }
}

pub struct Precision<const N: usize>;

trait AllowedPrecision {
    const SCALE: i64;

    fn to_scale(nanos: i64) -> i64 {
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
            const SCALE: i64 = 10i64.pow(9 - $precision);
        })*
    };
}
allow_precision!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

impl<const N: usize> Dummy<Precision<N>> for NaiveTime
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Precision<N>, rng: &mut R) -> Self {
        let hour = (0..24).fake_with_rng(rng);
        let min = (0..60).fake_with_rng(rng);
        let sec = (0..60).fake_with_rng(rng);
        let nanos: i64 = (0..1_000_000_000).fake_with_rng(rng);
        let nanos = Precision::<N>::to_scale(nanos) as u32;
        NaiveTime::from_hms_nano_opt(hour, min, sec, nanos).expect("failed to create time")
    }
}

impl<Tz, const N: usize> Dummy<Precision<N>> for DateTime<Tz>
where
    Tz: TimeZone + Dummy<Faker>,
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Precision<N>, rng: &mut R) -> Self {
        let nanos: i64 = Faker.fake_with_rng(rng);
        let utc: DateTime<Utc> = Utc.timestamp_nanos(Precision::<N>::to_scale(nanos));
        let tz: Tz = Faker.fake_with_rng(rng);
        utc.with_timezone(&tz)
    }
}
