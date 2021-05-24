use dirs::{config_dir, home_dir};
use git2::{Config, ErrorCode, Repository};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// This mostly exists to make a mock for unit testing
pub trait FileActions {
    fn write(&self, path: &Path, s: String) -> Result<(), Box<dyn Error>>;
    fn read(&self, path: &Path) -> Result<String, Box<dyn Error>>;
}

pub struct GmFileActions();

impl FileActions for GmFileActions {
    fn write(&self, path: &Path, s: String) -> Result<(), Box<dyn Error>> {
        if let Err(why) = fs::write(path, s.as_bytes()) {
            return Err(Box::from(format!(
                "couldn't write to {}: {}",
                path.display(),
                why
            )));
        }
        Ok(())
    }

    fn read(&self, path: &Path) -> Result<String, Box<dyn Error>> {
        let mut file = match File::open(&path) {
            Err(why) => {
                return Err(Box::from(format!(
                    "couldn't open {}: {}",
                    path.display(),
                    why
                )))
            }
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => Err(Box::from(format!(
                "couldn't read {}: {}",
                path.display(),
                why
            ))),
            Ok(_) => Ok(s),
        }
    }
}

pub struct GitMob {
    pub file_actions: Box<dyn FileActions>,
}

impl Default for GitMob {
    fn default() -> Self {
        GitMob {
            file_actions: Box::from(GmFileActions()),
        }
    }
}

impl GitMob {
    pub fn get_repo(&self) -> Repository {
        Repository::open_from_env().unwrap_or_else(|_| {
            panic!("Not in a git repository");
        })
    }

    pub fn get_gitmessage_path(&self) -> PathBuf {
        self.get_repo().path().join(".gitmessage")
    }

    pub fn write_gitmessage(&self, s: String) {
        let gitmessage_path = self.get_gitmessage_path();

        // for git solo
        let s = if s.is_empty() {
            s
        } else {
            format!("\n\n{}", s)
        };

        self.file_actions
            .write(&gitmessage_path, s)
            .unwrap_or_else(|error| {
                panic!(
                    "{}\nMake sure you are in a git repository when running this command",
                    error
                )
            });

        self.set_git_template();
    }

    fn set_git_template(&self) {
        let mut cfg = self.get_repo().config().unwrap();
        let key = "commit.template";

        // don't write to if we don't have to
        match cfg.get_entry(key) {
            Ok(_) => return,
            Err(err) => {
                if err.code() != ErrorCode::NotFound {
                    panic!("{}", err);
                }
            }
        };

        cfg.set_str(key, ".git/.gitmessage").unwrap();
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

    fn get_git_config(&self, cfg: &Config, key: &str) -> String {
        // these errors should only really happen in ci
        cfg.get_string(key).unwrap_or_else(|_| {
            println!("Warning: your git config \"{}\" is missing!", key);
            "".to_string()
        })
    }

    pub fn get_git_user(&self) -> String {
        let cfg = self.get_repo().config().unwrap();

        let user = self.get_git_config(&cfg, "user.name");
        let email = self.get_git_config(&cfg, "user.email");

        format!("{} <{}>", user, email)
    }

    pub fn get_gitmessage(&self) -> String {
        let gitmessage_path = self.get_gitmessage_path();
        self.file_actions
            .read(&gitmessage_path)
            .unwrap_or_else(|error| {
                panic!(
                    "{}\nMake sure you are in a git repository when running this command.",
                    error
                )
            })
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
            MockFileActions {
                s: RefCell::new(String::new()),
            }
        }
    }

    impl FileActions for MockFileActions {
        fn write(&self, _: &Path, s: String) -> Result<(), Box<dyn Error>> {
            self.s.replace(s);
            Ok(())
        }

        fn read(&self, _: &Path) -> Result<String, Box<dyn Error>> {
            Ok(self.s.borrow().clone())
        }
    }

    pub fn get_git_mob() -> GitMob {
        GitMob {
            file_actions: Box::from(MockFileActions::default()),
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
