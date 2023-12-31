use std::path::Path;

const IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

pub fn reading_all_images<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut images = Vec::new();

    for entry in std::fs::read_dir(path).expect("Error reading folder") {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        let path_is_folder = std::fs::metadata(&path).unwrap().is_dir();

        if path_is_folder {
            images.append(&mut reading_all_images(&path_str));
            continue;
        }

        if IMAGE_EXTENSIONS.contains(&path_str.split('.').last().unwrap().to_lowercase().as_str()) {
            images.push(path_str);
        }
    }

    images
}

pub fn create_folder_on_root_folder<P>(path: P, folder_name: &str) -> Result<String, String>
where
    P: AsRef<Path>,
{
    let path_str = path.as_ref().to_str().unwrap();
    let root_folder = path_str.split("/").next().unwrap();

    if root_folder == folder_name {
        return Err("Folder already exists".to_string());
    }

    let mut path = String::from(path_str);
    path.push(char::from('/'));
    path.push_str(folder_name);

    match std::fs::create_dir_all(format!("{}/{}", path_str, folder_name)) {
        Ok(_) => Ok(path),
        Err(_) => Err("Folder already exists".to_string()),
    }
}

pub fn get_file_path_without_root_folder_and_filename<P>(path: &P) -> Option<String>
where
    P: AsRef<Path>,
{
    let application_path = std::env::current_dir().expect("Error getting current path");

    let relative_path = match path.as_ref().strip_prefix(application_path) {
        Ok(path) => path,
        Err(_) => return None,
    };
    let relative_path = match relative_path.parent() {
        Some(path) => path,
        None => return None,
    };

    let path_str = match relative_path.to_str() {
        Some(path) => path,
        None => return None,
    };

    let path = String::from(path_str);

    Some(path)
}
