mod cli;
mod dom;
mod nda;

use cli::{cli, ARGNAME_BANKDATA, ARGNAME_ILMODATA};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // read command line arguments
    let matches = cli();

    // read json file into enrolment info by group
    let enrolment_file = matches.value_of(ARGNAME_ILMODATA).unwrap();
    let groups = enrolment_info_from_json_file(enrolment_file);

    // read nda file into payments by reference number (viitenumero)
    let payments_file = matches.value_of(ARGNAME_BANKDATA).unwrap();
    let payments_by_ref = bankdata_from_nda_file(payments_file);

    println!("group_name, payments, missing_payments_eur");
    for group in &groups {
        let name = &group.meta.name;
        let required = group.total_sum;
        let ref_number = &group.ref_number;
        let payments = {
            match payments_by_ref.get(ref_number) {
                Some(amount) => *amount,
                None => 0f64,
            }
        };

        let to_be_paid = required - payments;
        println!("{}, {}, {}", name, payments, to_be_paid);
    }
}

/// Scrapes payments by reference number (viitenumero) from given .NDA file.
fn bankdata_from_nda_file(filename: &str) -> HashMap<String, f64> {
    let path = Path::new(filename);
    let mut file = File::open(path).unwrap_or_else(|_| panic!("cannot open file {:?}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("cannot read file {:?}", path));

    // scrape
    nda::scrape_payments(contents)
}

/// Deserializes enrolment information from a JSON source.
fn enrolment_info_from_json_file(filename: &str) -> Vec<dom::Group> {
    let path = Path::new(filename);
    let mut file = File::open(path).unwrap_or_else(|_| panic!("cannot open file {:?}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("cannot read file {:?}", path));

    // deserialize
    serde_json::from_str(&contents).expect("contents of {:?} do not parse into predefined schema")
}
