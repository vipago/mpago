use serde::{Deserialize, Serialize};

use crate::payments::types::{EntityType, IdentificationType, PhoneNumber};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Payer {
    pub entity_type: Option<EntityType>,
    pub r#type: Option<PayerType>,
    pub id: Option<String>,
    pub email: String,
    pub identification: Option<PayerIdentification>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdditionalInfoPayer {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<PhoneNumber>,
    pub address: Option<PayerAddress>,
    pub registration_date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayerAddress {
    pub zip_code: String,
    pub street_name: String,
    pub street_number: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayerIdentification {
    pub r#type: IdentificationType,
    pub number: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PayerType {
    Customer,
    Registered,
    Guest,
}
