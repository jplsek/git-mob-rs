use clap::Parser;
use git_mob_rs::{exit_with_error::ExitWithError, file_actions::FileActions, GitMob};

/// Print the .gitmessage template
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {}

trait Print {
    fn print(&self) -> String;
}

impl<T: FileActions, U: ExitWithError> Print for GitMob<T, U> {
    fn print(&self) -> String {
        format!("{}\n", self.get_gitmessage())
    }
}

fn main() {
    Cli::parse();

    let gm = GitMob::default();

    print!("{}", gm.print());
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;

    #[test]
    fn test_print() {
        let gm = get_git_mob();

        gm.write_gitmessage("test".to_string());

        assert_eq!("\n\ntest\n", gm.print());
    }
}
