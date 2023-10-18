use clap::Command;
use folder_webp::folder;
use std::path::Path;

fn main() {
    // let cmd = Command::new("fwebp")
    //     .about("Convert all images in a folder to webp")
    //     .subcommand_required(true)
    //     .subcommand(
    //         Command::new("convert")
    //             .about("Convert all images in a folder to webp")
    //             .arg(clap::Arg::new("folder").required(true).index(1)),
    //     );

    // cmd.get_matches();

    let images = folder::reading_all_images("resource");
    let converted_folder = folder::create_folder_on_root_folder("resource", "converted")
        .expect("unable to create folder, maybe it already exists");

    for img_in_folder in images {
        let img = match image::open(&img_in_folder) {
            Ok(img) => img,
            Err(_) => {
                println!("Error opening image Path:{0}", img_in_folder);
                continue;
            }
        };

        let webp_img = files::convert_to_webp(img);
        let image_name = Path::new(&img_in_folder)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let image_path =
            match folder::get_file_path_without_root_folder_and_filename(&img_in_folder) {
                Some(path) => format!("{}/{}", converted_folder, path),
                None => continue,
            };
        let image_location = format!("{}/{}.webp", image_path, image_name);

        match std::fs::create_dir_all(&image_path) {
            Ok(_) => {
                println!("Created folder");
            }
            Err(_) => {
                println!("Folder already exists");
            }
        }
        std::fs::write(image_location, &*webp_img).unwrap();
    }
}
