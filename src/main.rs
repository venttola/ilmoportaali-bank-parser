mod dom;

use serde_json::{Value, Error};

fn main() -> Result<(), Error> {
    let j = include_str!("../example_data/ilmodata.json");

    let parsed: Vec<dom::Group> = serde_json::from_str(j)?;

    println!("{:?}", parsed);
    Ok(())
}
