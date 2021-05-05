use clap::{AppSettings, Clap};
use git_mob_rs::GitMob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// Quickly populates the .git/.gitmessage template file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Users mobbing with, for example "git mob fb ab"
    users: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coauthors {
    coauthors: HashMap<String, Author>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

trait Mob {
    fn mob(&self, users: Vec<String>) -> Result<(), Box<dyn Error>>;
}

impl Mob for GitMob {
    fn mob(&self, users: Vec<String>) -> Result<(), Box<dyn Error>> {
        // make sure to not accidentally "solo"
        if users.is_empty() {
            return Ok(());
        }

        let coauthors_path = self.get_coauthors_path();
        let coauthors_path = coauthors_path.as_path();
        let coauthors_str = self.file_actions.read(coauthors_path);

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
}

fn main() {
    let opts: Opts = Opts::parse();

    let gm = GitMob::new();

    match gm.mob(opts.users) {
        Ok(_) => {}
        Err(why) => {
            println!("{}", why);
            return;
        }
    }

    gm.print_output();
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;
    use std::path::Path;

    #[test]
    fn test_mob() {
        let gm = get_git_mob();
        let gitmessage_path = gm.get_gitmessage_path();
        gm.file_actions.write(&gitmessage_path, "".to_string());

        gm.file_actions.write(
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
    }

    #[test]
    fn test_mob_2() {
        let gm = get_git_mob();

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

        gm.file_actions.write(Path::new(""), json.to_string());

        gm.mob(vec!["ab".to_string()]).unwrap();

        let author = "Co-authored-by: A B <ab@example.com>";

        assert_eq!(format!("\n\n{}", author), gm.get_gitmessage());
        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), author),
            gm.get_output()
        );

        gm.file_actions.write(Path::new(""), json.to_string());

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
        let gm = get_git_mob();
        let r = gm.mob(vec!["ab".to_string()]);

        let expected = format!(
            "Coauthors file {} is empty!",
            gm.get_coauthors_path().display()
        );
        assert_eq!(expected, r.unwrap_err().to_string());
    }

    #[test]
    fn test_mob_no_authors() {
        let gm = get_git_mob();

        gm.file_actions.write(
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
