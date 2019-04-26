#[macro_use]
extern crate clap;
use clap::App;

use exitfailure::ExitFailure;
use failure::ResultExt;

mod add;
mod browse;
mod decks;
mod new;

fn main() -> Result<(), ExitFailure> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("add", Some(matches)) => {
            add::add_card(matches.value_of("name").unwrap())?;
        }
        ("browse", Some(_matches)) => {
            browse::browse_decks(matches.value_of("name").unwrap())?;
        }
        ("decks", Some(matches)) => {
            decks::new_site(matches.value_of("name").unwrap())?;
        }
        ("new", Some(_matches)) => {
            new::new_site(matches.value_of("name").unwrap())?;
        }
        _ => unreachable!()
    }

    Ok(())
}
