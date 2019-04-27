#[macro_use]
extern crate clap;

extern crate rand;
extern crate recall;

use clap::{App, ArgMatches};
use regex::{self, Regex};
use std::process;
use recall::{card::Card, file, learn};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Err(report) = execute(matches) {
        eprintln!("{}", report);
        process::exit(1);
    }
}

fn execute(matches: ArgMatches) -> Result<(), String> {
    if let Some(matches) = matches.subcommand_matches("add") {
        let question = matches.value_of("question").unwrap();
        let answer = matches.value_of("answer").unwrap();

        return if matches.is_present("bidir") {
            add(&[(question, answer), (answer, question)])
        } else {
            add(&[(question, answer)])
        };
    }

    if let Some(matches) = matches.subcommand_matches("find") {
        let regexp = matches.value_of("regex").unwrap();
        return find(regexp);
    }

    if let Some(_matches) = matches.subcommand_matches("learn") {
        learn::learning_loop()?;
    }

    println!("Welcome to Recall!");
    println!("A flashcard CLI that uses spaced repetition for improved recall.");

    Ok(())
}

fn find(regex: &str) -> Result<(), String> {
    let regex = match Regex::new(regex) {
        Ok(regex) => regex,
        Err(why) => return Err(format!("Invalid regex: {}", why)),
    };

    let reader = file::read_cards()?;
    for card in reader {
        let card = match card {
            Ok(card) => card,
            Err(why) => return Err(why),
        };
        if !regex.is_match(card.question()) && !regex.is_match(card.answer()) {
            continue;
        }
        print!("{}", card.to_line());
    }

    Ok(())
}

fn add(qa: &[(&str, &str)]) -> Result<(), String> {
    let reader = file::read_cards()?;

    let last_id: u64 = match reader.last() {
        Some(Ok(card)) => card.id(),
        Some(Err(error)) => return Err(error),
        None => 0,
    };

    let cards: Vec<Card> = qa
        .iter()
        .scan(last_id, |last_id, &(q, a)| {
            *last_id += 1;
            Some(Card::new(*last_id, String::from(q), String::from(a)))
        })
        .collect();
    file::store_cards(&cards)
}
