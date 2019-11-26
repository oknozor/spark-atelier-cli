extern crate config;
extern crate clap;

use clap::SubCommand;
use clap::App;

mod error;
use error::Error;

use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::process::Output;

fn main() -> Result<(), Error> {
    let matches = App::new("Le cli étincelant")
        .version("1.0")
        .author("Paul Delafosse <paul.delafosse.etu@univ-lille.fr>")
        .about("C'est cool")
        .subcommand(SubCommand::with_name("init").about("(re)initialiser votre projet"))
        .subcommand(SubCommand::with_name("next").about("passer à l'étape suivante"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {}

    if let Some(matches) = matches.subcommand_matches("next") {
        let group_name = group_name().unwrap();

        let mvn = maven_test().unwrap();
        println!("{}", mvn);

        git_add().unwrap();
        git_commit().unwrap();
        git_merge(1).unwrap();
    }

    Ok(())
}

fn group_name() -> Result<String, Error> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config"))?;

    settings
        .try_into::<HashMap<String, String>>()
        .map_err(|e| e.into())
        .map(|conf| conf.get("group_name").cloned())
        .map(|opt| opt.expect("expected a string value for `group_name"))
}

fn maven_test() -> Result<String, Error> {
    let mvn_test = Command::new("mvn").arg("test").output()?;
    let stdout = mvn_test.stdout;
    String::from_utf8(stdout).map_err(|e| e.into())
}

fn git_add() -> Result<Output, Error> {
    Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .map_err(|e| e.into())
}

fn git_commit() -> Result<Output, Error> {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("\"solve step\"")
        .output()
        .map_err(|e| e.into())
}

fn git_merge(step: i32) -> Result<Output, Error> {
    Command::new("git")
        .arg("merge")
        .arg("--no-ff")
        .arg("step2")
        .output()
        .map_err(|e| e.into())
}
