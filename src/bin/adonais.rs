extern crate reqwest;

extern crate adonais;

use reqwest::Error;

use adonais::keats::{Event, URI};

fn main() -> Result<(), Error> {
    let mut response = reqwest::get(URI)?;

    let events: Vec<Event> = response.json()?;
    println!("{:?}", events);
    Ok(())
}
