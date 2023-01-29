use clap::Parser;
use git_mob_rs::GitMob;

/// Edits a coauthor in the coauthors config file.
/// For example: git edit-coauthor jd --name "John Doe" --email jdoe@example.com
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// The initials of the coauthor, such as jd
    initials: String,
    /// The name of the coauthor, such as "John Doe"
    #[arg(short, long)]
    name: Option<String>,
    /// The email of the coauthor, such as jdoe@example.com
    #[arg(short, long)]
    email: Option<String>,
}

trait Edit {
    fn edit(&self, initials: String, name: Option<String>, email: Option<String>) -> String;
}

impl Edit for GitMob {
    fn edit(&self, initials: String, name: Option<String>, email: Option<String>) -> String {
        let coauthors_path = self.get_coauthors_path();

        let mut coauthors = self.get_all_coauthors();
        let coauthor = coauthors.get_mut(&initials);
        match coauthor {
            Some(coauthor) => {
                if let Some(name) = name {
                    coauthor.name = name
                }
                if let Some(email) = email {
                    coauthor.email = email
                }
            }
            None => {
                panic!(
                    "Author with initials \"{}\" not found in {}!",
                    initials,
                    coauthors_path.display()
                );
            }
        };

        self.write_coauthors(coauthors);

        format!("{initials} has been updated")
    }
}

fn main() {
    let opts = Cli::parse();

    let gm = GitMob::default();

    println!("{}", gm.edit(opts.initials, opts.name, opts.email));
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;
    use git_mob_rs::{test_utils::get_git_mob, Author};
    use linked_hash_map::LinkedHashMap;
    use serde_json::json;

    #[test]
    fn test_edit_name_and_email() {
        let gm = get_git_mob();

        let coauthors = json!({
            "coauthors": {
                "ab": {
                    "name": "A B",
                    "email": "ab@example.com"
                },
                "ef": {
                    "name": "E F",
                    "email": "ef@example.com"
                }
            }
        });

        gm.file_actions
            .write(Path::new(""), coauthors.to_string())
            .unwrap();

        let mut expected_coauthors = LinkedHashMap::new();
        expected_coauthors.insert(
            String::from("ab"),
            Author {
                name: String::from("C D"),
                email: "cd@example.com".to_string(),
            },
        );
        expected_coauthors.insert(
            String::from("ef"),
            Author {
                name: String::from("E F"),
                email: "ef@example.com".to_string(),
            },
        );

        assert_eq!(
            "ab has been updated",
            gm.edit(
                String::from("ab"),
                Some(String::from("C D")),
                Some(String::from("cd@example.com")),
            )
        );
        assert_eq!(expected_coauthors, gm.get_all_coauthors());
    }

    #[test]
    fn test_edit_name() {
        let gm = get_git_mob();

        let coauthors = json!({
            "coauthors": {
                "ab": {
                    "name": "A B",
                    "email": "ab@example.com"
                }
            }
        });

        gm.file_actions
            .write(Path::new(""), coauthors.to_string())
            .unwrap();

        let mut expected_coauthors = LinkedHashMap::new();
        expected_coauthors.insert(
            String::from("ab"),
            Author {
                name: String::from("C D"),
                email: "ab@example.com".to_string(),
            },
        );

        assert_eq!(
            "ab has been updated",
            gm.edit(String::from("ab"), Some(String::from("C D")), None)
        );
        assert_eq!(expected_coauthors, gm.get_all_coauthors());
    }

    #[test]
    fn test_edit_email() {
        let gm = get_git_mob();

        let coauthors = json!({
            "coauthors": {
                "ab": {
                    "name": "A B",
                    "email": "ab@example.com"
                }
            }
        });

        gm.file_actions
            .write(Path::new(""), coauthors.to_string())
            .unwrap();

        let mut expected_coauthors = LinkedHashMap::new();
        expected_coauthors.insert(
            String::from("ab"),
            Author {
                name: String::from("A B"),
                email: "cd@example.com".to_string(),
            },
        );

        assert_eq!(
            "ab has been updated",
            gm.edit(
                String::from("ab"),
                None,
                Some(String::from("cd@example.com"))
            )
        );
        assert_eq!(expected_coauthors, gm.get_all_coauthors());
    }

    #[test]
    #[should_panic]
    fn test_edit_author_who_does_not_exist() {
        let gm = get_git_mob();
        gm.edit(String::from("ab"), None, None);
    }
}
