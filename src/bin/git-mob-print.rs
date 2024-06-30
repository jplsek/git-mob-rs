use clap::Parser;
use git_mob_rs::{exit_with_error::ExitWithError, file_actions::FileActions, GitMob};

/// Print the .gitmessage template
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Prints a comma separated list of selected co-author initials
    #[arg(short, long)]
    initials: bool,
}

trait Print {
    fn print(&self) -> String;
    fn print_initials(&self) -> String;
}

impl<T: FileActions, U: ExitWithError> Print for GitMob<T, U> {
    fn print(&self) -> String {
        format!("{}\n", self.get_gitmessage())
    }

    fn print_initials(&self) -> String {
        self.get_gitinitials()
    }
}

fn main() {
    let opts: Cli = Cli::parse();

    let gm = GitMob::default();

    if opts.initials {
        print!("{}", gm.print_initials());
    } else {
        print!("{}", gm.print());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;

    #[test]
    fn test_print() {
        let gm = get_git_mob();

        let authors =
            "\n\nCo-authored-by: A B <ab@example.com>\nCo-authored-by: C D <cd@example.com>\n";

        gm.write_gitmessage(vec![String::from("ab"), String::from("cd")]);

        assert_eq!(authors, gm.print());
    }

    #[test]
    fn test_print_initials() {
        let gm = get_git_mob();

        gm.write_gitmessage(vec![]);

        assert_eq!("\n", gm.print_initials());

        gm.write_gitmessage(vec![String::from("ab"), String::from("cd")]);

        assert_eq!("ab,cd\n", gm.print_initials());
    }
}
