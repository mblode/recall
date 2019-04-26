#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use exitfailure::ExitFailure;
use failure::ResultExt;

mod add;
mod browse;
mod decks;
mod init;
mod new;

fn main() -> Result<(), ExitFailure> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("add", Some(matches)) => {
            add::add_card(matches.value_of("name"))?;
        }
        ("browse", Some(_matches)) => {
            browse::browse_decks(matches.value_of("name"))?;
        }
        ("decks", Some(matches)) => {
            decks::view_deck(matches.value_of("name"))?;
        }
        ("init", Some(_matches)) => {
            init::init_recall()?;
        }
        ("new", Some(_matches)) => {
            new::new_deck(matches.value_of("name"))?;
        }
        _ => unreachable!()
    }

    Ok(())
}
