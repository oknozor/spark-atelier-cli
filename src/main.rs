extern crate clap;
extern crate config;
extern crate reqwest;
extern crate termion;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use clap::App;
use clap::Arg;
use clap::SubCommand;

mod dashboard;
mod foreman_config;
mod wizard;

mod command;

mod error;
use error::Error;

extern crate short_crypt;

const DASHBOARD_URL: &str = "http://spark-leaderboard-backend.hoohoot.org";

fn main() -> Result<(), Error> {
    let _matches = App::new("Foreman le contremaître")
        .version("1.0")
        .author("Paul Delafosse <paul.delafosse.etu@univ-lille.fr>")
        .about("Cli pour l'atelier spark - Université de Lille - IFI - 2019")
        .subcommand(
            SubCommand::with_name("init")
                .about("initialiser votre atelier")
                .arg(
                    Arg::with_name("hard")
                        .short("h")
                        .long("hard")
                        .help("réinitialise l'atelier (attention cela supprimera votre progression actuelle)")
                        .value_name("TEAM_NAME")
                        .required(false)
                        .takes_value(true)
                ),
        )
        .subcommand(SubCommand::with_name("next").about("Passer à l'étape suivante"))
        .get_matches();

    if let Some(matches) = _matches.subcommand_matches("init") {
        let team_name;

        if matches.is_present("hard") {
            team_name = matches
                .value_of("hard")
                .map(|name| name.to_owned())
                .unwrap();
        } else {
            team_name = wizard::walkthrough();
        }

        let team = dashboard::create_team(&team_name)
            .expect("Une erreur est survenue, contacter Paul, Florian ou Lucas");

        foreman_config::write(&team)?;
    }

    if let Some(_) = _matches.subcommand_matches("next") {
        let config = foreman_config::get()?;
        let test_passed = command::maven::test(config.step).unwrap();

        if test_passed {
            command::git::add().unwrap();
            if let Ok(_) = command::git::commit() {
                command::git::merge(config.step).unwrap();

                let team = dashboard::step_forward(&config)
                    .expect("Contact Paul, Florian ou Lucas quelque chose c'est mal passé");

                foreman_config::write(&team)?;
                wizard::congrat();
            }
        } else {
            dashboard::step_failed(&config)
                .expect("Contact Paul, Florian ou Lucas quelque chose c'est mal passé");
            wizard::shame();
        }
    }

    Ok(())
}
