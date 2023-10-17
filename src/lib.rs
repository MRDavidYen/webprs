pub mod folder {
    use std::path::Path;

    const IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

    pub fn reading_all_images<P>(path: P) -> Vec<String>
    where
        P: AsRef<Path>,
    {
        let mut images = Vec::new();

        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let path = path.to_str().unwrap().to_string();

            if IMAGE_EXTENSIONS.contains(&path.split('.').last().unwrap()) {
                images.push(path);
            }
        }

        println!("{:?}", images);

        images
    }

    pub fn create_folder_on_root_folder<P>(path: P, folder_name: &str) -> Result<bool, String>
    where
        P: AsRef<Path>,
    {
        let path_str = path.as_ref().to_str().unwrap();
        let root_folder = path_str.split("/").next().unwrap();

        if root_folder == folder_name {
            return Err("Folder already exists".to_string());
        }

        match std::fs::create_dir_all(format!("{}/{}", path_str, folder_name)) {
            Ok(_) => Ok(true),
            Err(_) => Err("Folder already exists".to_string()),
        }
    }
}

pub mod files {
    use webp::Encoder;

    pub fn convert_to_webp(img: image::DynamicImage) -> webp::WebPMemory {
        let encoder = Encoder::from_image(&img).unwrap();
        let webp_img = encoder.encode(100.0);

        webp_img
    }
}
