use std::str::FromStr;

use anyhow::anyhow;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::{
    charge::{Charge, ChargeJson},
    util::{date_time_to_rfc3339, rfc3339_to_date_time},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticePhotosJson {
    pub filename: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoticeJson {
    /// Token value of the notice
    pub token: String,
    /// Processing status of the notice
    pub status: String,
    // Street the offense was recorded in
    pub street: String,
    /// City the offense was recorded in
    pub city: String,
    /// Zip conde the offense was recorded in
    pub zip: String,
    /// Latitude of offense location
    pub latitude: f64,
    /// Longitude of offense location
    pub longitude: f64,
    /// Licence tag of the vehicle
    pub registration: String,
    /// Color of the vehicle
    pub color: String,
    /// Brand of the vehicle
    pub brand: String,
    /// Charge reported with the notice
    pub charge: ChargeJson,
    /// "Tatbestandsnummer"
    pub tbnr: String,
    /// Start timestamp of observing the offense
    pub start_date: String,
    /// End timestamp of observing the offense
    pub end_date: String,
    /// Free text field for additional notes and descriptions
    pub note: Option<String>,
    /// Photos attached to the notice as evidence
    pub photos: Vec<NoticePhotosJson>,
    /// Creation timestamp of the notice
    pub created_at: String,
    /// Update timestamp of the notice
    pub updated_at: String,
    /// Timestamp the notice was sent to the email responsible for the distict
    pub sent_at: String,
    /// Whether the vehicle was empty at the time of offense
    pub vehicle_empty: bool,
    /// Whether the hazard lights were on at the time of offense
    pub hazard_lights: bool,
    /// Whether the "TÜV" ("Technischer Überwachungsverein" - Technical Control Board) certification was expired on the vehicle
    pub expired_tuv: bool,
    /// Whether the waste gas examination certificate was expired on the vehicle
    pub expired_eco: bool,
    /// Whether the vehicle weighs more than 2.8 metric tons
    pub over_2_8_tons: bool,
}

#[derive(Debug)]
pub enum NoticeStatus {
    OPEN,
    DISABLED,
    ANALYZING,
    /// Notice has been sent to the responsible contact email of the district.
    SHARED,
}

impl std::str::FromStr for NoticeStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(NoticeStatus::OPEN),
            "disabled" => Ok(NoticeStatus::DISABLED),
            "analyzing" => Ok(NoticeStatus::ANALYZING),
            "shared" => Ok(NoticeStatus::SHARED),
            _ => Err(anyhow!("'{}' is not a valid NoticeStatus", s)),
        }
    }
}

impl ToString for NoticeStatus {
    fn to_string(&self) -> String {
        match self {
            NoticeStatus::OPEN => "open".to_string(),
            NoticeStatus::DISABLED => "disabled".to_string(),
            NoticeStatus::ANALYZING => "analyzing".to_string(),
            NoticeStatus::SHARED => "shared".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Notice {
    /// Token value of the notice
    pub token: String,
    /// Processing status of the notice
    pub status: NoticeStatus,
    // Street the offense was recorded in
    pub street: String,
    /// City the offense was recorded in
    pub city: String,
    /// Zip conde the offense was recorded in
    pub zip: String,
    /// Latitude of offense location
    pub latitude: f64,
    /// Longitude of offense location
    pub longitude: f64,
    /// Licence tag of the vehicle
    pub registration: String,
    /// Color of the vehicle
    pub color: String,
    /// Brand of the vehicle
    pub brand: String,
    /// Charge reported with the notice
    pub charge: Charge,
    /// "Tatbestandsnummer"
    pub tbnr: String,
    /// Start timestamp of observing the offense
    pub start_date: DateTime<FixedOffset>,
    /// End timestamp of observing the offense
    pub end_date: DateTime<FixedOffset>,
    /// Free text field for additional notes and descriptions
    pub note: Option<String>,
    /// Photos attached to the notice as evidence
    pub photos: Vec<NoticePhotosJson>,
    /// Creation timestamp of the notice
    pub created_at: DateTime<FixedOffset>,
    /// Update timestamp of the notice
    pub updated_at: DateTime<FixedOffset>,
    /// Timestamp the notice was sent to the email responsible for the distict
    pub sent_at: DateTime<FixedOffset>,
    /// Whether the vehicle was empty at the time of offense
    pub vehicle_empty: bool,
    /// Whether the hazard lights were on at the time of offense
    pub hazard_lights: bool,
    /// Whether the "TÜV" ("Technischer Überwachungsverein" - Technical Control Board) certification was expired on the vehicle
    pub expired_tuv: bool,
    /// Whether the waste gas examination certificate was expired on the vehicle
    pub expired_eco: bool,
    /// Whether the vehicle weighs more than 2.8 metric tons
    pub over_2_8_tons: bool,
}

impl TryFrom<&NoticeJson> for Notice {
    type Error = anyhow::Error;
    fn try_from(value: &NoticeJson) -> Result<Self, Self::Error> {
        Ok(Notice {
            token: value.token.clone(),
            status: match NoticeStatus::from_str(&value.status) {
                Ok(status) => status,
                Err(error) => return Err(anyhow!(error)),
            },
            street: value.street.clone(),
            city: value.city.clone(),
            zip: value.zip.clone(),
            latitude: value.latitude,
            longitude: value.longitude,
            registration: value.registration.clone(),
            color: value.color.clone(),
            brand: value.brand.clone(),
            charge: match Charge::try_from(&value.charge) {
                Ok(val) => val,
                Err(error) => return Err(anyhow!(error)),
            },
            tbnr: value.tbnr.clone(),
            start_date: match rfc3339_to_date_time(&value.start_date) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            end_date: match rfc3339_to_date_time(&value.end_date) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            note: value.note.clone(),
            photos: value.photos.clone(),
            created_at: match rfc3339_to_date_time(&value.created_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            updated_at: match rfc3339_to_date_time(&value.updated_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            sent_at: match rfc3339_to_date_time(&value.sent_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            vehicle_empty: value.vehicle_empty,
            hazard_lights: value.hazard_lights,
            expired_tuv: value.expired_tuv,
            expired_eco: value.expired_eco,
            over_2_8_tons: value.over_2_8_tons,
        })
    }
}

impl From<&Notice> for NoticeJson {
    fn from(value: &Notice) -> Self {
        NoticeJson {
            token: value.token.clone(),
            status: value.status.to_string(),
            street: value.street.clone(),
            city: value.city.clone(),
            zip: value.zip.clone(),
            latitude: value.latitude,
            longitude: value.longitude,
            registration: value.registration.clone(),
            color: value.color.clone(),
            brand: value.brand.clone(),
            charge: ChargeJson::from(&value.charge),
            tbnr: value.tbnr.clone(),
            start_date: date_time_to_rfc3339(&value.start_date),
            end_date: date_time_to_rfc3339(&value.end_date),
            note: value.note.clone(),
            photos: value.photos.clone(),
            created_at: date_time_to_rfc3339(&value.created_at),
            updated_at: date_time_to_rfc3339(&value.updated_at),
            sent_at: date_time_to_rfc3339(&value.sent_at),
            vehicle_empty: value.vehicle_empty,
            hazard_lights: value.hazard_lights,
            expired_tuv: value.expired_tuv,
            expired_eco: value.expired_eco,
            over_2_8_tons: value.over_2_8_tons,
        }
    }
}
