use clap::Parser;
use git_mob_rs::GitMob;

/// Delete a coauthor from the coauthors config file.
/// For example: git delete-coauthor jd
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// The initials of the coauthor, such as jd
    initials: Vec<String>,
}

trait Delete {
    fn delete(&self, initials: Vec<String>) -> String;
}

impl Delete for GitMob {
    fn delete(&self, initials: Vec<String>) -> String {
        let coauthors_path = self.get_coauthors_path();
        let coauthors_path = coauthors_path.display();

        let mut coauthors = self.get_all_coauthors();
        let mut s = String::new();
        for initial in initials.iter() {
            coauthors.remove(initial);
            s.push_str(format!("{initial}: has been removed from {coauthors_path}\n").as_str());
        }

        self.write_coauthors(coauthors);

        s
    }
}

fn main() {
    let opts = Cli::parse();

    let gm = GitMob::default();

    print!("{}", gm.delete(opts.initials));
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;
    use git_mob_rs::{test_utils::get_git_mob, Author};
    use linked_hash_map::LinkedHashMap;
    use serde_json::json;

    #[test]
    fn test_delete() {
        let gm = get_git_mob();
        let coauthors_path = gm.get_coauthors_path();

        let coauthors = json!({
            "coauthors": {
                "ab": {
                    "name": "A B",
                    "email": "ab@example.com"
                },
                "cd": {
                    "name": "C D",
                    "email": "cd@example.com"
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
                name: String::from("A B"),
                email: "ab@example.com".to_string(),
            },
        );

        assert_eq!(
            format!(
                "cd: has been removed from {}\nef: has been removed from {}\n",
                coauthors_path.display(),
                coauthors_path.display()
            ),
            gm.delete(vec![String::from("cd"), String::from("ef")])
        );
        assert_eq!(expected_coauthors, gm.get_all_coauthors());
    }
}
