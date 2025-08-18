use crate::faker::address::raw::*;
use crate::faker::name::raw::{FirstName, LastName, Name};
use crate::faker::numerify_sym;
use crate::locales::{Data};
use crate::{Dummy, Fake, Faker};
use rand::seq::IndexedRandom;
use rand::Rng;

const ADDRESS_ZIP_CODE_CHARS: [char; 23] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V',
    'W', 'X', 'Y', 'Z',
];

#[inline]
pub(crate) fn numerify_zip_code<R: Rng + ?Sized>(string: &str, rng: &mut R) -> String {
    string
        .chars()
        .map(|x| match x {
            '$' => *ADDRESS_ZIP_CODE_CHARS.choose(rng).unwrap(),
            '#' => char::from_digit((0..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            other => other,
        })
        .collect()
}

impl<L: Data> Dummy<CityPrefix<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CityPrefix<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_CITY_PREFIX.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CityPrefix<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CityPrefix<L>, rng: &mut R) -> Self {
        L::ADDRESS_CITY_PREFIX.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<CitySuffix<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CitySuffix<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_CITY_SUFFIX.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CitySuffix<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CitySuffix<L>, rng: &mut R) -> Self {
        L::ADDRESS_CITY_SUFFIX.choose(rng).unwrap()
    }
}

pub trait CityNameGenFn: Data + Sized + Copy {
    fn gen<R: Rng + ?Sized>(c: &CityName<Self>, rng: &mut R) -> String {
        match (0..5).fake_with_rng::<u8, _>(rng) {
            0 => Self::ADDRESS_CITY_WITH_PREFIX_TPL
                .replace(
                    "{CityPrefix}",
                    CityPrefix(c.0).fake_with_rng::<&str, _>(rng),
                )
                .replace(
                    "{CityName}",
                    Name(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
            1 => Self::ADDRESS_CITY_TPL
                .replace("{CityName}", FirstName(c.0).fake_with_rng::<&str, _>(rng))
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
            _ => Self::ADDRESS_CITY_TPL
                .replace("{CityName}", LastName(c.0).fake_with_rng::<&str, _>(rng))
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
        }
    }
}

impl<L: CityNameGenFn> Dummy<CityName<L>> for String {
    #[inline(always)]
    fn dummy_with_rng<R: Rng + ?Sized>(c: &CityName<L>, rng: &mut R) -> Self {
        L::gen(c, rng)
    }
}

impl<L: Data> Dummy<CountryName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CountryName<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_COUNTRY.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CountryName<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CountryName<L>, rng: &mut R) -> Self {
        L::ADDRESS_COUNTRY.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<CountryCode<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CountryCode<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_COUNTRY_CODE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CountryCode<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CountryCode<L>, rng: &mut R) -> Self {
        L::ADDRESS_COUNTRY_CODE.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<StreetSuffix<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &StreetSuffix<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_STREET_SUFFIX.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<StreetSuffix<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &StreetSuffix<L>, rng: &mut R) -> Self {
        L::ADDRESS_STREET_SUFFIX.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<StreetName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &StreetName<L>, rng: &mut R) -> Self {
        let name = if Faker.fake_with_rng::<bool, _>(rng) {
            FirstName(c.0).fake_with_rng::<&str, _>(rng)
        } else {
            LastName(c.0).fake_with_rng::<&str, _>(rng)
        };
        L::ADDRESS_STREET_TPL.replace("{StreetName}", name).replace(
            "{StreetSuffix}",
            StreetSuffix(c.0).fake_with_rng::<&str, _>(rng),
        )
    }
}

impl<L: Data> Dummy<TimeZone<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &TimeZone<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_TIME_ZONE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<TimeZone<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &TimeZone<L>, rng: &mut R) -> Self {
        L::ADDRESS_TIME_ZONE.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<StateName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &StateName<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_STATE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<StateName<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &StateName<L>, rng: &mut R) -> Self {
        L::ADDRESS_STATE.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<StateAbbr<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &StateAbbr<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_STATE_ABBR.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<StateAbbr<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &StateAbbr<L>, rng: &mut R) -> Self {
        L::ADDRESS_STATE_ABBR.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<SecondaryAddressType<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &SecondaryAddressType<L>, rng: &mut R) -> Self {
        let s = *L::ADDRESS_SECONDARY_ADDR_TYPE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<SecondaryAddressType<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &SecondaryAddressType<L>, rng: &mut R) -> Self {
        L::ADDRESS_SECONDARY_ADDR_TYPE.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<SecondaryAddress<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &SecondaryAddress<L>, rng: &mut R) -> Self {
        L::ADDRESS_SECONDARY_ADDR_TPL
            .replace(
                "{SecondaryAddrType}",
                SecondaryAddressType(c.0).fake_with_rng::<&str, _>(rng),
            )
            .replace(
                "{Number}",
                (1..99).fake_with_rng::<u8, _>(rng).to_string().as_ref(),
            )
    }
}

impl<L: Data + Copy> Dummy<ZipCode<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &ZipCode<L>, rng: &mut R) -> Self {
        let fmt = L::ADDRESS_ZIP_FORMATS.choose(rng).unwrap();
        numerify_zip_code(fmt, rng)
    }
}

impl<L: Data + Copy> Dummy<PostCode<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &PostCode<L>, rng: &mut R) -> Self {
        let fmt = L::ADDRESS_POSTCODE_FORMATS.choose(rng).unwrap();
        numerify_sym(fmt, rng)
    }
}

impl<L: Data + Copy> Dummy<BuildingNumber<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BuildingNumber<L>, rng: &mut R) -> Self {
        let fmt = L::ADDRESS_BUILDING_NUMBER_FORMATS.choose(rng).unwrap();
        numerify_sym(fmt, rng)
    }
}

impl<L: Data> Dummy<Latitude<L>> for f64 {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Latitude<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng::<f64, _>(rng) * 180_f64 - 90_f64
    }
}

impl<L: Data> Dummy<Latitude<L>> for f32 {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Latitude<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng::<f32, _>(rng) * 360_f32 - 90_f32
    }
}

impl<L: Data> Dummy<Latitude<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Latitude<L>, rng: &mut R) -> Self {
        c.fake_with_rng::<f64, _>(rng).to_string()
    }
}

impl<L: Data> Dummy<Longitude<L>> for f64 {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Longitude<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng::<f64, _>(rng) * 360_f64 - 90_f64
    }
}

impl<L: Data> Dummy<Longitude<L>> for f32 {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Longitude<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng::<f32, _>(rng) * 360_f32 - 90_f32
    }
}

impl<L: Data> Dummy<Longitude<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Longitude<L>, rng: &mut R) -> Self {
        c.fake_with_rng::<f32, _>(rng).to_string()
    }
}

impl<L: Data> Dummy<Geohash<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(g: &Geohash<L>, rng: &mut R) -> Self {
        L::ADDRESS_GEOHASH_CHARS
            .choose_multiple(rng, g.1 as usize)
            .cloned()
            .collect()
    }
}