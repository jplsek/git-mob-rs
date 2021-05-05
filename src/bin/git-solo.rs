use clap::{AppSettings, Clap};
use git_mob_rs::GitMob;

/// Reset back to just yourself (clears the gitmessage template)
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {}

fn main() {
    Opts::parse();

    let gm = GitMob::new();

    gm.solo();

    gm.print_output();
}
