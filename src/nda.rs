use std::collections::HashMap;

/// Scrapes payments by reference number from .NDA
pub fn scrape_payments<S>(nda: S) -> HashMap<String, f64>
where
    S: Into<String>,
{
    let mut payments_by_ref = HashMap::new();

    let nda = nda.into();
    let lines = nda.lines();

    for line in lines {
        let tokens: Vec<_> = line.split_whitespace().collect();

        // Line needs to have at least 3 elements to be valid:
        //   'Viitemaksu' in the 1st element, payment sum in the 2nd
        //   element, and reference number in the last element.
        if tokens.len() < 3 {
            continue;
        }

        // find lines ending with "Viitemaksu" in the first token, these are the payment lines
        let search_for = "Viitemaksu";
        let token1 = tokens.first().unwrap();
        let found_at = token1.rfind("Viitemaksu");
        let is_payment_line = match found_at {
            None => false,
            Some(at) => at == token1.len() - search_for.len(),
        };

        if !is_payment_line {
            continue;
        }

        // 2nd token contains the amount paid
        let token2 = tokens.iter().nth(1).unwrap();
        let amount_paid_raw = token2
            .parse::<i32>()
            .expect("Error in parsing i32 from 2nd token on 'Viitemaksu' line. Malformed .nda or incorrect parser implementation.");
        // convert to Eur
        let amount_paid_eur = amount_paid_raw as f64 / 100.;

        let last_token = tokens.last().unwrap();
        assert_eq!(last_token.len(), 20, "Error in parsing 'Viitenumero' from last token on 'Viitemaksu' line. Token is not exactly 20 characters long. Malformed .nda or incorrect parser implementation.");
        let ref_number = last_token.trim_start_matches("0");

        if let Some(x) = payments_by_ref.get_mut(ref_number) {
            *x += amount_paid_eur;
        } else {
            payments_by_ref.insert(ref_number.to_string(), amount_paid_eur);
        }
    }

    payments_by_ref
}
