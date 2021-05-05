use clap::{AppSettings, Clap};
use git_mob_rs::GitMob;

/// Reset back to just yourself (clears the gitmessage template)
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {}

trait Solo {
    fn solo(&self);
}

impl Solo for GitMob {
    fn solo(&self) {
        let gitmessage_path = self.get_gitmessage_path();

        self.file_actions.write(&gitmessage_path, "".to_string());
    }
}

fn main() {
    Opts::parse();

    let gm = GitMob::new();

    gm.solo();

    gm.print_output();
}

#[cfg(test)]
mod test {
    use super::*;
    use git_mob_rs::test_utils::get_git_mob;

    #[test]
    fn test_solo() {
        let gm = get_git_mob();
        gm.solo();

        assert_eq!("", gm.get_gitmessage());
        assert_eq!(gm.get_git_user(), gm.get_output());
    }
}
