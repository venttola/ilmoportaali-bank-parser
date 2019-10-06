mod dom;
mod nda;

use serde_json::Error;

fn main() -> Result<(), Error> {
    let j = include_str!("../example_data/ilmodata.json");

    let parsed: Vec<dom::Group> = serde_json::from_str(j)?;

    for group in &parsed {
        let name = &group.meta.name;
        let required = group.total_sum;
        let account = &group.organization_bank_account;
        let ref_number = &group.ref_number;
        println!(
            "Ryhmän '{}', pitäisi maksaa {} €. Tilinumero: {:?}. Viitenumero: {:?}.",
            name, required, account, ref_number
        );
    }

    let nda_file = include_str!("../example_data/bankdata.NDA");
    let payments_by_ref = nda::scrape_payments(nda_file);
    println!("Maksut viitenumeron perusteella: {:?}", &payments_by_ref);

    Ok(())
}
