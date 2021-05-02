use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Display, Path};
use clap::{AppSettings, Clap};
use dirs;
use git2::Config;
use serde::{Deserialize, Serialize};

/// Quickly populates the .git/.gitmessage template file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    Mob(Mob),
    Solo(Solo),
}

/// Users mobbing with, for example "git mob fb ab"
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Mob {
    /// Users mobbing with, for example "git mob fb ab"
    users: Vec<String>,
}

/// Reset back to just yourself
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Solo {
}

#[derive(Serialize, Deserialize, Debug)]
struct Coauthors {
    coauthors: HashMap<String, Author>
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

fn get_gitmessage<'a>() -> (&'a Path, Display<'a>) {
    let path = Path::new(".git/.gitmessage");
    (path, path.display())
}

fn write_gitmessage(s: String) {
    let (path, display) = get_gitmessage();

    match fs::write(path, s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {}
    };
}

fn solo() {
    write_gitmessage(String::new());
}

fn mob(users: Vec<String>) {
    let mut home = dirs::home_dir().unwrap();
    home.push(".git-coauthors");

    let coauthors_path = home.as_path();
    let display = coauthors_path.display();

    let coauthors_file = match File::open(&coauthors_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let coauthors: Coauthors = serde_json::from_reader(BufReader::new(coauthors_file)).unwrap();
    let coauthors = coauthors.coauthors;

    let mut name_emails: Vec<String> = vec![];

    for user in users.iter() {
        let author = &coauthors[user];
        name_emails.push(format!("Co-authored-by: {} <{}>", &author.name, &author.email));
    }

    write_gitmessage(format!("\n\n{}", name_emails.join("\n")));
}

fn print_template() {
    let (gitmessage_path, display) = get_gitmessage();

    let mut gitmessage_file = match File::open(&gitmessage_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let cfg = Config::open_default().unwrap();
    let user = cfg.get_string("user.name").unwrap();
    let email = cfg.get_string("user.email").unwrap();
    println!("{} <{}>", user, email);

    let mut s = String::new();
    let s = match gitmessage_file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s.trim(),
    };

    if !s.is_empty() {
        println!("{}", s);
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        Some(cmd) => {
            match cmd {
                SubCommand::Solo(..) => {
                    solo();
                }
                SubCommand::Mob(t) => {
                    mob(t.users);
                }
            }
        }
        None => { }
    }

    print_template();
}
