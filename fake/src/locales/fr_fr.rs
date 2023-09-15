use crate::locales::Data;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct FR_FR;

impl Data for FR_FR {
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        "Amandine",
        "Béatrice",
        "Céline",
        "Damien",
        "Elodie",
        "François",
        "Gerard",
        "Héloïse",
        "Ingrid",
        "Jérémie",
        "Karim",
        "Léopold",
        "Martine",
        "Nicolas",
        "Océane",
        "Pascal",
        "Quentin",
        "Régis",
        "Salomé",
        "Thierry",
        "Ugo",
        "Victor",
        "William",
        "Xavier",
        "Yann",
        "Zoé",
    ];
    const NAME_LAST_NAME: &'static [&'static str] = &[
        "Aubry",
        "Barbier",
        "Cannet",
        "Dupond",
        "Évrat",
        "Ferry",
        "Geneau",
        "Hénaut",
        "Ivanov",
        "Janel",
        "Kléber",
        "Lombard",
        "Martin",
        "Niels",
        "Ogier",
        "Parmentier",
        "Quesada",
        "Rollin",
        "Sablonnière",
        "Truchot",
        "Urbain",
        "Vergez",
        "Walliand",
        "Xharde",
        "Yvars",
        "Zola",
    ];

    const INTERNET_FREE_EMAIL_PROVIDER: &'static [&'static str] = &[
        "free.fr",
        "sfr.fr",
        "laposte.fr",
        "outlook.fr",
        "hotmail.fr",
        "orange.fr",
    ];

    const PHONE_NUMBER_FORMATS: &'static [&'static str] = &[
        "01 ## ## ## ##",
        "02 ## ## ## ##",
        "03 ## ## ## ##",
        "04 ## ## ## ##",
        "05 ## ## ## ##",
        "08 ## ## ## ##",
        "09 ## ## ## ##",
    ];
    const PHONE_CELL_NUMBER_FORMATS: &'static [&'static str] =
        &["06 ## ## ## ##", "07 ## ## ## ##"];
}
