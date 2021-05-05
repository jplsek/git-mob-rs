use clap::{AppSettings, Clap};
use git_mob_rs::GitMob;

/// Quickly populates the .git/.gitmessage template file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Users mobbing with, for example "git mob fb ab"
    users: Vec<String>,
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
