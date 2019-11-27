extern crate clap;
extern crate config;
extern crate reqwest;
extern crate termion;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use clap::App;
use clap::SubCommand;

mod dashboard;
mod foreman_config;
mod wizard;

mod command;

mod error;
use error::Error;

extern crate short_crypt;

const DASHBOARD_URL: &str = "http://localhost:8080";

fn main() -> Result<(), Error> {
    let _matches = App::new("Le cli étincelant")
        .version("1.0")
        .author("Paul Delafosse <paul.delafosse.etu@univ-lille.fr>")
        .about("Cli pour l'atelier spark - Université de Lille - IFI - 2019")
        .subcommand(SubCommand::with_name("init").about("(re)initialiser votre projet"))
        .subcommand(SubCommand::with_name("next").about("passer à l'étape suivante"))
        .get_matches();

    if let Some(_matches) = _matches.subcommand_matches("init") {
        let team_name = wizard::walkthrough();
        let team = dashboard::create_team(&team_name)
            .expect("Une erreur est survenue, contactes Paul ou Lucas");

        foreman_config::write(&team)?;
        println!("{}", foreman_config::get().unwrap().step);
    }

    if let Some(_matches) = _matches.subcommand_matches("next") {
        let config = foreman_config::get()?;
        let test_passed = command::maven::test().unwrap();

        command::git::add().unwrap();
        command::git::commit().unwrap();
        command::git::merge(config.step).unwrap();

        if test_passed {
            dashboard::step_forward(&config)
                .expect("Contact Paul ou Lucas quelque chose c'est mal passé")
        } else {
            dashboard::step_failed(&config)
                .expect("Contact Paul ou Lucas quelque chose c'est mal passé")
        }
    }

    Ok(())
}
