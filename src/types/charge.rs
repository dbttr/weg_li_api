use anyhow::anyhow;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::util::{date_time_to_rfc3339, rfc3339_to_date_time};

#[derive(Debug, Serialize, Deserialize)]
/// A [charge](https://www.weg.li/charges) as received by the API call.
pub struct ChargeJson {
    /// The "Tatbestandsnummer", a unique identifier for the element of an offense ("Tatbestand")
    pub tbnr: String,
    pub description: String,
    /// The fine associated, in Euros as stringified float
    pub fine: String,
    pub bkat: String,
    pub penalty: Option<String>,
    pub fap: Option<String>,
    pub points: Option<u8>,
    /// Start date of legal validity
    pub valid_from: Option<String>,
    /// End date of legal validity
    pub valid_to: Option<String>,
    pub implementation: Option<u8>,
    pub classification: u8,
    pub variant_table_id: Option<u32>,
    pub rule_id: u16,
    pub table_id: Option<u32>,
    pub required_refinements: String,
    pub number_required_refinements: u8,
    pub max_fine: String,
    /// Creation date of the charge in weg.li
    pub created_at: String,
    /// Update date of the charge in weg.li
    pub updated_at: String,
}

#[derive(Debug)]
/// A [charge](https://www.weg.li/charges) with fields parsed to structured types.
pub struct Charge {
    /// The "Tatbestandsnummer", a unique identifier for the element of an offense ("Tatbestand")
    pub tbnr: String,
    pub description: String,
    /// The fine associated, in Euros
    pub fine: f64,
    pub bkat: String,
    pub penalty: Option<String>,
    pub fap: Option<String>,
    pub points: Option<u8>,
    /// Start date of legal validity
    pub valid_from: Option<DateTime<FixedOffset>>,
    /// End date of legal validity
    pub valid_to: Option<DateTime<FixedOffset>>,
    pub implementation: Option<u8>,
    pub classification: u8,
    pub variant_table_id: Option<u32>,
    pub rule_id: u16,
    pub table_id: Option<u32>,
    pub required_refinements: String,
    pub number_required_refinements: u8,
    pub max_fine: f64,
    /// Creation date of the charge in weg.li
    pub created_at: DateTime<FixedOffset>,
    /// Update date of the charge in weg.li
    pub updated_at: DateTime<FixedOffset>,
}

impl TryFrom<&ChargeJson> for Charge {
    type Error = anyhow::Error;

    fn try_from(value: &ChargeJson) -> Result<Self, Self::Error> {
        let charge = Charge {
            tbnr: value.tbnr.clone(),
            description: value.description.clone(),
            fine: match value.fine.parse::<f64>() {
                Ok(val) => val,
                Err(error) => return Err(anyhow!(error)),
            },
            bkat: value.bkat.clone(),
            penalty: value.penalty.clone(),
            fap: value.fap.clone(),
            points: value.points,
            valid_from: {
                match &value.valid_from {
                    None => None,
                    Some(valid_from_str) => match rfc3339_to_date_time(&valid_from_str) {
                        Err(error) => return Err(anyhow!(error)),
                        Ok(val) => Some(val),
                    },
                }
            },
            valid_to: {
                match &value.valid_to {
                    None => None,
                    Some(valid_to_str) => match rfc3339_to_date_time(&valid_to_str) {
                        Err(error) => return Err(anyhow!(error)),
                        Ok(val) => Some(val),
                    },
                }
            },
            implementation: value.implementation,
            classification: value.classification,
            variant_table_id: value.variant_table_id,
            rule_id: value.rule_id,
            table_id: value.table_id,
            required_refinements: value.required_refinements.clone(),
            number_required_refinements: value.number_required_refinements,
            max_fine: match value.max_fine.parse::<f64>() {
                Ok(val) => val,
                Err(error) => return Err(anyhow!(error)),
            },
            created_at: match rfc3339_to_date_time(&value.created_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            updated_at: match rfc3339_to_date_time(&value.updated_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
        };
        Ok(charge)
    }
}

impl From<&Charge> for ChargeJson {
    fn from(value: &Charge) -> Self {
        ChargeJson {
            tbnr: value.tbnr.clone(),
            description: value.description.clone(),
            fine: value.fine.to_string(),
            bkat: value.bkat.clone(),
            penalty: value.penalty.clone(),
            fap: value.fap.clone(),
            points: value.points,
            valid_from: match &value.valid_from {
                None => None,
                Some(valid_from) => Some(date_time_to_rfc3339(valid_from)),
            },
            valid_to: match &value.valid_to {
                None => None,
                Some(valid_to) => Some(date_time_to_rfc3339(valid_to)),
            },
            implementation: value.implementation,
            classification: value.classification,
            variant_table_id: value.variant_table_id,
            rule_id: value.rule_id,
            table_id: value.table_id,
            required_refinements: value.required_refinements.clone(),
            number_required_refinements: value.number_required_refinements,
            max_fine: value.max_fine.to_string(),
            created_at: date_time_to_rfc3339(&value.created_at),
            updated_at: date_time_to_rfc3339(&value.updated_at),
        }
    }
}
