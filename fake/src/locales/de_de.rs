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
pub struct DE_DE;

impl Data for DE_DE {
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        "Erika", // German ID Card
        "André",
        "Johanna",
        "Edeltraut",
        "Lisbeth",
        "Ağça",
        "Hans",
        "Günther",
        "Annekathrin",
        "Karl",
        "Lilly",
        "Hildegard",
        "Maija", // EU Citizen Card
        "Tomáš",
        "Karel",
        "Maria",
        "Elena",
        "Inês",
        "Paddy",
        "Åsa",
        "Maria",
        "Ursula", // 50 most common female first names
        "Monika",
        "Petra",
        "Elisabeth",
        "Sabine",
        "Renate",
        "Helga",
        "Karin",
        "Brigitte",
        "Ingrid",
        "Erika",
        "Andrea",
        "Gisela",
        "Claudia",
        "Susanne",
        "Gabriele",
        "Christa",
        "Christine",
        "Hildegard",
        "Anna",
        "Birgit",
        "Barbara",
        "Gertrud",
        "Heike",
        "Marianne",
        "Elke",
        "Martina",
        "Angelika",
        "Irmgard",
        "Inge",
        "Ute",
        "Elfriede",
        "Doris",
        "Marion",
        "Ruth",
        "Ulrike",
        "Hannelore",
        "Jutta",
        "Gerda",
        "Kerstin",
        "Ilse",
        "Anneliese",
        "Margarete",
        "Ingeborg",
        "Anja",
        "Edith",
        "Sandra",
        "Waltraud",
        "Beate",
        "Peter",
        "Wolfgang", // 50 most common male firstnames
        "Michael",
        "Werner",
        "Klaus",
        "Thomas",
        "Manfred",
        "Helmut",
        "Jürgen",
        "Heinz",
        "Gerhard",
        "Andreas",
        "Hans",
        "Josef",
        "Günter",
        "Dieter",
        "Horst",
        "Walter",
        "Frank",
        "Bernd",
        "Karl",
        "Herbert",
        "Franz",
        "Martin",
        "Uwe",
        "Georg",
        "Heinrich",
        "Stefan",
        "Christian",
        "Karl-Heinz",
        "Rudolf",
        "Kurt",
        "Hermann",
        "Johann",
        "Wilhelm",
        "Siegfried",
        "Rolf",
        "Joachim",
        "Alfred",
        "Rainer",
        "Jörg",
        "Ralf",
        "Erich",
        "Norbert",
        "Bernhard",
        "Willi",
        "Alexander",
        "Ulrich",
        "Markus",
        "Matthias",
    ];
    const NAME_LAST_NAME: &'static [&'static str] = &[
        "Mustermann", // German ID Card
        "Musterfrau",
        "Gabler",
        "Müller-Schwarzenberg",
        "Ćosić",
        "Özm̂en",
        "Drebenbusch-Dalgoßen",
        "Lerch",
        "Hillebrandt",
        "Schuster",
        "Meikäläinen", // EU Citizen Card
        "Vomáčka",
        "Rudaí",
        "Lærke",
        "Müller",
        "Schmidt", // 100 most common german lastnames
        "Schneider",
        "Fischer",
        "Weber",
        "Meyer",
        "Wagner",
        "Becker",
        "Schulz",
        "Hoffmann",
        "Schäfer",
        "Bauer",
        "Koch",
        "Richter",
        "Klein",
        "Wolf",
        "Schröder",
        "Neumann",
        "Schwarz",
        "Braun",
        "Hofmann",
        "Zimmermann",
        "Schmitt",
        "Hartmann",
        "Krüger",
        "Schmid",
        "Werner",
        "Lange",
        "Schmitz",
        "Meier",
        "Krause",
        "Maier",
        "Lehmann",
        "Huber",
        "Mayer",
        "Herrmann",
        "Köhler",
        "Walter",
        "König",
        "Schulze",
        "Fuchs",
        "Kaiser",
        "Lang",
        "Weiß",
        "Peters",
        "Scholz",
        "Jung",
        "Möller",
        "Hahn",
        "Keller",
        "Vogel",
        "Schubert",
        "Roth",
        "Frank",
        "Friedrich",
        "Beck",
        "Günther",
        "Berger",
        "Winkler",
        "Lorenz",
        "Baumann",
        "Schuster",
        "Kraus",
        "Böhm",
        "Simon",
        "Franke",
        "Albrecht",
        "Winter",
        "Ludwig",
        "Martin",
        "Krämer",
        "Schumacher",
        "Vogt",
        "Jäger",
        "Stein",
        "Otto",
        "Groß",
        "Sommer",
        "Haas",
        "Graf",
        "Heinrich",
        "Seidel",
        "Schreiber",
        "Ziegler",
        "Brandt",
        "Kuhn",
        "Schulte",
        "Dietrich",
        "Kühn",
        "Engel",
        "Pohl",
        "Horn",
        "Sauer",
        "Arnold",
        "Thomas",
        "Bergmann",
        "Busch",
        "Pfeiffer",
        "Voigt",
        "Götz",
    ];
    const NAME_TITLE: &'static [&'static str] = &["Frau", "Herr"];

    const ADDRESS_STATE: &'static [&'static str] = &[
        "Baden-Württemberg",
        "Bayern",
        "Berlin",
        "Brandenburg",
        "Bremen",
        "Hamburg",
        "Hessen",
        "Mecklenburg-Vorpommern",
        "Niedersachsen",
        "Nordrhein-Westfalen",
        "Rheinland-Pfalz",
        "Saarland",
        "Sachsen",
        "Sachsen-Anhalt",
        "Schleswig-Holstein",
        "Thüringen",
    ];

    const ADDRESS_STATE_ABBR: &'static [&'static str] = &[
        "BW", "BY", "BE", "BB", "HB", "HH", "HE", "MV", "NI", "NW", "RP", "SL", "SN", "ST", "SH",
        "TH",
    ];

    const ADDRESS_COUNTRY: &'static [&'static str] = &["Deutschland", "Germany"];
    const ADDRESS_CITY_SUFFIX: &'static [&'static str] = &[
        "berg", "burg", "feld", "furt", "heim", "ing", "kirchen", "stadt",
    ];

    const ADDRESS_CITY_PREFIX: &'static [&'static str] = &[
        "Alt", "Bad", "Groß", "Hohen", "Klein", "Neu", "Alt", "Ober", "Unter",
    ];
    const ADDRESS_ZIP_FORMATS: &'static [&'static str] = &["#####"];
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = &[
        "allee", "gang", "gasse", "pfad", "platz", "steg", "straße", "ufer", "weg",
    ];
    const ADDRESS_SECONDARY_ADDR_TYPE: &'static [&'static str] =
        &["Hof", "Gebäude", "Etage", "Eingang", "Aufgang"];

    const ADDRESS_STREET_TPL: &'static str = "{StreetName}{StreetSuffix}";

    const COMPANY_SUFFIX: &'static [&'static str] = &[
        "OHG",
        "KG",
        "GbR",
        "AG",
        "GmbH",
        "UG",
        "eG",
        "Stiftung",
        "e.V.",
        "und Partner",
        "& Partner",
    ];
}

impl CityNameGenFn for DE_DE {
    fn gen<R: Rng + ?Sized>(c: &CityName<Self>, rng: &mut R) -> String {
        // german cities are often suffixed by a river name
        const RIVERS: [&str; 10] = [
            "(Rhein)", "(Elbe)", "(Donau)", "(Main)", "(Weser)", "(Oder)", "(Neckar)", "(Havel)",
            "(Mosel)", "(Ems)",
        ];

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
                )
                .replace("{River}", RIVERS[rng.random_range(0..RIVERS.len())]),
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
