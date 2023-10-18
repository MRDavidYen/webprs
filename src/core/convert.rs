use std::path::Path;

pub fn convert_folder_images_to_webp(
    path_arg: &str,
    destination_arg: Option<&String>,
) -> Result<i64, String> {
    let default_desination = String::from("converted");
    let destination = destination_arg.unwrap_or(&default_desination);

    let images = crate::system::folder::reading_all_images(path_arg);

    if images.len() == 0 {
        return Err("No images found".to_string());
    }

    let converted_folder =
        match crate::system::folder::create_folder_on_root_folder(path_arg, destination) {
            Ok(path) => path,
            Err(err) => return Err(err),
        };

    for img_in_folder in &images {
        let img = match image::open(&img_in_folder) {
            Ok(img) => img,
            Err(_) => {
                println!("Error opening image Path:{0}", img_in_folder);
                continue;
            }
        };

        let webp_img = crate::system::files::convert_to_webp(img);
        let image_name = Path::new(&img_in_folder)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let image_path = match crate::system::folder::get_file_path_without_root_folder_and_filename(
            &img_in_folder,
        ) {
            Some(path) => format!("{}/{}", converted_folder, path),
            None => continue,
        };
        let image_location = format!("{}/{}.webp", image_path, image_name);

        match std::fs::create_dir_all(&image_path) {
            Ok(_) => {}
            Err(_) => {
                println!("Folder already exists");
            }
        }

        println!("Converting image: {}", img_in_folder);
        std::fs::write(image_location, &*webp_img).unwrap();
    }

    Ok(images.len() as i64)
}
