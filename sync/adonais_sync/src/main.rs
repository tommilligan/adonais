extern crate reqwest;

extern crate adonais_core;

use reqwest::Error;

use adonais_core::keats::{Event, URI};

fn main() -> Result<(), Error> {
    let mut response = reqwest::get(URI)?;
    let events: Vec<Event> = response.json()?;
    println!("{:?}", events);
    Ok(())
}
