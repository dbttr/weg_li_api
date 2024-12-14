use anyhow::anyhow;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::util::{date_time_to_rfc3339, rfc3339_to_date_time};

#[derive(Debug)]
pub struct District {
    pub name: String,
    pub zip: String,
    pub email: String,
    pub prefixes: Vec<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub aliases: Option<Vec<String>>,
    /// Indicates whether the email is specific to a single person
    pub personal_email: bool,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistrictJson {
    pub name: String,
    pub zip: String,
    pub email: String,
    pub prefixes: Vec<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub aliases: Option<Vec<String>>,
    /// Indicates whether the email is specific to a single person
    pub personal_email: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl TryFrom<&DistrictJson> for District {
    type Error = anyhow::Error;
    fn try_from(value: &DistrictJson) -> Result<Self, Self::Error> {
        Ok(District {
            name: value.name.clone(),
            zip: value.zip.clone(),
            email: value.email.clone(),
            prefixes: value.prefixes.clone(),
            latitude: value.latitude,
            longitude: value.longitude,
            aliases: value.aliases.clone(),
            personal_email: value.personal_email,
            created_at: match rfc3339_to_date_time(&value.created_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            updated_at: match rfc3339_to_date_time(&value.updated_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
        })
    }
}

impl From<&District> for DistrictJson {
    fn from(value: &District) -> Self {
        DistrictJson {
            name: value.name.clone(),
            zip: value.zip.clone(),
            email: value.email.clone(),
            prefixes: value.prefixes.clone(),
            latitude: value.latitude,
            longitude: value.longitude,
            aliases: value.aliases.clone(),
            personal_email: value.personal_email,
            created_at: date_time_to_rfc3339(&value.created_at),
            updated_at: date_time_to_rfc3339(&value.updated_at),
        }
    }
}
