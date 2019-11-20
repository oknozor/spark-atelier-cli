extern crate config;

mod error;
use error::Error;

use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::process::Output;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if let Some(group_name) = parse_args(&args) {
        println!("found group name in args: {}", group_name)
    }

    let group_name = group_name().unwrap();

    let mvn = maven_test().unwrap();
    println!("{}", mvn);

    git_add().unwrap();
    git_commit().unwrap();
    git_merge(1).unwrap();

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

fn parse_args(args: &[String]) -> Option<&str> {
    if args.len() > 2 {
        if &args[1] != "--init" {
            panic!("Expected param `--init` found {}", &args[1])
        } else {
            return Some(&args[2]);
        }
    }

    None
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
