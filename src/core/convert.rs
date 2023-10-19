use core::panic;
use std::{path::Path, thread};

#[derive(Clone)]
pub struct ConvertOptions {
    pub quality: f32,
    pub threads: i16,
}

pub fn convert_folder_images_to_webp(
    path_arg: &str,
    destination_arg: Option<&String>,
    convert_options: &ConvertOptions,
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

    let images_len = images.len();

    if images_len > 20 {
        split_images_into_threads(&images, &converted_folder, convert_options)
    } else {
        convert_webp_thread(&images, &converted_folder, convert_options);
    }

    Ok(images_len as i64)
}

fn split_images_into_threads(
    images: &Vec<String>,
    converted_folder: &String,
    convert_options: &ConvertOptions,
) {
    if convert_options.threads > images.len() as i16 {
        panic!("Threads count is bigger than images count");
    }

    let images_vec = split_images_vec(images, convert_options.threads);
    let mut threads = Vec::new();

    for part in images_vec {
        let converted_folder = converted_folder.clone();
        let options = convert_options.clone();

        let t = thread::spawn(move || {
            convert_webp_thread(&part, &converted_folder, &options);
        });

        threads.push(t);
    }

    // Wait for all threads to complete
    for t in threads {
        t.join().unwrap();
    }
}

fn split_images_vec(images: &Vec<String>, split_count: i16) -> Vec<Vec<String>> {
    let mut images_vec = Vec::new();
    let mut images = images.clone();

    let images_len = images.len();
    let split_count = split_count as usize;

    if images_len < split_count {
        images_vec.push(images);
        return images_vec;
    }

    let slice_size = (images_len + split_count - 1) / split_count;

    for _ in 0..split_count {
        let mut slice = Vec::new();

        for _ in 0..slice_size {
            if images.len() == 0 {
                break;
            }

            slice.push(images.remove(0));
        }

        images_vec.push(slice);
    }

    images_vec
}

fn convert_webp_thread(
    sub_imgs: &Vec<String>,
    converted_folder: &String,
    convert_options: &ConvertOptions,
) {
    for img_in_folder in sub_imgs {
        let img = match image::open(&img_in_folder) {
            Ok(img) => img,
            Err(_) => {
                println!("Error opening image Path:{0}", img_in_folder);
                continue;
            }
        };

        let webp_img = crate::system::files::convert_to_webp(img, &convert_options.quality);
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
}
