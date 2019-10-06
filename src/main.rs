mod dom;
mod nda;

use serde_json::Error;

fn main() -> Result<(), Error> {
    let payments_by_ref = {
        let nda_file = include_str!("../example_data/bankdata.NDA");
        nda::scrape_payments(nda_file)
    };

    let enrolment_info = include_str!("../example_data/ilmodata.json");
    let parsed: Vec<dom::Group> = serde_json::from_str(enrolment_info)?;

    for group in &parsed {
        let name = &group.meta.name;
        let required = group.total_sum;
        let ref_number = &group.ref_number;
        let payments = {
            match payments_by_ref.get(ref_number) {
                Some(amount) => *amount,
                None => 0f64,
            }
        };

        println!(
            "Ryhmän '{}' pitäisi maksaa {} €. Ryhmä on maksanut {} €. Erotus: {} €.",
            name,
            required,
            payments,
            payments - required
        );
    }

    Ok(())
}
