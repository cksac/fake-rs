use crate::faker::time::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake, Faker};
use rand::Rng;

const MINUTES_MAX_BOUND: i64 = 1_000_000;

impl<L: Data> Dummy<Time<L>> for time::Time {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Time<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<Time<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Time<L>, rng: &mut R) -> Self {
        let time: time::Time = Faker.fake_with_rng(rng);
        time.format(&time::format_description::parse(L::TIME_DEFAULT_TIME_FORMAT).unwrap())
            .unwrap()
    }
}

impl<L: Data> Dummy<Date<L>> for time::Date {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Date<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<Date<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Date<L>, rng: &mut R) -> Self {
        let date: time::Date = Faker.fake_with_rng(rng);
        date.format(&time::format_description::parse(L::TIME_DEFAULT_DATE_FORMAT).unwrap())
            .unwrap()
    }
}

impl<L: Data> Dummy<DateTime<L>> for time::PrimitiveDateTime {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTime<L>> for time::OffsetDateTime {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTime<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        let datetime: time::OffsetDateTime = Faker.fake_with_rng(rng);
        datetime
            .format(&time::format_description::parse(L::TIME_DEFAULT_DATETIME_FORMAT).unwrap())
            .unwrap()
    }
}

impl<L: Data> Dummy<Duration<L>> for time::Duration {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Duration<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTimeBefore<L>> for time::OffsetDateTime {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &DateTimeBefore<L>, rng: &mut R) -> Self {
        let mins: i64 = (1..MINUTES_MAX_BOUND).fake_with_rng(rng);
        let duration = time::Duration::minutes(mins);
        c.1 - duration
    }
}

impl<L: Data> Dummy<DateTimeBefore<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &DateTimeBefore<L>, rng: &mut R) -> Self {
        let datetime: time::OffsetDateTime = c.fake_with_rng(rng);
        datetime
            .format(&time::format_description::parse(L::TIME_DEFAULT_DATETIME_FORMAT).unwrap())
            .unwrap()
    }
}

impl<L: Data> Dummy<DateTimeAfter<L>> for time::OffsetDateTime {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &DateTimeAfter<L>, rng: &mut R) -> Self {
        let mins: i64 = (1..MINUTES_MAX_BOUND).fake_with_rng(rng);
        let duration = time::Duration::minutes(mins);
        c.1 + duration
    }
}

impl<L: Data> Dummy<DateTimeAfter<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(c: &DateTimeAfter<L>, rng: &mut R) -> Self {
        let datetime: time::OffsetDateTime = c.fake_with_rng(rng);
        datetime
            .format(&time::format_description::parse(L::TIME_DEFAULT_DATETIME_FORMAT).unwrap())
            .unwrap()
    }
}

impl<L: Data> Dummy<DateTimeBetween<L>> for time::OffsetDateTime {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(c: &DateTimeBetween<L>, rng: &mut R) -> Self {
        let diff = c.2 - c.1;
        let max_minutes = diff.whole_minutes();

        let from_start: i64 = (0..max_minutes).fake_with_rng(rng);
        let duration = time::Duration::minutes(from_start);
        c.1 + duration
    }
}

impl<L: Data> Dummy<DateTimeBetween<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(c: &DateTimeBetween<L>, rng: &mut R) -> Self {
        let datetime: time::OffsetDateTime = c.fake_with_rng(rng);
        datetime
            .format(&time::format_description::parse(L::TIME_DEFAULT_DATETIME_FORMAT).unwrap())
            .unwrap()
    }
}
