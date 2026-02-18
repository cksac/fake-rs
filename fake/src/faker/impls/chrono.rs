use crate::faker::chrono::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake, Faker};
use chrono::{TimeZone, Utc};
use rand::RngExt;

const MINUTES_MAX_BOUND: i64 = 1_000_000;

impl<L: Data> Dummy<Time<L>> for chrono::NaiveTime {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Time<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<Time<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Time<L>, rng: &mut R) -> Self {
        let time: chrono::NaiveTime = Faker.fake_with_rng(rng);
        time.format(L::CHRONO_DEFAULT_TIME_FORMAT).to_string()
    }
}

impl<L: Data> Dummy<Date<L>> for chrono::NaiveDate {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Date<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<Date<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Date<L>, rng: &mut R) -> Self {
        let date: chrono::NaiveDate = Faker.fake_with_rng(rng);
        date.format(L::CHRONO_DEFAULT_DATE_FORMAT).to_string()
    }
}

impl<L: Data> Dummy<DateTime<L>> for chrono::NaiveDateTime {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data, Tz: TimeZone + Dummy<Faker>> Dummy<DateTime<L>> for chrono::DateTime<Tz> {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTime<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        let datetime: chrono::DateTime<Utc> = Faker.fake_with_rng(rng);
        datetime
            .format(L::CHRONO_DEFAULT_DATETIME_FORMAT)
            .to_string()
    }
}

impl<L: Data> Dummy<Duration<L>> for chrono::Duration {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Duration<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTimeBefore<L>> for chrono::DateTime<Utc> {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBefore<L>, rng: &mut R) -> Self {
        let mins: i64 = (1..MINUTES_MAX_BOUND).fake_with_rng(rng);
        let duration = chrono::Duration::minutes(mins);
        c.1 - duration
    }
}

impl<L: Data> Dummy<DateTimeBefore<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBefore<L>, rng: &mut R) -> Self {
        let datetime: chrono::DateTime<Utc> = c.fake_with_rng(rng);
        datetime
            .format(L::CHRONO_DEFAULT_DATETIME_FORMAT)
            .to_string()
    }
}

impl<L: Data> Dummy<DateTimeAfter<L>> for chrono::DateTime<Utc> {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeAfter<L>, rng: &mut R) -> Self {
        let mins: i64 = (1..MINUTES_MAX_BOUND).fake_with_rng(rng);
        let duration = chrono::Duration::minutes(mins);
        c.1 + duration
    }
}

impl<L: Data> Dummy<DateTimeAfter<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeAfter<L>, rng: &mut R) -> Self {
        let datetime: chrono::DateTime<Utc> = c.fake_with_rng(rng);
        datetime
            .format(L::CHRONO_DEFAULT_DATETIME_FORMAT)
            .to_string()
    }
}

impl<L: Data> Dummy<DateTimeBetween<L>> for chrono::DateTime<Utc> {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBetween<L>, rng: &mut R) -> Self {
        let diff = c.2 - c.1;
        let max_minutes = diff.num_minutes();

        let from_start: i64 = (0..max_minutes).fake_with_rng(rng);
        let duration = chrono::Duration::minutes(from_start);
        c.1 + duration
    }
}

impl<L: Data> Dummy<DateTimeBetween<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBetween<L>, rng: &mut R) -> Self {
        let datetime: chrono::DateTime<Utc> = c.fake_with_rng(rng);
        datetime
            .format(L::CHRONO_DEFAULT_DATETIME_FORMAT)
            .to_string()
    }
}
