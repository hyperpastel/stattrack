use clap::{Arg, ArgAction, Command, ValueHint};

pub fn build_cli() -> Command {
    Command::new("stattrack")
        .about("Pomodoro app")
        .arg(
            Arg::new("work-time")
                .short('w')
                .long("work-time")
                .action(ArgAction::Set)
                .default_value("25")
                .value_hint(ValueHint::Other)
                .value_parser(clap::value_parser!(u64))
                .help("Sets the working time interval"),
        )
        .arg(
            Arg::new("break-time")
                .short('b')
                .long("break-time")
                .action(ArgAction::Set)
                .default_value("5")
                .value_hint(ValueHint::Other)
                .value_parser(clap::value_parser!(u64))
                .help("Set the break time interval"),
        )
        .arg(
            Arg::new("project-name")
                .short('p')
                .long("project-name")
                .action(ArgAction::Set)
                .required(true)
                .help("Name of the project for which to track time"),
        )
}

