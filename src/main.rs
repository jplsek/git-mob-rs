use clap::{AppSettings, Clap};
use dirs;
use git2::Config;
use open;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/// Quickly populates the .git/.gitmessage template file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    Mob(Mob),
    Solo(Solo),
    EditCoauthors(EditCoauthors),
}

/// Users mobbing with, for example "git mob fb ab"
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Mob {
    /// Users mobbing with, for example "git mob fb ab"
    users: Vec<String>,
}

/// Reset back to just yourself (clears the gitmessage template)
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Solo {}

/// Edit the coauthors config file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct EditCoauthors {}

#[derive(Serialize, Deserialize, Debug)]
struct Coauthors {
    coauthors: HashMap<String, Author>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

trait FileActions {
    fn write(&self, path: &Path, s: String);
    fn read(&self, path: &Path) -> String;
}

struct GitMobFileActions();

// Mostly here for unit testing
impl FileActions for GitMobFileActions {
    fn write(&self, path: &Path, s: String) {
        match fs::write(path, s.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
            Ok(_) => {}
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

struct GitMob(Box<dyn FileActions>);

impl GitMob {
    fn new() -> GitMob {
        GitMob(Box::from(GitMobFileActions()))
    }

    fn get_gitmessage_path<'a>(&self) -> &'a Path {
        Path::new(".git/.gitmessage")
    }

    fn write_gitmessage(&self, s: String) {
        let gitmessage_path = self.get_gitmessage_path();

        self.0.write(&gitmessage_path, format!("\n\n{}", s));
    }

    fn solo(&self) {
        let gitmessage_path = self.get_gitmessage_path();

        self.0.write(&gitmessage_path, "".to_string());
    }

    /// Returns the coauthors path
    ///
    /// This supports both xdg (prioritized) or if the config is in the home directory (like
    /// git-mob).
    fn get_coauthors_path(&self) -> PathBuf {
        let file_name = "git-coauthors";

        // most likely on fresh install after first use
        let mut coauthors_path = dirs::config_dir().unwrap();
        coauthors_path.push(file_name);
        if coauthors_path.exists() {
            return coauthors_path;
        }

        // else check home dir - if it doesn't exist (like a fresh install) use xdg instead
        let mut home_coauthors_path = dirs::home_dir().unwrap();
        home_coauthors_path.push(format!(".{}", file_name));
        if home_coauthors_path.exists() {
            home_coauthors_path
        } else {
            coauthors_path
        }
    }

    fn mob(&self, users: Vec<String>) -> Result<(), Box<dyn Error>> {
        // make sure to not accidentally "solo"
        if users.is_empty() {
            return Ok(())
        }

        let coauthors_path = self.get_coauthors_path();
        let coauthors_path = coauthors_path.as_path();
        let coauthors_str = self.0.read(coauthors_path);

        if coauthors_str.is_empty() {
            return Err(Box::from(format!(
                "Coauthors file {} is empty!",
                coauthors_path.display()
            )));
        }

        let coauthors: Coauthors = serde_json::from_str(coauthors_str.as_str()).unwrap();
        let coauthors = coauthors.coauthors;

        let mut name_emails: Vec<String> = vec![];

        for user in users.iter() {
            if coauthors.contains_key(user) {
                let author = &coauthors[user];
                name_emails.push(format!(
                    "Co-authored-by: {} <{}>",
                    &author.name, &author.email
                ));
            } else {
                return Err(Box::from(format!(
                    "Author with initials \"{}\" not found in {}!",
                    user,
                    coauthors_path.display()
                )));
            }
        }

        self.write_gitmessage(name_emails.join("\n"));

        Ok(())
    }

    fn get_git_user(&self) -> String {
        let cfg = Config::open_default().unwrap();
        let user = cfg.get_string("user.name").unwrap();
        let email = cfg.get_string("user.email").unwrap();
        format!("{} <{}>", user, email)
    }

    fn get_gitmessage(&self) -> String {
        let gitmessage_path = self.get_gitmessage_path();
        self.0.read(&gitmessage_path)
    }

    fn get_output(&self) -> String {
        let git_user = self.get_git_user();

        let gitmessage = self.get_gitmessage();
        let gitmessage = gitmessage.trim();

        if gitmessage.is_empty() {
            git_user
        } else {
            format!("{}\n{}", git_user, gitmessage)
        }
    }

    fn print_output(&self) {
        println!("{}", self.get_output());
    }

    fn edit(&self) {
        let coauthors_path = self.get_coauthors_path();

        // write part of the config for convenience
        if !coauthors_path.exists() {
            let s = "{\n  \"coauthors\": {\n    \"\": {\n      \"name\": \"\",\n      \"email\": \"\"\n    }\n  }\n}\n";
            self.0.write(&coauthors_path, s.to_string());
        }

        println!(
            "Opening {} in the default text editor...",
            coauthors_path.display()
        );

        match open::that(coauthors_path) {
            Ok(exit_status) => {
                if !exit_status.success() {
                    if let Some(code) = exit_status.code() {
                        println!("Command returned non-zero exit status {}!", code);
                    } else {
                        println!("Command returned with unknown exit status!");
                    }
                }
            }
            Err(why) => println!("Failure to execute command: {}", why),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let gm = GitMob::new();

    match opts.subcmd {
        Some(cmd) => match cmd {
            SubCommand::Solo(..) => {
                gm.solo();
            }
            SubCommand::Mob(t) => match gm.mob(t.users) {
                Ok(_) => {}
                Err(why) => {
                    println!("{}", why);
                    return;
                }
            },
            SubCommand::EditCoauthors(..) => {
                gm.edit();
                return;
            }
        },
        None => {}
    }

    gm.print_output();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockGitMobFileActions {
        s: RefCell<String>,
    }

    impl MockGitMobFileActions {
        fn new() -> MockGitMobFileActions {
            MockGitMobFileActions {
                s: RefCell::new(String::new()),
            }
        }
    }

    impl FileActions for MockGitMobFileActions {
        fn write(&self, _: &Path, s: String) {
            self.s.replace(s);
        }

        fn read(&self, _: &Path) -> String {
            self.s.borrow().clone()
        }
    }

    #[test]
    fn test_write_gitmessage() {
        let gm = GitMob(Box::from(MockGitMobFileActions::new()));
        gm.write_gitmessage("test".to_string());

        assert_eq!("\n\ntest", gm.get_gitmessage());
        assert_eq!(format!("{}\ntest", gm.get_git_user()), gm.get_output());
    }

    #[test]
    fn test_solo() {
        let gm = GitMob(Box::from(MockGitMobFileActions::new()));
        gm.solo();

        assert_eq!("", gm.get_gitmessage());
        assert_eq!(gm.get_git_user(), gm.get_output());
    }

    #[test]
    fn test_mob() {
        let gm = GitMob(Box::from(MockGitMobFileActions::new()));

        gm.0.write(
            Path::new(""),
            r#"
        {
          "coauthors": {
            "ab": {
                "name": "A B",
                "email": "ab@example.com"
            }
          }
        }
        "#
            .to_string(),
        );

        gm.mob(vec!["ab".to_string()]).unwrap();

        let author = "Co-authored-by: A B <ab@example.com>";

        assert_eq!(format!("\n\n{}", author), gm.get_gitmessage());
        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), author),
            gm.get_output()
        );

        // make sure empty vec doesn't reset gitmessage file
        gm.mob(vec![]).unwrap();

        assert_eq!(format!("\n\n{}", author), gm.get_gitmessage());
        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), author),
            gm.get_output()
        );

        // make sure solo resets properly
        gm.solo();

        assert_eq!("", gm.get_gitmessage());
        assert_eq!(gm.get_git_user(), gm.get_output());
    }

    #[test]
    fn test_mob_2() {
        let gm = GitMob(Box::from(MockGitMobFileActions::new()));

        let json = r#"
        {
          "coauthors": {
            "ab": {
                "name": "A B",
                "email": "ab@example.com"
            },
            "cd": {
                "name": "C D",
                "email": "cd@example.com"
            }
          }
        }
        "#;

        gm.0.write(Path::new(""), json.to_string());

        gm.mob(vec!["ab".to_string()]).unwrap();

        let author = "Co-authored-by: A B <ab@example.com>";

        assert_eq!(format!("\n\n{}", author), gm.get_gitmessage());
        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), author),
            gm.get_output()
        );

        gm.0.write(Path::new(""), json.to_string());

        gm.mob(vec!["ab".to_string(), "cd".to_string()]).unwrap();

        let authors = "Co-authored-by: A B <ab@example.com>\nCo-authored-by: C D <cd@example.com>";

        assert_eq!(format!("\n\n{}", authors), gm.get_gitmessage());
        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), authors),
            gm.get_output()
        );
    }

    #[test]
    fn test_mob_empty_authors() {
        let gm = GitMob(Box::from(MockGitMobFileActions::new()));
        let r = gm.mob(vec!["ab".to_string()]);

        let expected = format!(
            "Coauthors file {} is empty!",
            gm.get_coauthors_path().display()
        );
        assert_eq!(expected, r.unwrap_err().to_string());
    }

    #[test]
    fn test_mob_no_authors() {
        let gm = GitMob(Box::from(MockGitMobFileActions::new()));

        gm.0.write(
            Path::new(""),
            r#"
        {
          "coauthors": {
          }
        }
        "#
            .to_string(),
        );

        let r = gm.mob(vec!["ab".to_string()]);

        let expected = format!(
            "Author with initials \"ab\" not found in {}!",
            gm.get_coauthors_path().display()
        );
        assert_eq!(expected, r.unwrap_err().to_string());
    }
}
