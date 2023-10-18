use clap::ArgMatches;

pub fn convert_command(cmd: Option<&ArgMatches>) {
    let matches = cmd.unwrap();
    let destination_arg = matches.get_one::<String>("destination");

    let current_path = std::env::current_exe().expect("Error getting current path");
    let current_path = current_path
        .parent()
        .expect("Error getting current path parent");
    let current_path = current_path
        .to_str()
        .expect("Error converting path to string");

    let converted_count =
        crate::core::convert::convert_folder_images_to_webp(current_path, destination_arg)
            .expect("Error converting images to webp");

    println!("Converted {} images", converted_count);
}
