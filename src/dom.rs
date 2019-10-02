#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub type Euro = f64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    #[serde(rename = "group")]
    pub meta: GroupMeta,
    #[serde(rename = "isPaid")]
    is_paid: bool,
    payments: Vec<PersonPayment>,
    #[serde(rename = "totalSum")]
    pub total_sum: Euro,
    #[serde(rename = "refNumber")]
    pub ref_number: String,
    #[serde(rename = "organizationBankAccount")]
    pub organization_bank_account: BankAccount,
    barcode: Barcode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupMeta {
    pub name: String,
    description: String,
    id: u16,
    platoon: Vec<Platoon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Platoon {
    name: String,
    id: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonPayment {
    #[serde(rename = "member")]
    name: String,
    #[serde(rename = "productSum")]
    pub product_sum: Euro,
    #[serde(rename = "discountSum")]
    pub discount_sum: Euro,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankAccount(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Barcode(String);

impl BankAccount {
    fn from_str(s: &str) -> BankAccount {
        // validate country code
        assert!(s[0..2].is_ascii());

        // validate length
        assert_eq!(s.len(), 18);

        BankAccount(s.to_string())
    }
}

impl Barcode {
    fn from_str(s: &str) -> Barcode {
        // validate length
        assert_eq!(s.len(), 54);

        Barcode(s.to_string())
    }
}
