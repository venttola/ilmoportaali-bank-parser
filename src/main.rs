use serde_json::{Value, Error};

fn main() -> Result<(), Error> {
    let j = include_str!("../example_data/ilmodata.json");

    let parsed: Value = serde_json::from_str(j)?;

    println!("{:?}", parsed);
    Ok(())
}
