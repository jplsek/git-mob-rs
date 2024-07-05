use clap::Parser;
use git_mob_rs::{exit_with_error::ExitWithError, file_actions::FileActions, GitMob};

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
    fn mob(&self, users: &[String]) -> String;
    fn list(&self) -> String;
}

impl<T: FileActions, U: ExitWithError> Mob for GitMob<T, U> {
    fn list(&self) -> String {
        let coauthors = self.get_all_coauthors();
        let initials: Vec<String> = coauthors
            .into_iter()
            .map(|(initials, author)| {
                let name = author.name;
                let email = author.email;
                format!("{initials} {name} <{email}>")
            })
            .collect();
        format!("{}\n", initials.join("\n"))
    }

    fn mob(&self, initials: &[String]) -> String {
        // make sure to not accidentally "solo"
        if initials.is_empty() {
            return self.get_formatted_gitmessage();
        }

        self.write_gitmessage(initials);
        self.get_formatted_gitmessage()
    }
}

fn main() {
    let opts: Cli = Cli::parse();

    let gm = GitMob::default();

    if opts.list {
        print!("{}", gm.list());
    } else {
        println!("{}", gm.mob(&opts.initials));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;
    use serde_json::json;

    #[test]
    fn test_mob() {
        let gm = get_git_mob();

        let authors = "Co-authored-by: A B <ab@example.com>\nCo-authored-by: C D <cd@example.com>";

        assert_eq!(
            format!("{}\n{}", gm.get_git_user(), authors),
            gm.mob(&[String::from("ab"), String::from("cd")])
        );
        assert_eq!(format!("\n\n{}", authors), gm.get_gitmessage());

        // make sure empty vec doesn't reset gitmessage file
        assert_eq!(format!("{}\n{}", gm.get_git_user(), authors), gm.mob(&[]));
        assert_eq!(format!("\n\n{}", authors), gm.get_gitmessage());
    }

    #[test]
    fn test_list() {
        let gm = get_git_mob();

        let author1 = "ab A B <ab@example.com>";
        let author2 = "cd C D <cd@example.com>";

        assert_eq!(format!("{}\n{}\n", author1, author2), gm.list());
    }

    #[test]
    #[should_panic]
    fn test_mob_empty_authors() {
        let gm = get_git_mob();
        gm.mob(&[String::from("ef")]);
    }

    #[test]
    #[should_panic]
    fn test_mob_no_authors() {
        let gm = get_git_mob();

        gm.file_actions
            .write(
                &gm.get_coauthors_path(),
                &json!({
                    "coauthors": {
                    }
                })
                .to_string(),
            )
            .unwrap();

        gm.mob(&[String::from("ab")]);
    }
}
