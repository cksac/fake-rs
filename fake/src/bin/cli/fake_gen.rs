use clap::{builder::StyledStr, ArgMatches};
use fake::{faker, Fake};
use rand::Rng;
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum AVAILABLE_LOCALES {
    EN,
    FR_FR,
    ZH_TW,
    ZH_CN,
    JA_JP,
    AR_SA,
    PT_BR,
    PT_PT,
    DE_DE,
    IT_IT,
    CY_GB,
    NL_NL,
    TR_TR,
}

macro_rules! fake_gen_on_return_type {
    ($s:ident,$rng:ident,Vec) => {
        format!("{:?}", $s.fake_with_rng::<Vec<String>, _>($rng))
    };
    ($s:ident,$rng:ident) => {
        $s.fake_with_rng($rng)
    };
}

macro_rules! some_rules {
    ($locale:expr, $module:ident, $fake:ident($($arg:ident)?)$(,$return_type:ident)?) => {

        match $locale {
            AVAILABLE_LOCALES::EN => {
                let s = faker::$module::en::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::FR_FR => {
                let s = faker::$module::fr_fr::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::ZH_TW => {
                let s = faker::$module::zh_tw::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::ZH_CN => {
                let s = faker::$module::zh_cn::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::AR_SA => {
                let s = faker::$module::ar_sa::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::JA_JP => {
                let s = faker::$module::ja_jp::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::PT_BR => {
                let s = faker::$module::pt_br::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::PT_PT => {
                let s = faker::$module::pt_pt::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::DE_DE => {
                let s = faker::$module::de_de::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::IT_IT => {
                let s = faker::$module::it_it::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::CY_GB => {
                let s = faker::$module::cy_gb::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::NL_NL => {
                let s = faker::$module::nl_nl::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
            AVAILABLE_LOCALES::TR_TR => {
                let s = faker::$module::nl_nl::$fake($($arg)?);
                Box::new(move |rng: &mut R| fake_gen_on_return_type!(s,rng$(,$return_type)?))
            }
        }
    };
}

use clap::{value_parser, Arg, Command};
macro_rules! generate_command {
    ($fake:literal) => {
        Command::new($fake)
    };
    ($fake:ident) => {
        Command::new(stringify!($fake))
    };
    ($fake:ident($($arg:ident: $type:ty=$default:literal),+)) => {
        Command::new(stringify!($fake))$(.arg(
            Arg::new(stringify!($arg))
                .long(stringify!($arg))
                .default_value(stringify!($default))
                .value_parser(value_parser!($type))))+

    };
}

macro_rules! right_arm {
    ($locale:ident, $module:ident, $fake:ident) => {
        some_rules!($locale, $module, $fake())
    };
    ($locale:ident, $module:ident, $fake:ident($arg:ident: u8),$sub_matches:ident) => {{
        let value = *$sub_matches.get_one::<u8>(stringify!($arg)).unwrap();
        some_rules!($locale, $module, $fake(value))
    }};

    ($locale:ident, $module:ident, $fake:ident(min: usize, max: usize),$sub_matches:ident$(,$return_type:ident)?) => {{
        let min = *$sub_matches.get_one::<usize>("min").unwrap();
        let max = *$sub_matches.get_one::<usize>("max").unwrap();
        let range = min..max;
        some_rules!($locale, $module, $fake(range)$(,$return_type)?)
    }};
}

macro_rules! fakegen_commands {
    ($(($fake:ident$(($($arg:ident: $arg_type:tt=$arg_default:literal),+))?$(->$return_type:ident)?,$module:ident)),+) => {
        (
            vec![$(generate_command!($fake$(($($arg:$arg_type=$arg_default),+))?)),+],
            |arg_matches:ArgMatches, locale:AVAILABLE_LOCALES, help_message:StyledStr| {
                match arg_matches.subcommand(){
                    $(Some((stringify!($fake),_sub_matches))=>{
                        right_arm!(locale, $module, $fake$(($($arg: $arg_type),+),_sub_matches)?$(,$return_type)?)
                    })+
                    _ => {
                        println!("Didn't receive subcommand\n {}", help_message);
                        std::process::exit(0)
                    }
                }
            }

        )
    };
}

pub fn all_fakegen_commands<R: Rng>() -> (
    Vec<Command>,
    impl Fn(ArgMatches, AVAILABLE_LOCALES, StyledStr) -> Box<dyn Fn(&mut R) -> String>,
) {
    fakegen_commands!(
        //address
        (CityPrefix, address),
        (CitySuffix, address),
        (CityName, address),
        (CountryName, address),
        (CountryCode, address),
        (StreetSuffix, address),
        (StreetName, address),
        (TimeZone, address),
        (StateName, address),
        (StateAbbr, address),
        (SecondaryAddressType, address),
        (SecondaryAddress, address),
        (ZipCode, address),
        (PostCode, address),
        (BuildingNumber, address),
        (Latitude, address),
        (Longitude, address),
        (Geohash(precision:u8=1),address),

        //barcode
        (Isbn,barcode),
        (Isbn10,barcode),
        (Isbn13,barcode),

        //creditcard
        (CreditCardNumber,creditcard),

        //company
        (CompanySuffix, company),
        (CompanyName, company),
        (Buzzword, company),
        (BuzzwordMiddle, company),
        (BuzzwordTail, company),
        (CatchPhrase, company),
        (BsVerb, company),
        (BsAdj, company),
        (BsNoun, company),
        (Bs, company),
        (Profession, company),
        (Industry, company),

        //internet
        (FreeEmailProvider,internet),
        (DomainSuffix,internet),
        (FreeEmail,internet),
        (SafeEmail,internet),
        (Username,internet),
        (Password(min:usize=10, max:usize=20),internet),
        (IPv4,internet),
        (IPv6,internet),
        (IP,internet),
        (MACAddress,internet),
        (UserAgent,internet),

        //job
        (Seniority,job),
        (Field,job),
        (Position,job),

        //lorem
        (Word,lorem),
        (Words(min:usize=5, max:usize=10)->Vec,lorem),
        (Sentence(min:usize=5, max:usize=10),lorem),
        (Sentences(min:usize=5, max:usize=10)->Vec,lorem),
        (Paragraph(min:usize=5, max:usize=10),lorem),
        (Paragraphs(min:usize=5, max:usize=10)->Vec,lorem),

        //markdown
        (ItalicWord,markdown),
        (BoldWord,markdown),
        (Link,markdown),
        (BulletPoints(min:usize=5, max:usize=10)->Vec,markdown),
        (ListItems(min:usize=5, max:usize=10)->Vec,markdown),
        (BlockQuoteSingleLine(min:usize=5, max:usize=10),markdown),
        (BlockQuoteMultiLine(min:usize=5, max:usize=10)->Vec,markdown),
        (Code(min:usize=5, max:usize=10),markdown),

        //name
        (FirstName,name),
        (LastName,name),
        (Title,name),
        (Suffix,name),
        (Name,name),
        (NameWithTitle,name),

        //phone_number
        (PhoneNumber,phone_number),
        (CellNumber,phone_number),

        //filesystem
        (FilePath,filesystem),
        (FileName,filesystem),
        (FileExtension,filesystem),
        (DirPath,filesystem),
        (MimeType,filesystem),
        (Semver,filesystem),
        (SemverStable,filesystem),
        (SemverUnstable,filesystem),

        //currency
        (CurrencyCode,currency),
        (CurrencyName,currency),
        (CurrencySymbol,currency),

        //finance
        (Bic,finance),
        (Isin,finance),

        //color
        (HexColor,color),
        (RgbColor,color),
        (RgbaColor,color),
        (HslColor,color),
        (HslaColor,color),
        (Color,color),

        //chrono
        (Time,chrono),
        (Date,chrono),
        (DateTime,chrono),

        //http
        (RfcStatusCode,http),
        (ValidStatusCode,http)

    )
}
