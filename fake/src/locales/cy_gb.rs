use crate::{faker::impls::address::CityNameGenFn, locales::Data};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct CY_GB;

impl Data for CY_GB {
    // Source:
    // https://www.gov.wales/cymraeg-for-kids/welsh-names-for-children (and more sites)
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        "Addien",
        "Aeres",
        "Aled",
        "Alwyn",
        "Alys",
        "Aneira",
        "Angharad",
        "Ariana",
        "Arianwen",
        "Arlais",
        "Arthur",
        "Arwel",
        "Arwyn",
        "Aures",
        "Avalon",
        "Bethan",
        "Briallen",
        "Brian",
        "Bronwen",
        "Bryn",
        "Brynna",
        "Caden",
        "Caerwyn",
        "Cai",
        "Cariad",
        "Carwyn",
        "Carys",
        "Cassian",
        "Catrin",
        "Celyn",
        "Ceri",
        "Ceridwen",
        "Ceron",
        "Cerys",
        "Dafydd",
        "Delyth",
        "Deri",
        "Derwen",
        "Deryn",
        "Dewi",
        "Dilys",
        "Dwyn",
        "Dylan",
        "Ebrill",
        "Edryd",
        "Efa",
        "Eira",
        "Eirlys",
        "Eirwen",
        "Eleri",
        "Elin",
        "Elisedd",
        "Ellis",
        "Eluned",
        "Emlyn",
        "Emyr",
        "Enfys",
        "Enid",
        "Erin",
        "Eurion",
        "Evan",
        "Ffion",
        "Flynn",
        "Gareth",
        "Gawain",
        "Geraint",
        "Gethin",
        "Glain",
        "Griffin",
        "Griffith",
        "Gruffudd",
        "Guinevere",
        "Gwaltney",
        "Gwen",
        "Gwendolyn",
        "Gwilym",
        "Gwuan",
        "Gwyn",
        "Gwyneth",
        "Gwynfor",
        "Harri",
        "Hayden",
        "Hefin",
        "Heledd",
        "Huw",
        "Hywel",
        "Idwal",
        "Ieuan",
        "Ifor",
        "Iola",
        "Iolo",
        "Isolde",
        "Iwan",
        "Jac",
        "Jennifer",
        "Jeston",
        "Kendrick",
        "Kiah",
        "Kynan",
        "Lloyd",
        "Llwyd",
        "Llywelyn",
        "Lowri",
        "Lunet",
        "Lynessa",
        "Mabon",
        "Macsen",
        "Maddox",
        "Mai",
        "Mali",
        "Manon",
        "Mari",
        "Megan",
        "Meilyr",
        "Meredith",
        "Mererid",
        "Merlin",
        "Merlyn",
        "Morgan",
        "Morgan",
        "Morwenna",
        "Nesta",
        "Nia",
        "Osian",
        "Owain",
        "Rhain",
        "Rhett",
        "Rhian",
        "Rhiannon",
        "Rhodri",
        "Rhoslyn",
        "Rhoswen",
        "Rhydian",
        "Rhys",
        "Sara",
        "Seren",
        "Sian",
        "Siani",
        "Sion",
        "Sioned",
        "Siriol",
        "Siselya",
        "Steffan",
        "Talaith",
        "Tegan",
        "Telor",
        "Tirion",
        "Tomos",
        "Trevyn",
        "Trystan",
    ];

    // Source:
    // https://en.wikipedia.org/wiki/Category:Surnames_of_Welsh_origin
    const NAME_LAST_NAME: &'static [&'static str] = &[
        "Ab Owain",
        "Ab Owen",
        "Abse",
        "Allen",
        "Baines",
        "Baugh",
        "Bevan",
        "Carew",
        "Castell",
        "Cox",
        "Crowther",
        "Daffey",
        "Davies",
        "Davies-Jones",
        "Davis",
        "Ellis",
        "Evans",
        "Every",
        "Fardd",
        "Foulkes",
        "Glasco",
        "Glascock",
        "Glasscock",
        "Griffith",
        "Griffiths",
        "Guynn",
        "Hanford",
        "Hooley",
        "Hopkins",
        "Hopkinson",
        "Howell",
        "Hughes",
        "Jenner",
        "Jones",
        "Lewis",
        "Leyshon",
        "Lloyd",
        "Lowcock",
        "Lush",
        "Machen",
        "Maddock",
        "Maddocks",
        "Maddox",
        "Medwin",
        "Merwin",
        "Morgan",
        "Myddelton",
        "Naegle",
        "Nanney",
        "Nash-Williams",
        "Neagle",
        "Nutbrown",
        "Owen",
        "Owens",
        "Perry",
        "Phillips",
        "Powell",
        "Price",
        "Prichard",
        "Pritchard",
        "Probert",
        "Prodger",
        "Prosser",
        "Rees-Jones",
        "Rhys-Davies",
        "Shankland",
        "Terfel",
        "Thomas",
        "Upjohn",
        "Uprichard",
        "Vaughan",
        "Wales",
        "Watkins",
        "Wegg-Prosser",
        "Williams",
        "Williams-Wynn",
        "Wogan",
        "Wynn-Williams",
        "Wynne-Jones",
    ];

    const NAME_TITLE: &'static [&'static str] = &["Mr.", "Ms.", "Mx."];

    const ADDRESS_CITY_TPL: &'static str = "{CityName}";

    // Source:
    // https://www.carmarthenshire.gov.wales/media/1232405/snn-policy-june-2023-en.pdf
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = &[
        "Rhodfa",
        "Cylch",
        "Clôs",
        "Comin",
        "Cwrt or Llys",
        "Cilgant",
        "Tywyn",
        "Rhodfa",
        "Pen",
        "Gerddi",
        "Gelli",
        "Rhiw",
        "Lôn",
        "Dol",
        "Dol",
        "Stablau",
        "Parêd",
        "Parc",
        "Llwybr",
        "Maes",
        "Dyfroedd",
        "Crib",
        "Bryn",
        "Heol",
        "Rhes",
        "Sgwâr",
        "Stryd",
        "Teras",
        "Gwel y",
        "Rhodfa",
        "Ffordd",
        "Glanfa",
    ];

    const ADDRESS_STREET_TPL: &'static str = "{StreetSuffix} {StreetName}";
    const ADDRESS_CITY_WITH_PREFIX_TPL: &'static str = "{CityPrefix} {CityName} {CitySuffix}";
    const ADDRESS_SECONDARY_ADDR_TPL: &'static str = "{SecondaryAddrType} {Number}";

    // Country list:
    // https://www.101languages.net/welsh/country-names-welsh/
    const ADDRESS_COUNTRY: &'static [&'static str] = &[
        "Affganistan",
        "Albania",
        "Algeria",
        "Andorra",
        "Angola",
        "Antigwa a Barbiwda",
        "Yr Ariannin",
        "Armenia",
        "Awstralia",
        "Awstria",
        "Aserbaijan",
        "Y Bahamas",
        "Bahrain",
        "Bangladesh",
        "Barbados",
        "Belarws",
        "Gwlad Belg",
        "Belize",
        "Benin",
        "Bhwtan",
        "Bolifia",
        "Bosnia a Hercegovina",
        "Botswana",
        "Brasil",
        "Brunei",
        "Bwlgaria",
        "Burkina Faso",
        "Bwrwndi",
        "Cambodia",
        "Camerŵn",
        "Canada",
        "Cabo Verde",
        "Gweriniaeth Canolbarth Affrica",
        "Tchad",
        "Chile",
        "Tsieina",
        "Colombia",
        "Y Comoros",
        "Costa Rica",
        "Côte d’Ivoire",
        "Croatia",
        "Ciwba",
        "Cyprus",
        "Y Weriniaeth Tsiec",
        "Gweriniaeth Ddemocrataidd Congo",
        "Denmarc",
        "Djibouti",
        "Dominica",
        "Gweriniaeth Dominica",
        "Dwyrain Timor",
        "Ecwador",
        "Yr Aifft",
        "El Salvador",
        "Gini Gyhydeddol",
        "Eritrea",
        "Estonia",
        "Ethiopia",
        "Ffiji",
        "Y Ffindir",
        "Ffrainc",
        "Gabon",
        "Y Gambia",
        "Georgia",
        "Yr Almaen",
        "Ghana",
        "Gwlad Groeg",
        "Grenada",
        "Guatemala",
        "Gini",
        "Guiné-Bissau",
        "Gaiana",
        "Haiti",
        "Hondwras",
        "Hwngari",
        "Gwlad yr Iâ",
        "India",
        "Indonesia",
        "Iran",
        "Irac",
        "Iwerddon",
        "Israel",
        "Yr Eidal",
        "Jamaica",
        "Japan",
        "Iorddonen",
        "Kazakstan",
        "Kenya",
        "Kiribati",
        "Kuwait",
        "Kyrgyzstan",
        "Laos",
        "Latfia",
        "Libanus",
        "Lesotho",
        "Liberia",
        "Libia",
        "Liechtenstein",
        "Lithwania",
        "Lwcsembwrg",
        "Madagasgar",
        "Malawi",
        "Malaysia",
        "Maldives",
        "Mali",
        "Malta",
        "Ynysoedd Marshall",
        "Mauritania",
        "Mauritius",
        "Mecsico",
        "Micronesia",
        "Moldofa",
        "Monaco",
        "Mongolia",
        "Montenegro",
        "Moroco",
        "Mosambic",
        "Myanmar",
        "Namibia",
        "Nauru",
        "Nepal",
        "Yr Iseldiroedd",
        "Seland Newydd",
        "Nicaragwa",
        "Niger",
        "Nigeria",
        "Gogledd Corea",
        "Norwy",
        "Oman",
        "Pakistan",
        "Palau",
        "Panama",
        "Papua Guinea Newydd",
        "Paragwâi",
        "Periw",
        "Pilipinas",
        "Gwlad Pwyl",
        "Portiwgal",
        "Qatar",
        "Gweriniaeth y Congo",
        "Gweriniaeth Macedonia",
        "Rwmania",
        "Rwsia",
        "Rwanda",
        "Saint Kitts a Nevis",
        "Saint Lucia",
        "Saint Vincent a’r Grenadines",
        "Samoa",
        "San Marino",
        "São Tomé a Príncipe",
        "Saudi Arabia",
        "Sénégal",
        "Serbia",
        "Seychelles",
        "Sierra Leone",
        "Singapore",
        "Slofacia",
        "Slofenia",
        "Ynysoedd Solomon",
        "Somalia",
        "De Affrica",
        "De Corea",
        "De Sudan",
        "Sbaen",
        "Sri Lanka",
        "Sudan",
        "Swrinam",
        "Gwlad Swazi",
        "Sweden",
        "Y Swistir",
        "Syria",
        "Tajikistan",
        "Tanzania",
        "Gwlad Thai",
        "Togo",
        "Tonga",
        "Trinidad a Thobago",
        "Tunisia",
        "Twrci",
        "Turkmenistan",
        "Twfalw",
        "Uganda",
        "Wcráin",
        "Yr Emiradau Arabaidd Unedig",
        "Y Deyrnas Unedig",
        "Unol Daleithiau America",
        "Wrwgwái",
        "Uzbekistan",
        "Vanuatu",
        "Feneswela",
        "Fietnam",
        "Yemen",
        "Zambia",
        "Zimbabwe",
    ];

    const INTERNET_FREE_EMAIL_PROVIDER: &'static [&'static str] = &[
        "afal.example.cymru",
        "blodyn.example.cymru",
        "coeden.example.cymru",
        "draig.example.cymru",
        "ddraig.example.cymru",
        "enfys.example.cymru",
        "fâs.example.cymru",
        "ffidil.example.cymru",
        "gwely.example.cymru",
        "ngwely.example.cymru",
        "haul.example.cymru",
        "iâr.example.cymru",
        "jraff.example.cymru",
        "losin.example.cymru",
        "llyfr.example.cymru",
        "morfil.example.cymru",
        "neidr.example.cymru",
        "oren.example.cymru",
        "pabell.example.cymru",
        "phabell.example.cymru",
        "roced.example.cymru",
        "rhosyn.example.cymru",
        "seren.example.cymru",
        "telyn.example.cymru",
        "thelyn.example.cymru",
        "uncorn.example.cymru",
        "wyneb.example.cymru",
        "ynys.example.cymru",
    ];

    const INTERNET_DOMAIN_SUFFIX: &'static [&'static str] =
        &["cymru", "wales", "com", "net", "org"];

    // Source:
    // https://en.wikipedia.org/wiki/List_of_UK_dialling_codes_covering_Wales
    const PHONE_NUMBER_FORMATS: &'static [&'static str] = &[
        "01239 ######",
        "01244 ######",
        "01248 ######",
        "01267 ######",
        "01269 ######",
        "01286 ######",
        "01291 ######",
        "01341 ######",
        "01348 ######",
        "01352 ######",
        "01407 ######",
        "01437 ######",
        "01443 ######",
        "01446 ######",
        "01490 ######",
        "01492 ######",
        "01495 ######",
        "01497 ######",
        "01544 ######",
        "01545 ######",
        "01547 ######",
        "01550 ######",
        "01554 ######",
        "01558 ######",
        "01559 ######",
        "01570 ######",
        "01591 ######",
        "01597 ######",
        "01600 ######",
        "01633 ######",
        "01639 ######",
        "01646 ######",
        "01650 ######",
        "01654 ######",
        "01656 ######",
        "01678 ######",
        "01685 ######",
        "01686 ######",
        "01690 ######",
        "01691 ######",
        "01745 ######",
        "01758 ######",
        "01766 ######",
        "01792 ######",
        "01824 ######",
        "01834 ######",
        "01873 ######",
        "01874 ######",
        "01938 ######",
        "01948 ######",
        "01970 ######",
        "01974 ######",
        "01978 ######",
        "01982 ######",
        "01994 ######",
        "029 #### ######",
    ];

    // Source:
    // https://en.wikipedia.org/wiki/List_of_UK_dialling_codes_covering_Wales
    const PHONE_CELL_NUMBER_FORMATS: &'static [&'static str] = &[
        "01239 ######",
        "01244 ######",
        "01248 ######",
        "01267 ######",
        "01269 ######",
        "01286 ######",
        "01291 ######",
        "01341 ######",
        "01348 ######",
        "01352 ######",
        "01407 ######",
        "01437 ######",
        "01443 ######",
        "01446 ######",
        "01490 ######",
        "01492 ######",
        "01495 ######",
        "01497 ######",
        "01544 ######",
        "01545 ######",
        "01547 ######",
        "01550 ######",
        "01554 ######",
        "01558 ######",
        "01559 ######",
        "01570 ######",
        "01591 ######",
        "01597 ######",
        "01600 ######",
        "01633 ######",
        "01639 ######",
        "01646 ######",
        "01650 ######",
        "01654 ######",
        "01656 ######",
        "01678 ######",
        "01685 ######",
        "01686 ######",
        "01690 ######",
        "01691 ######",
        "01745 ######",
        "01758 ######",
        "01766 ######",
        "01792 ######",
        "01824 ######",
        "01834 ######",
        "01873 ######",
        "01874 ######",
        "01938 ######",
        "01948 ######",
        "01970 ######",
        "01974 ######",
        "01978 ######",
        "01982 ######",
        "01994 ######",
        "029 #### ######",
    ];

    const CURRENCY_NAME: &'static [&'static str] = &[
        "Dirham Emiradau Arabaidd Unedig",
        "Affganiaid",
        "Lek Albanaidd",
        "Dram Armenaidd",
        "Gilder Antilles yr Iseldiroedd",
        "Kwanza Angolaaidd",
        "Peso Ariannin",
        "Doler Awstralia",
        "Fflorin Aruba",
        "Manat Aserbaijanaidd",
        "Marc Bosniaidd",
        "Doler Barbados",
        "Taca Bengali",
        "Lev Bwlgaraidd newydd",
        "Dinar Bahrain",
        "Franco o Burundi",
        "Doler Bermuda",
        "Doler Brwnei",
        "Mvdol Bolifiaidd",
        "Real Brasil",
        "Doler y Bahamas",
        "Ngultrum o Bhutan",
        "Pula Botswana",
        "Rwbl Belarwsaidd",
        "Doler Belize",
        "Doler Canada",
        "Ffranc Congo",
        "Ffranc y Swistir",
        "Peso Chile",
        "Yuan",
        "Peso Colombiaidd",
        "Colon Costa Rica",
        "Peso Ciwbaidd",
        "Escwdo Cape Verde",
        "Coron Tsiec",
        "Ffranc Djibouti",
        "Coron Ddenmarc",
        "Peso Dominica",
        "Dinar Algeria",
        "Punt yr Aifft",
        "Nacfa Eritrea",
        "Cwrw Ethiopiaidd",
        "Ewro",
        "Doler Ffiji",
        "Punt Falkland",
        "Punt Prydeinig",
        "Lari Sioraidd",
        "Cedi Ghana",
        "Punt Gibraltar",
        "Gamby Dalasi",
        "Ffranc Gini",
        "Cwetzal Gwatemala",
        "Doler Guyana",
        "Doler Hong Kong",
        "Lempira Honduras",
        "Pwmpen Haiti",
        "Forint Hwngari",
        "Rupiah Indonesia",
        "Sicel Israel Newydd",
        "Rwpi Indiaidd",
        "Dinar Iracaidd",
        "Riyal Iranaidd",
        "Crona Gwlad yr Iâ",
        "Doler Jamaica",
        "Dinar Gwlad Iorddonen",
        "Yen",
        "Swllt Cenia",
        "Cirgisaidd ydw i",
        "Riel Cambodia",
        "Franco o'r Comoros",
        "Won Gogledd Corea",
        "Won De Corea",
        "Dinar Kuwaiti",
        "Doler Cayman",
        "Tenge Kazakh",
        "Kip Laosaidd",
        "Punt Libanaidd",
        "Rwpi Sinhala",
        "Doler Liberia",
        "Lotws Lesotho",
        "Dinar Libia",
        "Dirham Moroco",
        "Lew Moldafaidd",
        "Ariary Malagasy",
        "Dinar Macedonaidd",
        "Kyat Byrmanaidd",
        "Tugrik Mongolaidd",
        "Pataca Macao",
        "Ouguiya Mawritanaidd",
        "Rwpi Mauritius",
        "Rwfiyaa'r Maldives",
        "Kwacha Malawiaidd",
        "Peso Mecsicanaidd",
        "Ringgit Maleisia",
        "Metical Mosambicaidd",
        "Doler Namibia",
        "Naira Nigeria",
        "Cordoba Nicaragua",
        "Coron Norwyaidd",
        "Rwpi Nepal",
        "Doler Seland Newydd",
        "Rial Oman",
        "Balboa Panama",
        "Haul Periw Newydd",
        "Papuan China",
        "Peso Philipinaidd",
        "Rwpi Pacistanaidd",
        "Zloty Pwylaidd",
        "Guarani Paraguayaidd",
        "Rial Qatar",
        "Lew Rwmanaidd Newydd",
        "Dinar Serbia",
        "Rwbl Rwsiaidd",
        "Ffranc Rwanda",
        "Riyal Saudi",
        "Doler Solomon",
        "Rwpi Seychelles",
        "Punt Swdanaidd",
        "Coron Sweden",
        "Doler Singapore",
        "Punt Sant Helena",
        "Llew Sierra Leone",
        "Swllt Somalia",
        "Doler Swrinam",
        "Punt De Swdan",
        "Dobra o São Tomé a Príncipe",
        "Punt Syriaidd",
        "Lilangeni o eSwatini",
        "Baht Thai",
        "Somoni Tajicaidd",
        "Manat Tyrcmeneg",
        "Dinar Tiwnisia",
        "Paʻanga Tongaidd",
        "Lira Twrcaidd Newydd",
        "Doler Trinidad a Tobago",
        "Doler Taiwan Newydd",
        "Swllt Tansanïaidd",
        "Hryvnia Wcreinaidd",
        "Swllt Wganda",
        "Doler yr Unol Daleithiau",
        "Peso Wrwgwái",
        "Wsbeceg ydw i",
        "Bolivar Feneswelaidd",
        "Dong",
        "Vatu",
        "Tālā Samoaidd",
        "Doler Dwyrain y Caribî",
        "Riyal Yemeni",
        "Rand De Affrica",
        "Kwacha Sambia",
        "Doler Simbabwe",
    ];

    const JOB_SENIORITY: &'static [&'static str] = &["Uwch", "Iau", "Bos"];

    const JOB_FIELD: &'static [&'static str] = &[
        "Marchnata",
        "Gwybodeg",
        "Cyfrifeg",
        "Gweinyddiaeth",
        "Hysbysebu",
        "Cyllid",
        "Gwasanaethau Cymunedol",
        "Adeiladau",
        "Dylunio",
        "Addysg",
        "Amaethyddiaeth",
        "Llywodraeth",
        "Iechyd",
        "Gwestyaria",
        "Cyfreithiol",
        "Diwydiant",
        "Mwyngloddio",
        "Eiddo Tiriog",
        "Gwerthiannau",
        "Technoleg",
    ];

    const JOB_POSITION: &'static [&'static str] = &[
        "Arbenigwr",
        "Athro",
        "Cofrestrydd",
        "Cydlynydd",
        "Cyfarwyddwr",
        "Cynhyrchydd",
        "Cynorthwyydd",
        "Cynrychiolydd",
        "Dadansoddwr",
        "Darlithydd",
        "Derbynnydd",
        "Dylunydd",
        "Goruchwyliwr",
        "Gweinyddwr",
        "Is-Ganghellor",
        "Peiriannydd",
        "Pensaer",
        "Porthor",
        "Rheolwr",
        "Swyddog",
        "Technegydd",
        "Tiwtor",
        "Ymchwilydd",
        "Ymgynghorydd",
    ];

    const JOB_TITLE_TPL: &'static str = "{Position} {Seniority} {Field}";

    const COMPANY_NAME_TPLS: &'static [&'static str] =
        &["{Name_1} {Suffix}", "{Name_1} e {Name_2} {Suffix}"];
    const COMPANY_BUZZWORD_HEAD: &'static [&'static str] = &[
        "Addasadwy",
        "Uwch",
        "Awtomataidd",
        "Canolog",
        "Cydnaws",
        "Ffurfweddadwy",
        "Digidol",
        "Dosbarthwyd",
        "Amrywiol",
        "Unigryw",
        "Ehangadwy",
        "Ffocws",
        "Sylfaenol",
        "Arloesol",
        "Integredig",
        "Greddfol",
        "Wedi'i optimeiddio",
        "Wedi'i drefnu",
        "Blaengar",
        "Cadarn",
        "Amryddawn",
        "Rhithwir",
    ];

    const COMPANY_BUZZWORD_MIDDLE: &'static [&'static str] = &[
        "24 awr",
        "byd-eang",
        "digidol",
        "deinamig",
        "busnes",
        "effeithlon",
        "hyblyg",
        "integredig",
        "arloesol",
        "deallus",
        "symudol",
        "rhagweithiol",
        "proffesiynol",
        "rhanbarthol",
        "ymatebol",
        "cynaliadwy",
        "technolegol",
    ];

    const COMPANY_BUZZWORD_TAIL: &'static [&'static str] = &[
        "datrysiadau",
        "gwasanaethau",
        "systemau",
        "strategaethau",
        "technolegau",
        "methodolegau",
        "pensaernïaeth",
        "seilwaith",
        "ceisiadau",
        "rhyngwynebau",
        "mentrau",
        "llwyfannau",
        "arloesedd",
        "paradigm",
        "gweledigaeth",
    ];

    const COMPANY_CATCH_PHASE_TPL: &'static str = "{Head} {Middle} {Tail}";

    const COMPANY_BS_VERBS: &'static [&'static str] = &[
        "gweithredu",
        "defnyddio",
        "integreiddio",
        "optimeiddio",
        "esblygu",
        "trawsnewid",
        "i arloesi",
        "gwneud y mwyaf o",
        "i gryfhau",
        "ehangu",
    ];

    const COMPANY_BS_ADJ: &'static [&'static str] = &[
        "effeithlon",
        "rhagweithiol",
        "cadarn",
        "chwyldroadol",
        "graddadwy",
        "arloesol",
        "reddfol",
        "strategol",
        "integredig",
        "digidol",
        "deinamig",
        "byd-eang",
    ];

    const COMPANY_BS_NOUNS: &'static [&'static str] = &[
        "synergeddau",
        "datrysiadau",
        "seilweithiau",
        "llwyfannau",
        "mentrau",
        "cymuned",
        "technolegau",
        "methodolegau",
    ];
}

impl CityNameGenFn for CY_GB {}
