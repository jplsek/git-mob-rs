use clap::Parser;
use git_mob_rs::{Author, GitMob};

/// Adds a coauthor to the coauthors config file.
/// For example: git add-coauthor jd "John Doe" jdoe@example.com
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// The initials of the coauthor, such as jd
    initials: String,
    /// The name of the coauthor, such as "John Doe"
    name: String,
    /// The email of the coauthor, such as jdoe@example.com
    email: String,
}

trait Add {
    fn add(&self, initials: String, name: String, email: String) -> String;
}

impl Add for GitMob {
    fn add(&self, initials: String, name: String, email: String) -> String {
        let coauthors_path = self.get_coauthors_path();
        let coauthors_path = coauthors_path.display();

        let mut coauthors = self.get_all_coauthors();
        coauthors.insert(
            initials,
            Author {
                name: name.clone(),
                email,
            },
        );

        self.write_coauthors(coauthors);

        format!("{name} has been added to the {coauthors_path} file")
    }
}

fn main() {
    let opts = Cli::parse();

    let gm = GitMob::default();

    println!("{}", gm.add(opts.initials, opts.name, opts.email));
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;
    use linked_hash_map::LinkedHashMap;

    #[test]
    fn test_add() {
        let gm = get_git_mob();
        let coauthors_path = gm.get_coauthors_path();

        let mut expected_coauthors = LinkedHashMap::new();
        expected_coauthors.insert(
            String::from("ab"),
            Author {
                name: String::from("A B"),
                email: "ab@example.com".to_string(),
            },
        );

        assert_eq!(
            format!(
                "A B has been added to the {} file",
                coauthors_path.display()
            ),
            gm.add(
                String::from("ab"),
                String::from("A B"),
                String::from("ab@example.com"),
            )
        );
        assert_eq!(expected_coauthors, gm.get_all_coauthors());
    }
}
