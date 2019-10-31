use clap::{App, Arg};

pub const ARGNAME_ILMODATA: &str = "ilmodata JSON";
pub const ARGNAME_BANKDATA: &str = "bankdata NDA";

pub fn cli() -> clap::ArgMatches<'static> {
    App::new("Ilmoportaali bank parser")
        .version("1.0")
        .author("Henri L. <henri.lunnikivi@gmail.com>")
        .about(
            "Uses Nordea's .nda and Ilmoportaali's .json to determine which groups have not paid their enrolment fees and outputs it as .csv.",
        )
        .arg(
            Arg::with_name(ARGNAME_ILMODATA)
                .help("Sets the file for ilmoportaali JSON with enrolment information")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name(ARGNAME_BANKDATA)
                .help("Sets the file for bank data NDA with payment information")
                .required(true)
                .index(2),
        )
        .get_matches()
}
