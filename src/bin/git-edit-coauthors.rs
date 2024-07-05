use clap::Parser;
use git_mob_rs::{exit_with_error::ExitWithError, file_actions::FileActions, GitMob};
use serde_json::{json, to_string_pretty};

/// Edit the coauthors config file
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {}

trait Edit {
    fn edit(&self);
}

impl<T: FileActions, U: ExitWithError> Edit for GitMob<T, U> {
    fn edit(&self) {
        let coauthors_path = self.get_coauthors_path();

        // write part of the config for convenience
        if !coauthors_path.exists() {
            let s = json!({
                "coauthors": {
                    "": {
                        "name": "",
                        "email": ""
                    }
                }
            });
            self.file_actions
                .write(&coauthors_path, &to_string_pretty(&s).unwrap())
                .unwrap();
        }

        println!(
            "Opening {} in the default text editor...",
            coauthors_path.display()
        );

        open::that(coauthors_path).unwrap();
    }
}

fn main() {
    Cli::parse();

    let gm = GitMob::default();

    gm.edit();
}
