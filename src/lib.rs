use dirs::{config_dir, home_dir};
use git2::Config;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub trait FileActions {
    fn write(&self, path: &Path, s: String);
    fn read(&self, path: &Path) -> String;
}

pub struct GmFileActions();

// Mostly here for unit testing
impl FileActions for GmFileActions {
    fn write(&self, path: &Path, s: String) {
        if let Err(why) = fs::write(path, s.as_bytes()) {
            panic!("couldn't write to {}: {}", path.display(), why)
        }
    }

    fn read(&self, path: &Path) -> String {
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
            Ok(_) => s,
        }
    }
}

pub struct GitMob {
    pub file_actions: Box<dyn FileActions>,
}

impl Default for GitMob {
    fn default() -> Self {
        Self::new()
    }
}

impl GitMob {
    pub fn new() -> GitMob {
        GitMob {
            file_actions: Box::from(GmFileActions()),
        }
    }

    pub fn get_gitmessage_path<'a>(&self) -> &'a Path {
        Path::new(".git/.gitmessage")
    }

    pub fn write_gitmessage(&self, s: String) {
        let gitmessage_path = self.get_gitmessage_path();

        self.file_actions
            .write(&gitmessage_path, format!("\n\n{}", s));
    }

    /// Returns the coauthors path
    ///
    /// This supports both xdg (prioritized) or if the config is in the home directory (like
    /// git-mob).
    pub fn get_coauthors_path(&self) -> PathBuf {
        let file_name = "git-coauthors";

        // most likely on fresh install after first use
        let mut coauthors_path = config_dir().unwrap();
        coauthors_path.push(file_name);
        if coauthors_path.exists() {
            return coauthors_path;
        }

        // else check home dir - if it doesn't exist (like a fresh install) use xdg instead
        let mut home_coauthors_path = home_dir().unwrap();
        home_coauthors_path.push(format!(".{}", file_name));
        if home_coauthors_path.exists() {
            home_coauthors_path
        } else {
            coauthors_path
        }
    }

    pub fn get_git_user(&self) -> String {
        let cfg = Config::open_default().unwrap();

        // these errors should only really happen in ci
        let c = "user.name";
        let user = match cfg.get_string(c) {
            Err(_) => {
                println!("Warning: your git config \"{}\" is missing!", c);
                "".to_string()
            }
            Ok(user) => user,
        };

        let c = "user.email";
        let email = match cfg.get_string(c) {
            Err(_) => {
                println!("Warning: your git config \"{}\" is missing!", c);
                "".to_string()
            }
            Ok(email) => email,
        };

        format!("{} <{}>", user, email)
    }

    pub fn get_gitmessage(&self) -> String {
        let gitmessage_path = self.get_gitmessage_path();
        self.file_actions.read(&gitmessage_path)
    }

    pub fn get_output(&self) -> String {
        let git_user = self.get_git_user();

        let gitmessage = self.get_gitmessage();
        let gitmessage = gitmessage.trim();

        if gitmessage.is_empty() {
            git_user
        } else {
            format!("{}\n{}", git_user, gitmessage)
        }
    }

    pub fn print_output(&self) {
        println!("{}", self.get_output());
    }
}

pub mod test_utils {
    use super::*;
    use std::cell::RefCell;

    pub struct MockFileActions {
        s: RefCell<String>,
    }

    impl Default for test_utils::MockFileActions {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockFileActions {
        pub fn new() -> MockFileActions {
            MockFileActions {
                s: RefCell::new(String::new()),
            }
        }
    }

    impl FileActions for MockFileActions {
        fn write(&self, _: &Path, s: String) {
            self.s.replace(s);
        }

        fn read(&self, _: &Path) -> String {
            self.s.borrow().clone()
        }
    }

    pub fn get_git_mob() -> GitMob {
        GitMob {
            file_actions: Box::from(MockFileActions::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::get_git_mob;

    #[test]
    fn test_write_gitmessage() {
        let gm = get_git_mob();
        gm.write_gitmessage("test".to_string());

        assert_eq!("\n\ntest", gm.get_gitmessage());
        assert_eq!(format!("{}\ntest", gm.get_git_user()), gm.get_output());
    }
}
