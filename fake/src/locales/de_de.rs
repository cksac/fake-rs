use crate::locales::Data;

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
    const ADDRESS_CITY_TPL: &'static str = "{CityName} {CitySuffix}";
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
