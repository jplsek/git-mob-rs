use clap::{AppSettings, Clap};
use git_mob_rs::GitMob;
use open;

/// Edit the coauthors config file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {}

trait Edit {
    fn edit(&self);
}

impl Edit for GitMob {
    fn edit(&self) {
        let coauthors_path = self.get_coauthors_path();

        // write part of the config for convenience
        if !coauthors_path.exists() {
            let s = "{\n  \"coauthors\": {\n    \"\": {\n      \"name\": \"\",\n      \"email\": \"\"\n    }\n  }\n}\n";
            self.file_actions.write(&coauthors_path, s.to_string());
        }

        println!(
            "Opening {} in the default text editor...",
            coauthors_path.display()
        );

        match open::that(coauthors_path) {
            Ok(exit_status) => {
                if !exit_status.success() {
                    if let Some(code) = exit_status.code() {
                        println!("Command returned non-zero exit status {}!", code);
                    } else {
                        println!("Command returned with unknown exit status!");
                    }
                }
            }
            Err(why) => println!("Failure to execute command: {}", why),
        }
    }
}

fn main() {
    Opts::parse();

    let gm = GitMob::new();

    gm.edit();
}
