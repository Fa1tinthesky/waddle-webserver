use std::{
    collections::HashMap,
    fs, path::Path
};

pub struct Router {
}

impl Router {
    pub fn new() -> Router {
        return Router {}
    }

    pub fn get_static_file(&self, filename: &str) -> Option<String> {
        let string = String::from("./public/");
        find_staticf(Path::new(&string), Path::new(filename))
    }
}


fn find_staticf(dir: &Path, filename: &Path) -> Option<String> {
    fn get_basename(path: &Path) -> &str {
        return path.file_name().unwrap().to_str().unwrap();
    }

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            if path.is_file() && get_basename(&path) == get_basename(&filename) {
                println!("I got here ___");
                return Some(path.to_string_lossy().into_owned());
            } else if path.is_dir() {
                if let Some(found) = find_staticf(&path, filename) {
                    return Some(found);
                }
            }
        }
    }

    None
}

