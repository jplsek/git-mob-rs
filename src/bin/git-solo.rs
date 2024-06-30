use clap::Parser;
use git_mob_rs::{exit_with_error::ExitWithError, file_actions::FileActions, GitMob};

/// Reset back to just yourself (clears the gitmessage template)
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {}

trait Solo {
    fn solo(&self) -> String;
}

impl<T: FileActions, U: ExitWithError> Solo for GitMob<T, U> {
    fn solo(&self) -> String {
        self.write_gitmessage(vec![]);
        self.get_formatted_gitmessage()
    }
}

fn main() {
    Cli::parse();

    let gm = GitMob::default();

    println!("{}", gm.solo());
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;

    #[test]
    fn test_solo() {
        let gm = get_git_mob();
        let actual = gm.solo();

        assert_eq!("", gm.get_gitmessage());
        assert_eq!(gm.get_git_user(), actual);
    }
}
