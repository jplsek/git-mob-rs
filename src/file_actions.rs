use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub trait FileActions {
    fn write(&self, path: &Path, s: &str) -> Result<(), String>;
    fn read(&self, path: &Path) -> Result<String, String>;
}

pub struct FileSystemActions();

impl FileActions for FileSystemActions {
    fn write(&self, path: &Path, s: &str) -> Result<(), String> {
        let path_display = path.display();
        if let Err(why) = fs::write(path, s.as_bytes()) {
            return Err(format!("couldn't write to {path_display}: {why}"));
        }
        Ok(())
    }

    fn read(&self, path: &Path) -> Result<String, String> {
        let path_display = path.display();
        let mut file = match File::open(path) {
            Err(why) => return Err(format!("couldn't open {path_display}: {why}")),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => Err(format!("couldn't read {path_display}: {why}")),
            Ok(_) => Ok(s),
        }
    }
}
