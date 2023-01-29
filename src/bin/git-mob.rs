use clap::Parser;
use git_mob_rs::GitMob;

/// Quickly populates the .git/gitmessage template file
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Who to set as the coauthor, for example "git mob fb ab"
    initials: Vec<String>,
    /// Show a list of all co-authors
    #[arg(short, long)]
    list: bool,
}

trait Mob {
    fn mob(&self, users: Vec<String>) -> String;
    fn list(&self) -> String;
}

impl Mob for GitMob {
    fn list(&self) -> String {
        let coauthors = self.get_all_coauthors();
        let mut s = String::new();
        for (initials, author) in coauthors {
            let name = author.name;
            let email = author.email;
            s.push_str(format!("{initials} {name} <{email}>\n").as_str());
        }
        s
    }

    fn mob(&self, initials: Vec<String>) -> String {
        // make sure to not accidentally "solo"
        if initials.is_empty() {
            return self.get_formatted_gitmessage();
        }

        let coauthors = self.get_all_coauthors();

        let mut name_emails: Vec<String> = vec![];

        for initial in initials.iter() {
            if coauthors.contains_key(initial) {
                let author = &coauthors[initial];
                let name = &author.name;
                let email = &author.email;
                name_emails.push(format!("Co-authored-by: {name} <{email}>"));
            } else {
                let coauthors_path = self.get_coauthors_path();
                let coauthors_path = coauthors_path.as_path();
                panic!(
                    "Author with initials \"{}\" not found in {}!",
                    initial,
                    coauthors_path.display()
                );
            }
        }

        self.write_gitmessage(name_emails.join("\n"));
        self.get_formatted_gitmessage()
    }
}

fn main() {
    let opts: Cli = Cli::parse();

    let gm = GitMob::default();

    if opts.list {
        print!("{}", gm.list());
    } else {
        println!("{}", gm.mob(opts.initials));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;
    use serde_json::json;
    use std::path::Path;

    #[test]
    fn test_mob() {
        let gm = get_git_mob();

        let json = json!({
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
        });

        gm.file_actions
            .write(Path::new(""), json.to_string())
            .unwrap();

        let authors = "Co-authored-by: A B <ab@example.com>\nCo-authored-by: C D <cd@example.com>";

        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), authors),
            gm.mob(vec![String::from("ab"), String::from("cd")])
        );
        assert_eq!(format!("\n\n{}", authors), gm.get_gitmessage());

        // make sure empty vec doesn't reset gitmessage file
        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), authors),
            gm.mob(vec![])
        );
        assert_eq!(format!("\n\n{}", authors), gm.get_gitmessage());
    }

    #[test]
    fn test_list() {
        let gm = get_git_mob();

        let coauthors = json!({
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
        });

        gm.file_actions
            .write(Path::new(""), coauthors.to_string())
            .unwrap();

        let author1 = "ab A B <ab@example.com>";
        let author2 = "cd C D <cd@example.com>";

        assert_eq!(format!("{}\n{}\n", author1, author2), gm.list());
    }

    #[test]
    #[should_panic]
    fn test_mob_empty_authors() {
        let gm = get_git_mob();
        gm.mob(vec![String::from("ab")]);
    }

    #[test]
    #[should_panic]
    fn test_mob_no_authors() {
        let gm = get_git_mob();

        gm.file_actions
            .write(
                Path::new(""),
                json!({
                    "coauthors": {
                    }
                })
                .to_string(),
            )
            .unwrap();

        gm.mob(vec![String::from("ab")]);
    }
}
