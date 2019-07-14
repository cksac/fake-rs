use crate::{Dummy, Fake, Faker};
use chrono::{Date, DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use rand::seq::SliceRandom;
use rand::Rng;

const YEAR_MAG: i32 = 3_000i32;

fn is_leap(year: i32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

impl Dummy<Faker> for Duration {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Duration::nanoseconds(Faker.fake_with_rng(rng))
    }
}

impl Dummy<Faker> for DateTime<Utc> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Utc.timestamp_nanos(Faker.fake_with_rng(rng))
    }
}

impl Dummy<Faker> for Date<Utc> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let year: i32 = (0..YEAR_MAG).fake_with_rng(rng);
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
        let year: i32 = (0..YEAR_MAG).fake_with_rng(rng);
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
