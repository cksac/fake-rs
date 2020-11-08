use crate::Fake;
use rand::Rng;
use std::char;

#[inline]
fn numerify_sym<R: Rng + ?Sized>(string: &str, rng: &mut R) -> String {
    string
        .chars()
        .map(|x| match x {
            '^' => char::from_digit((1..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            '#' => char::from_digit((0..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            other => other,
        })
        .collect()
}

macro_rules! def_fakers {
    (@m $locale_m:ident=>$locale_s:ident { $($name:ident($($arg:ident : $typ:ty),*);)+}) => {
        pub mod $locale_m {
            use super::raw;
            use crate::locales::$locale_s;
        $(
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name($($arg:$typ),*) -> raw::$name<$locale_s> {
                raw::$name($locale_s, $($arg),*)
            }
        )+
        }
    };
    ($($name:ident($($arg:ident : $typ:ty),*);)+) => {
        pub mod raw {
        $(
            pub struct $name<L>(pub L, $(pub $typ),*);
        )+
        }

        def_fakers!(@m en=>EN {$($name($($arg:$typ),*);)+});
        def_fakers!(@m zh_tw=>ZH_TW {$($name($($arg:$typ),*);)+});
    };
}

mod impls;

pub struct Semver();

pub enum SemverFaker {
    Default,
    Stable,
    Unstable
}

pub mod address {
    def_fakers! {
        CityPrefix();
        CitySuffix();
        CityName();
        CountryName();
        CountryCode();
        StreetSuffix();
        StreetName();
        TimeZone();
        StateName();
        StateAbbr();
        SecondaryAddressType();
        SecondaryAddress();
        ZipCode();
        PostCode();
        BuildingNumber();
        Latitude();
        Longitude();
    }
}

pub mod boolean {
    def_fakers! {
        Boolean(ratio: u8);
    }
}

#[cfg(feature = "chrono")]
pub mod chrono {
    def_fakers! {
        Time();
        Date();
        DateTime();
        Duration();
        DateTimeBefore(dt: chrono::DateTime<chrono::Utc>);
        DateTimeAfter(dt: chrono::DateTime<chrono::Utc>);
        DateTimeBetween(start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>);
    }
}

pub mod company {
    def_fakers! {
        CompanySuffix();
        CompanyName();
        Buzzword();
        BuzzwordMiddle();
        BuzzwordTail();
        CatchPhase();
        BsVerb();
        BsAdj();
        BsNoun();
        Bs();
        Profession();
        Industry();
    }
}

#[cfg(feature = "http")]
pub mod http {
    def_fakers! {
        RfcStatusCode();
        ValidStatusCode();
    }
}

pub mod internet {
    def_fakers! {
        FreeEmailProvider();
        DomainSuffix();
        FreeEmail();
        SafeEmail();
        Username();
        Password(len_range: std::ops::Range<usize>);
        IPv4();
        IPv6();
        IP();
        MACAddress();
        Color();
        UserAgent();
    }
}

pub mod job {
    def_fakers! {
        Seniority();
        Field();
        Position();
        Title();
    }
}

pub mod lorem {
    def_fakers! {
        Word();
        Words(count: std::ops::Range<usize>);
        Sentence(count: std::ops::Range<usize>);
        Sentences(count: std::ops::Range<usize>);
        Paragraph(count: std::ops::Range<usize>);
        Paragraphs(count: std::ops::Range<usize>);
    }
}

pub mod name {
    def_fakers! {
        FirstName();
        LastName();
        Title();
        Suffix();
        Name();
        NameWithTitle();
    }
}

pub mod number {
    def_fakers! {
        Digit();
        NumberWithFormat(fmt: &'static str);
    }
}

pub mod phone_number {
    def_fakers! {
        PhoneNumber();
        CellNumber();
    }
}

pub mod filesystem {
    def_fakers! {
        FilePath();
        FileName();
        FileExtension();
        DirPath();
        MimeType();
    }
}

pub mod currency {
    def_fakers! {
        CurrencyCode();
        CurrencyName();
        CurrencySymbol();
    }
}

pub mod finance {
    def_fakers! {
        Bic();
    }
}
