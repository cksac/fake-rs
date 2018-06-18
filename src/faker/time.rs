use std::u32;

use chrono::{Date, DateTime, Duration, NaiveTime, TimeZone, Utc};

use dummy::Dummy;
use helper::{self, gen_range};
use Fake;

const DEFAULT_DATE_FMT: &'static str = "%F";
const DEFAULT_TIME_FMT: &'static str = "%H:%M:%S%.f";
const DEFAULT_FMT: &'static str = "%+";
const YEAR_MAG: i32 = 3_000i32;
const NANO_MAX_BOUND: u32 = 1_000_000_000u32;
const SECS_PER_YEAR: i64 = 60 * 60 * 24 * 365;

pub trait Chrono: Fake {
    #[inline]
    fn time(fmt: Option<&str>) -> String {
        time(fmt)
    }

    #[inline]
    fn date(fmt: Option<&str>) -> String {
        date(fmt)
    }

    #[inline]
    fn datetime(fmt: Option<&str>) -> String {
        datetime(fmt)
    }

    #[inline]
    fn before(fmt: Option<&str>, end: &str) -> String {
        before(fmt, end)
    }

    #[inline]
    fn after(fmt: Option<&str>, start: &str) -> String {
        after(fmt, start)
    }

    #[inline]
    fn between(fmt: Option<&str>, start: &str, end: &str) -> String {
        between(fmt, start, end)
    }

    #[inline]
    fn duration() -> String {
        format!("{}", Duration::dummy())
    }
}

fn time(fmt: Option<&str>) -> String {
    let time = NaiveTime::dummy();

    format!("{}", time.format(fmt.unwrap_or(DEFAULT_TIME_FMT)))
}

fn date(fmt: Option<&str>) -> String {
    let date = Date::dummy();

    format!("{}", date.format(fmt.unwrap_or(DEFAULT_DATE_FMT)))
}

fn datetime(fmt: Option<&str>) -> String {
    let dt = DateTime::dummy();

    format!("{}", dt.format(fmt.unwrap_or(DEFAULT_FMT)))
}

fn between(fmt: Option<&str>, start: &str, end: &str) -> String {
    let format = fmt.unwrap_or(DEFAULT_FMT);
    let start_date: DateTime<Utc> = do_parse(start, format);
    let end_date: DateTime<Utc> = do_parse(end, format);

    let (s_secs, s_nano) = (start_date.timestamp(), start_date.timestamp_subsec_nanos());
    let (e_secs, e_nano) = (end_date.timestamp(), end_date.timestamp_subsec_nanos());

    let secs = gen_range(s_secs, e_secs + 1);

    let nsecs = if secs == e_secs {
        gen_range(0, e_nano)
    } else if secs == s_secs {
        gen_range(s_nano, NANO_MAX_BOUND)
    } else {
        gen_range(0, NANO_MAX_BOUND)
    };

    format!("{}", Utc.timestamp(secs, nsecs).format(format))
}

fn after(fmt: Option<&str>, date: &str) -> String {
    let format = fmt.unwrap_or(DEFAULT_FMT);
    let date = do_parse(date, format);

    let (min_secs, min_nsecs) = (date.timestamp(), date.timestamp_subsec_nanos());

    let secs = gen_range(min_secs, YEAR_MAG as i64 * SECS_PER_YEAR);
    let nsecs = if secs == min_secs {
        gen_range(min_nsecs, NANO_MAX_BOUND)
    } else {
        gen_range(0, NANO_MAX_BOUND)
    };

    format!("{}", Utc.timestamp(secs, nsecs).format(format))
}

fn before(fmt: Option<&str>, date: &str) -> String {
    let format = fmt.unwrap_or(DEFAULT_FMT);
    let date = do_parse(date, format);

    let (max_secs, max_nsecs) = (date.timestamp(), date.timestamp_subsec_nanos());

    let secs = gen_range(-(YEAR_MAG as i64 * SECS_PER_YEAR), max_secs + 1);
    let nsecs = if secs == max_secs {
        gen_range(0, max_nsecs + 1)
    } else {
        gen_range(0, NANO_MAX_BOUND)
    };

    format!("{}", Utc.timestamp(secs, nsecs).format(format))
}

impl Dummy for Duration {
    fn dummy() -> Duration {
        Duration::nanoseconds(i64::dummy())
    }
}

impl Dummy for Date<Utc> {
    fn dummy() -> Date<Utc> {
        let year = gen_range(0, YEAR_MAG);
        let end = if is_leap(year) { 366 } else { 365 };
        let day_ord = helper::gen_range(0, end);

        Utc.yo(year, day_ord)
    }
}

impl Dummy for NaiveTime {
    fn dummy() -> NaiveTime {
        let hour = gen_range(0, 24);
        let min = gen_range(0, 60);
        let sec = gen_range(0, 60);

        NaiveTime::from_hms(hour, min, sec)
    }
}

impl Dummy for DateTime<Utc> {
    fn dummy() -> DateTime<Utc> {
        let date = Date::dummy();
        let time = NaiveTime::dummy();

        date.and_time(time).expect("invalid date")
    }
}

// Parses a date, attmepting first to parse a date with
// and embedded timezone offset, and next just assumes
// UTC. Panics if both parse methods fail
fn do_parse(s: &str, fmt: &str) -> DateTime<Utc> {
    if let Ok(dt) = DateTime::parse_from_str(s, fmt) {
        return dt.with_timezone(&Utc);
    }

    match Utc.datetime_from_str(s, fmt) {
        Ok(dt) => dt,
        Err(e) => panic!("Couldn't parse date without offset: {}", e),
    }
}

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
