use rand::seq::IndexedRandom;

use crate::{
    faker::{automotive::raw::LicencePlate, impls::address::CityNameGenFn},
    locales::Data,
    Dummy,
};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct TR_TR;

impl Data for TR_TR {

    const NAME_FIRST_NAME: &'static [&'static str] = &[
        "Ahmet", "Mehmet", "Mustafa", "Ali", "Hüseyin", "Hasan",
        "Ayşe", "Fatma", "Emine", "Hatice", "Zeynep", "Elif",
        "Murat", "Can", "Kemal", "Burak", "Serkan", "Onur",
        "Ece", "Derya", "Selin", "Ceren", "Büşra", "Merve",
    ];

    const NAME_LAST_NAME: &'static [&'static str] = &[
        "Yılmaz", "Kaya", "Demir", "Çelik", "Şahin", "Yıldız",
        "Aydın", "Öztürk", "Arslan", "Doğan", "Kılıç", "Aslan",
        "Çetin", "Koç", "Polat", "Aksoy", "Erdoğan",
    ];

    const NAME_TITLE: &'static [&'static str] = &[
        "Sn.", "Dr.", "Prof.", "Doç.", "Av.", "Müh.",
    ];

    const NAME_SUFFIX: &'static [&'static str] = &["Bey", "Hanım"];

    const NAME_TPL: &'static str = "{FirstName} {LastName}";
    const NAME_WITH_TITLE_TPL: &'static str = "{Title} {FirstName} {LastName}";
    const JOB_SENIORITY: &'static [&'static str] = &["Uzman", "Başuzman", "Müdür", "Kıdemli"];
    const JOB_FIELD: &'static [&'static str] = &["Pazarlama", "Bilişim", "Muhasebe", "İdari İşler", "Reklamcılık", "Bankacılık", "Toplum Hizmetleri", "İnşaat", "Danışmanlık", "Tasarım", "Eğitim", "Tarım", "Kamu", "Sağlık", "Turizm ve Otelcilik", "Hukuk", "İmalat", "Madencilik", "Gayrimenkul", "Perakende", "Satış", "Teknoloji", ];
    const JOB_POSITION: &'static [&'static str] = &["Amir",  "Yönetici", "Memur", "Müdür", "Mühendis", "Direktör", "Koordinatör", "Yönetici", "Mimar", "Analist", "Tasarımcı", "Planlamacı", "Teknisyen", "Geliştirici", "Prodüktör", "Danışman", "Temsilci", "Stratejist"];
    const JOB_TITLE_TPL: &'static str = "{Seniority} {Field} {Position}";


    const ADDRESS_CITY_PREFIX: &'static [&'static str] =
        &["Merkez", "Yeni", "Eski"];

    const ADDRESS_CITY_SUFFIX: &'static [&'static str] =
        &["Mahallesi", "Köyü", "Beldesi"];


    const ADDRESS_CITY_TPL: &'static str = "{CityName} {CitySuffix}";

    const ADDRESS_CITY_WITH_PREFIX_TPL: &'static str =
        "{CityPrefix} {CityName} {CitySuffix}";


    const ADDRESS_COUNTRY: &'static [&'static str] = &[
        "ABD Küçük Dış Adaları",
        "ABD Virjin Adaları",
        "Afganistan",
        "Almanya",
        "Amerika Birleşik Devletleri",
        "Andorra",
        "Angola",
        "Antarktika",
        "Antigua ve Barbuda",
        "Arjantin",
        "Arnavutluk",
        "Avustralya",
        "Avusturya",
        "Azerbaycan",
        "Bahamalar",
        "Bahreyn",
        "Bangladeş",
        "Belarus",
        "Belçika",
        "Birleşik Arap Emirlikleri",
        "Birleşik Krallık",
        "Bosna-Hersek",
        "Brezilya",
        "Bulgaristan",
        "Çekya",
        "Çin",
        "Danimarka",
        "Endonezya",
        "Ermenistan",
        "Fas",
        "Finlandiya",
        "Fransa",
        "Güney Afrika",
        "Güney Kore",
        "Gürcistan",
        "Hindistan",
        "Hollanda",
        "Irak",
        "İran",
        "İrlanda",
        "İspanya",
        "İsrail",
        "İsveç",
        "İsviçre",
        "İtalya",
        "Japonya",
        "Kanada",
        "Katar",
        "Kazakistan",
        "Kıbrıs",
        "Kırgızistan",
        "Kolombiya",
        "Kuveyt",
        "Macaristan",
        "Meksika",
        "Mısır",
        "Norveç",
        "Özbekistan",
        "Pakistan",
        "Polonya",
        "Portekiz",
        "Romanya",
        "Rusya",
        "Sırbistan",
        "Suudi Arabistan",
        "Şili",
        "Tayland",
        "Tunus",
        "Türkiye",
        "Ukrayna",
        "Uruguay",
        "Venezuela",
        "Vietnam",
        "Yunanistan",
        "Zimbabve",
    ];
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] =
        &["Sokak", "Sokağı", "Cadde", "Caddesi", "Bulvar", "Bulvarı", "Yolu", "Meydanı"];
    const ADDRESS_STREET_TPL: &'static str = "{StreetName} {StreetSuffix}";

    const ADDRESS_TIME_ZONE: &'static [&'static str] = &["Europe/Istanbul"];
    const ADDRESS_STATE: &'static [&'static str] = &[
        "Adana", "Ankara", "Antalya", "Bursa", "İstanbul",
        "İzmir", "Konya", "Kayseri", "Kocaeli", "Samsun", "Trabzon",
    ];


    const ADDRESS_STATE_ABBR: &'static [&'static str] =
        &["01", "06", "07", "16", "34", "35", "42", "38", "41", "55", "61"];

    const ADDRESS_BUILDING_NUMBER_FORMATS: &'static [&'static str] =
        &["No: ###", "No: ##", "No: #"];

    const ADDRESS_ZIP_FORMATS: &'static [&'static str] = &["#####"];

    const ADDRESS_POSTCODE_FORMATS: &'static [&'static str] = &["#####"];

    const INTERNET_FREE_EMAIL_PROVIDER: &'static [&'static str] = &[
        "gmail.com",
        "hotmail.com",
        "outlook.com",
        "yahoo.com",
        "icloud.com",
    ];

    const INTERNET_DOMAIN_SUFFIX: &'static [&'static str] =
        &["com", "net", "org", "com.tr", "org.tr"];

    const PHONE_NUMBER_FORMATS: &'static [&'static str] = &[
        "+90 (212) ### ####",
        "+90 (216) ### ####",
        "+90 (262) ### ####",
    ];

    const PHONE_CELL_NUMBER_FORMATS: &'static [&'static str] = &[
        "+90 53# ### ####",
        "+90 54# ### ####",
        "+90 55# ### ####",
    ];
    const CHRONO_DEFAULT_TIME_FORMAT: &'static str = "%H:%M:%S";
    const CHRONO_DEFAULT_DATE_FORMAT: &'static str = "%d.%m.%Y";

    const CHRONO_DEFAULT_DATETIME_FORMAT: &'static str = "%d.%m.%Y %H:%M:%S";
    const TIME_DEFAULT_DATE_FORMAT: &'static str = "[day].[month].[year]";

    const TIME_DEFAULT_DATETIME_FORMAT: &'static str =
        "[day].[month].[year] [hour]:[minute]:[second]";
}

impl CityNameGenFn for TR_TR {}


const LICENSE_PLATE_TR_TR: &[&str] = &[
    "## ABC ###",
    "## AB ####",
];

impl Dummy<LicencePlate<TR_TR>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &LicencePlate<TR_TR>, rng: &mut R) -> Self {
        let fmt = LICENSE_PLATE_TR_TR.choose(rng).unwrap();
        crate::faker::impls::automotive::numerify_licence_plate(fmt, rng)
    }
}
