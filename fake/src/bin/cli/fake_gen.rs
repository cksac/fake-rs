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
}

macro_rules! some_rules {
    ($locale:expr, $module:ident, $fake:ident($($arg:ident)?)) => {

        match $locale {
            AVAILABLE_LOCALES::EN => {
                let s = faker::$module::en::$fake($($arg)?);
                Box::new(move |rng: &mut R| s.fake_with_rng(rng))

            }
            AVAILABLE_LOCALES::FR_FR => {
                let s = faker::$module::fr_fr::$fake($($arg)?);
                Box::new(move |rng: &mut R| s.fake_with_rng(rng))

            }
            AVAILABLE_LOCALES::ZH_TW => {
                let s = faker::$module::zh_tw::$fake($($arg)?);
                Box::new(move |rng: &mut R| s.fake_with_rng(rng))

            }
            AVAILABLE_LOCALES::ZH_CN => {
                let s = faker::$module::zh_cn::$fake($($arg)?);
                Box::new(move |rng: &mut R| s.fake_with_rng(rng))

            }
            AVAILABLE_LOCALES::AR_SA => {
                let s = faker::$module::ar_sa::$fake($($arg)?);
                Box::new(move |rng: &mut R| s.fake_with_rng(rng))

            }
            AVAILABLE_LOCALES::JA_JP => {
                let s = faker::$module::ja_jp::$fake($($arg)?);
                Box::new(move |rng: &mut R| s.fake_with_rng(rng))

            }
            AVAILABLE_LOCALES::PT_BR => {
                let s = faker::$module::pt_br::$fake($($arg)?);
                Box::new(move |rng: &mut R| s.fake_with_rng(rng))

            }
        }
    };
}

pub fn fake_generator<R>(
    matches: ArgMatches,
    locale: AVAILABLE_LOCALES,
    help_message: StyledStr,
) -> Box<dyn Fn(&mut R) -> String>
where
    R: Rng,
{
    match matches.subcommand() {
        Some(("FirstName", _)) => {
            some_rules!(locale, name, FirstName())
        }
        Some(("Name", _)) => {
            some_rules!(locale, name, Name())
        }
        Some(("CityPrefix", _)) => {
            some_rules!(locale, address, CityPrefix())
        }
        Some(("Password", matches)) => {
            let min = *matches.get_one::<usize>("min").unwrap();
            let max = *matches.get_one::<usize>("max").unwrap();
            let range = min..max;
            some_rules!(locale, internet, Password(range))
        }
        _ => {
            println!("Didn't receive subcommand\n {}", help_message);
            std::process::exit(0)
        }
    }
}
