use clap::{value_parser, Arg, Command};

mod core;
mod system;

fn main() {
    let cmd = Command::new("webprs")
        .about("image handler for webp")
        .subcommand_required(true)
        .subcommand(
            Command::new("convert")
                .about("Convert all images in a folder to webp")
                .arg(
                    Arg::new("destination")
                        .required(false)
                        .default_missing_value("destination")
                        .help("Path to folder with images")
                        .short('d'),
                )
                .arg(
                    Arg::new("quality")
                        .required(false)
                        .default_missing_value("100.0")
                        .short('q')
                        .value_parser(value_parser!(f32)),
                )
                .arg(
                    Arg::new("threads")
                        .required(false)
                        .default_missing_value("5")
                        .short('t')
                        .value_parser(value_parser!(i16)),
                ),
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("convert", matches)) => core::commands::convert_command(Some(matches)),
        _ => {
            println!("No command found");
            std::process::exit(1);
        }
    };
}
