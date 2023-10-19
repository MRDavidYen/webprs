use clap::ArgMatches;

pub fn convert_command(cmd: Option<&ArgMatches>) {
    let matches = cmd.unwrap();
    let destination_arg = matches.get_one::<String>("destination");
    let quality_arg = matches.get_one::<f32>("quality").unwrap_or(&100.0);
    let threads_arg = matches.get_one::<i16>("threads").unwrap_or(&5);

    let convert_options = crate::core::convert::ConvertOptions {
        quality: *quality_arg,
        threads: *threads_arg,
    };

    let time_spent = std::time::Instant::now();

    let current_path = std::env::current_dir().expect("Error getting current path");
    let current_path = current_path
        .to_str()
        .expect("Error converting path to string");

    println!("Current path: {}", current_path);

    let converted_count = crate::core::convert::convert_folder_images_to_webp(
        current_path,
        destination_arg,
        &convert_options,
    )
    .expect("Error converting images to webp");

    println!("Converted {} images", converted_count);
    println!("Time spent: {:?}", time_spent.elapsed());
}
