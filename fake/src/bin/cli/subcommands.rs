use clap::{value_parser, Arg, Command};
macro_rules! generate_command {
    ($fake:literal) => {
        Command::new($fake)
    };
    ($fake:literal, range, $min:literal, $max:literal) => {
        Command::new($fake)
            .arg(
                Arg::new("max")
                    .long("max")
                    .default_value($max)
                    .value_parser(value_parser!(usize)),
            )
            .arg(
                Arg::new("min")
                    .long("min")
                    .default_value($min)
                    .value_parser(value_parser!(usize)),
            )
    };
}

pub fn all_fakegen_commands() -> Vec<Command> {
    vec![
        generate_command!("Name"),
        generate_command!("FirstName"),
        generate_command!("CityPrefix"),
        generate_command!("Password", range, "10", "20"),
    ]
}
