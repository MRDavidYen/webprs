use clap::{Arg, Command};

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
                        .help("Path to folder with images")
                        .short('d'),
                )
                .arg_required_else_help(true),
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
