use rand::Rng;

use crate::{
    faker::{
        address::raw::{CityName, CityPrefix, CitySuffix},
        impls::address::CityNameGenFn,
        name::raw::{LastName, Name},
    },
    locales::Data,
    Fake,
};
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct NL_NL;

impl Data for NL_NL {
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        "Sanne",
        "Maaike",
        "Femke",
        "Marieke",
        "Marjolein",
        "Nienke",
        "Merel",
        "Janneke",
        "Marloes",
        "Lisanne",
        "Annemarie",
        "Renske",
        "Evelien",
        "Rianne",
        "Annemieke",
        "Loes",
        "Karlijn",
        "Mieke",
        "Hanneke",
        "Marjon",
        "Lonneke",
        "Moniek",
        "Paulien",
        "Annet",
        "Anne-Marie",
        "Fleur",
        "Lieke",
        "Milou",
        "Danique",
        "Veerle",
        "Mirthe",
        "Noortje",
        "Linde",
        "Pien",
        "Madelief",
        "Mirte",
        "Pleun",
        "FAnke",
        "Yfke",
        "Fem",
        "Daniek",
        "Patty",

        "Jeroen",
        "Martijn",
        "Maarten",
        "Wouter",
        "Bas",
        "Joost",
        "Jelle",
        "Koen",
        "Bastiaan",
        "Michiel",
        "Remco",
        "Sjoerd",
        "Matthijs",
        "Joris",
        "Menno",
        "Roel",
        "Robbert",
        "Rik",
        "Ruud",
        "Jos",
        "Lennart",
        "Daan",
        "Thijs",
        "Luuk",
        "Stijn",
        "Floris,",
        "Teun",
        "Niek",
        "Joep",
        "Wessel",
        "Pepijn",
        "Tijn",
        "Guus",
        "Thijmen",
        "Hidde",
        "Marijn",
        "Stef",
        "Sven",
        "Noud",
        "Loek",
        "Max",
    ];

    const NAME_LAST_NAME: &'static [&'static str] = &[
        "de Jong",
        "Jansen",
        "de Vries",
        "van den Berg",
        "van Dijk",
        "Bakker",
        "Janssen",
        "Visser",
        "Smit",
        "Meijer",
        "de Boer",
        "Mulder",
        "de Groot",
        "Bos",
        "Vos",
        "Peters",
        "Hendriks",
        "van Leeuwen",
        "Dekker",
        "Brouwer",
        "de Wit",
        "Dijkstra",
        "Smits",
        "de Graaaf",
        "van der Meer",
        "van der Linden",
        "Kok",
        "Jakobs",
        "de Haan",
        "Vermeulen",
        "van den Heuvel",
        "van der Veen",
        "van den Broek",
        "de Bruijn",
        "de Bruin",
        "van der Heijden",
        "Schouten",
        "van Beek",
        "Willems",
        "van Vliet",
        "van de Ven",
        "Hoekstra",
        "Maas",
        "Verhoeven",
        "Koster",
        "van Dam",
        "van der Wal",
        "Prins",
        "Blom",
        "Hisman",
        "Peeters",
        "de Jonge",
        "Kuipers",
        "van Veen",
        "Post",
        "Kuiper",
        "Veenstra",
        "Kramer",
        "van den Brink",
        "Scholten",
        "van Wijk",
        "Postma",
        "Martens",
        "Vink",
        "de Ruiter",
        "Timmermans",
        "Groen",
        "Gerritsen",
        "Jonker",
        "van Loon",
        "Boer",
        "van der Velde",
        "Willemsen",
        "Smeets",
        "de Lange",
        "de Vos",
        "Bosch",
        "van Dongen",
        "Schipper",
        "de Koning",
        "van der Laan",
        "Koning",
        "van der Velden",
        "Driessen",
        "van Doorn",
        "Hermans",
        "Evers",
        "van den Bosch",
        "van der Meulen",
        "Hofman",
        "Bosman",
        "Wolters",
        "Sanders",
        "van der Horst",
        "Mol",
        "Kuijpers",
        "Molenaar",
        "van de Pol",
        "de Leeuw",
        "Verbeek",
    ];
    const NAME_TITLE: &'static [&'static str] = &["Mevrouw", "Meneer"];

    const ADDRESS_STATE: &'static [&'static str] = &[
        "Groningen",
        "Fryslân",
        "Drenthe",
        "Overijssel",
        "Flevoland",
        "Gelderland",
        "Utrecht",
        "Noord-Holland",
        "Zuid-Holland",
        "Zeeland",
        "Noord-Brabant",
        "Limburg",
    ];

    const ADDRESS_STATE_ABBR: &'static [&'static str] = &[
        "DR", "FL", "FR", "GD", "GR", "LB", "NB", "NH", "OV", "UT", "ZH", "ZL",
    ];

    const ADDRESS_COUNTRY: &'static [&'static str] = &["Nederland", "The Netherlands"];
    const ADDRESS_CITY_SUFFIX: &'static [&'static str] = &[
        "aarde", "akker", "daal", "donk", "drecht", "geest", "gem", "gestel", "heem",
        "holt", "horst", "iacum", "lare", "loo", "rode", "schot", "schote", "sel",
        "speet", "voorde", "werd", "ward", "werf", "wierd", "wijk", "wolde",
    ];

    const ADDRESS_CITY_PREFIX: &'static [&'static str] = &[
        "Oud", "Groot", "Klein", "Lage", "Hoge", "Nieuw", "Boven", "Beneden",
    ];
    const ADDRESS_ZIP_FORMATS: &'static [&'static str] = &["#####"];
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = &[
        "laan", "pad", "straat", "plaats", "steeg", "weg",
    ];
    const ADDRESS_SECONDARY_ADDR_TYPE: &'static [&'static str] =
        &["Gebouw", "Verdieping", "Ingang", "Oprit"];

    const ADDRESS_STREET_TPL: &'static str = "{StreetName} {StreetSuffix}";

    const COMPANY_SUFFIX: &'static [&'static str] = &[
        "vof",
        "cv",
        "bv",
        "nv",
        "ez",
        "mts",
        "eG",
        "stichting",
        "vereniging",
        "coöperatie",
    ];
}

impl CityNameGenFn for NL_NL {
    fn gen<R: Rng + ?Sized>(c: &CityName<Self>, rng: &mut R) -> String {
        // common formats for city names
        const ADDRESS_CITY_WITHOUT_PREFIX: &str = "{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITHOUT_SPACE: &str = "{CityPrefix}{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_SPACE: &str = "{CityPrefix} {CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_DASH_TPL: &str = "{CityPrefix}-{CityName}{CitySuffix}";
        const ADDRESS_CITY_WITH_RIVER_TPL: &str = "{CityPrefix}-{CityName}{CitySuffix} {River}";

        let result = match (0..5).fake_with_rng::<u8, _>(rng) {
            0 => ADDRESS_CITY_WITHOUT_SPACE
                .replace(
                    "{CityPrefix}",
                    CityPrefix(c.0).fake_with_rng::<&str, _>(rng),
                )
                .replace(
                    "{CityName}",
                    (LastName(c.0).fake_with_rng::<String, _>(rng))
                        .to_lowercase()
                        .as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
            1 => ADDRESS_CITY_WITH_SPACE
                .replace(
                    "{CityPrefix}",
                    CityPrefix(c.0).fake_with_rng::<&str, _>(rng),
                )
                .replace(
                    "{CityName}",
                    LastName(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
            2 => ADDRESS_CITY_WITH_DASH_TPL
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
            3 => ADDRESS_CITY_WITH_RIVER_TPL
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
            _ => ADDRESS_CITY_WITHOUT_PREFIX
                .replace(
                    "{CityName}",
                    Name(c.0).fake_with_rng::<String, _>(rng).as_ref(),
                )
                .replace(
                    "{CitySuffix}",
                    CitySuffix(c.0).fake_with_rng::<&str, _>(rng),
                ),
        };
        result
    }
}
