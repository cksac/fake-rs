use clap::{command, value_parser, Arg};
use rand::Rng;
use std::io::{self, Write};

mod fake_gen;

const AVAILABLE_LOCALES: [&str; 8] = [
    "en", "fr_fr", "zh_tw", "zh_cn", "ja_jp", "ar_sa", "pt_br", "pt_pt", "de_de",
];

pub use fake_gen::{all_fakegen_commands, AVAILABLE_LOCALES};
pub fn main() {
    let stdout = io::stdout();
    let mut buf_stdout = io::BufWriter::new(stdout);

    let mut thread_rng = rand::rng();
    let args = cli_parser();

    writeln!(
        buf_stdout,
        "Generating {} fakes for {:?} locale",
        args.0.repeats, args.0.locale
    )
    .unwrap();

    (0..args.0.repeats).for_each(|_| writeln!(buf_stdout, "{}", args.1(&mut thread_rng)).unwrap());
}

impl TryFrom<&str> for AVAILABLE_LOCALES {
    type Error = String;
    fn try_from(str_val: &str) -> Result<Self, Self::Error> {
        let str_val = str_val.to_lowercase();
        let variant =  match str_val.as_str(){
            "en" => AVAILABLE_LOCALES::EN,
            "fr_fr" => AVAILABLE_LOCALES::FR_FR,
            "zh_tw" => AVAILABLE_LOCALES::ZH_TW,
            "zh_cn" => AVAILABLE_LOCALES::ZH_CN,
            "ja_jp" => AVAILABLE_LOCALES::JA_JP,
            "ar_sa" => AVAILABLE_LOCALES::AR_SA,
            "pt_br" => AVAILABLE_LOCALES::PT_BR,
            "pt_pt" => AVAILABLE_LOCALES::PT_PT,
            "de_de" => AVAILABLE_LOCALES::DE_DE,
            _=> return Err(format!("{} is either an invalid locale or not yet supported.\n The supported locales are: {:?}",str_val,AVAILABLE_LOCALES))
        };
        Ok(variant)
    }
}

fn cli_parser<R: Rng>() -> (Args, impl Fn(&mut R) -> String) {
    let (subcommands, fake_generator) = all_fakegen_commands::<R>();
    let mut command = command!()
        .arg(
            Arg::new("repeat")
                .long("repeat")
                .short('r')
                .default_value("1")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("locale")
                .short('l')
                .long("locale")
                .default_value("EN")
                .value_parser(|value: &str| AVAILABLE_LOCALES::try_from(value)),
        )
        .subcommands(subcommands)
        .arg_required_else_help(true);
    let help_message = command.render_help();
    let matches = command.get_matches();
    let repeats = *matches.get_one::<u32>("repeat").unwrap();
    let locale = matches
        .get_one::<AVAILABLE_LOCALES>("locale")
        .unwrap()
        .to_owned();

    let fake_gen = fake_generator(matches, locale, help_message);
    (Args { repeats, locale }, fake_gen)
}

struct Args {
    repeats: u32,
    locale: AVAILABLE_LOCALES,
}
