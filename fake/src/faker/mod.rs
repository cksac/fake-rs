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
    (@m $locale_m:ident=>$locale_s:ident { $($name:ident$(< $($lts:lifetime),* >)?($($arg:ident : $typ:ty),*);)+}) => {
        pub mod $locale_m {
            use super::raw;
            use crate::locales::$locale_s;
        $(
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name$(< $($lts),* >)?($($arg:$typ),*) -> raw::$name<$($($lts),*,)?$locale_s> {
                raw::$name($locale_s, $($arg),*)
            }
        )+
        }
    };
    ($($name:ident$(< $($lts:lifetime),* >)?($($arg:ident : $typ:ty),*);)+) => {
        pub mod raw {
        $(
            pub struct $name<$( $($lts),* , )?L>(pub L, $(pub $typ),*);
        )+
        }

        def_fakers!(@m en=>EN {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m fr_fr=>FR_FR {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m zh_tw=>ZH_TW {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m zh_cn=>ZH_CN {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m ar_sa=>AR_SA {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m ja_jp=>JA_JP {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m pt_br=>PT_BR {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m pt_pt=>PT_PT {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m de_de=>DE_DE {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m it_it=>IT_IT {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m cy_gb=>CY_GB {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});
        def_fakers!(@m nl_nl=>NL_NL {$($name$(< $($lts),* >)?($($arg:$typ),*);)+});

    };
}

pub mod impls;

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
        Geohash(precision: u8);
    }
}

pub mod barcode {
    def_fakers! {
        Isbn();
        Isbn10();
        Isbn13();
    }
}

pub mod boolean {
    def_fakers! {
        Boolean(ratio: u8);
    }
}

#[cfg(feature = "random_color")]
pub mod color {
    def_fakers! {
        HexColor();
        RgbColor();
        RgbaColor();
        HslColor();
        HslaColor();
        Color();
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

#[cfg(feature = "time")]
pub mod time {
    def_fakers! {
        Time();
        Date();
        DateTime();
        Duration();
        DateTimeBefore(dt: time::OffsetDateTime);
        DateTimeAfter(dt: time::OffsetDateTime);
        DateTimeBetween(start: time::OffsetDateTime, end: time::OffsetDateTime);
    }
}

pub mod creditcard {
    def_fakers! {
        CreditCardNumber();
    }
}

pub mod company {
    def_fakers! {
        CompanySuffix();
        CompanyName();
        Buzzword();
        BuzzwordMiddle();
        BuzzwordTail();
        CatchPhrase();
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

pub mod markdown {
    def_fakers! {
        ItalicWord();
        BoldWord();
        Link();
        BulletPoints(count: std::ops::Range<usize>);
        ListItems(count: std::ops::Range<usize>);
        BlockQuoteSingleLine(count: std::ops::Range<usize>);
        BlockQuoteMultiLine(count: std::ops::Range<usize>);
        Code(count: std::ops::Range<usize>);
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
        NumberWithFormat<'a>(fmt: &'a str);
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
        Semver();
        SemverStable();
        SemverUnstable();
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
        Isin();
    }
}

pub mod administrative {
    def_fakers! {
        HealthInsuranceCode();
    }
}

pub mod automotive {
    def_fakers! {
        LicencePlate();
    }
}
