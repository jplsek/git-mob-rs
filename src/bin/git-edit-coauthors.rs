use clap::{AppSettings, Clap};
use git_mob_rs::GitMob;

/// Edit the coauthors config file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {}

fn main() {
    Opts::parse();

    let gm = GitMob::new();

    gm.edit();
}
