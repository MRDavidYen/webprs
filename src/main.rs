use clap::Command;
use folder_webp::files;
use folder_webp::folder;

fn main() {
    let cmd = Command::new("fwebp")
        .about("Convert all images in a folder to webp")
        .subcommand_required(true)
        .subcommand(
            Command::new("convert")
                .about("Convert all images in a folder to webp")
                .arg(
                    clap::Arg::new("folder")
                        .required(true)
                        .index(1),
                ),
        );

    cmd.get_matches();

    let images = folder::reading_all_images("examples");
    folder::create_folder_on_root_folder("examples", "converted").unwrap();

    for img_in_folder in images {
        let img = match image::open(&img_in_folder) {
            Ok(img) => img,
            Err(_) => {
                println!("Error opening image Path:{0}", img_in_folder);
                continue;
            }
        };
        let webp_img = files::convert_to_webp(img);

        let image_extension = match img_in_folder.split('.').last() {
            Some(ext) => ext,
            None => {
                println!("Error getting image extension Path:{0}", img_in_folder);
                continue;
            }
        };
        let mut image_path = img_in_folder.clone();
        image_path = image_path.replace(image_extension, "webp");

        match std::fs::create_dir_all(img_in_folder) {
            Ok(_) => {
                println!("Created folder");
            }
            Err(_) => {
                println!("Folder already exists");
            }
        }
        std::fs::write(image_path, &*webp_img).unwrap();
    }
}
