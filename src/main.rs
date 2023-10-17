use folder_webp::files;
use folder_webp::folder;

fn main() {
    let images = folder::reading_all_images("examples");

    for img_in_folder in images {
        let img = image::open(&img_in_folder).unwrap();
        let webp_img = files::convert_to_webp(img);

        let image_extension = img_in_folder.split('.').last().unwrap();
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
