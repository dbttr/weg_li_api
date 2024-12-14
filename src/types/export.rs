use std::str::FromStr;

use anyhow::anyhow;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::util::{
    date_time_to_export_timestamp, date_time_to_rfc3339, export_timestamp_to_date_time,
    rfc3339_to_date_time,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDownload {
    /// Filename of the export
    pub filename: String,
    /// URL of the export to downlaod
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum ExportType {
    NOTICES,
}

impl std::str::FromStr for ExportType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "notices" => Ok(ExportType::NOTICES),
            _ => Err(anyhow!("'{}' is not a valid ExportType", s)),
        }
    }
}

impl ToString for ExportType {
    fn to_string(&self) -> String {
        match self {
            ExportType::NOTICES => "notices".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Export {
    pub export_type: ExportType,
    pub file_extension: String,
    pub created_at: DateTime<FixedOffset>,
    pub download: ExportDownload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportJson {
    pub export_type: String,
    pub file_extension: String,
    pub created_at: String,
    pub download: ExportDownload,
}

impl TryFrom<&ExportJson> for Export {
    type Error = anyhow::Error;
    fn try_from(value: &ExportJson) -> Result<Self, Self::Error> {
        Ok(Export {
            export_type: match ExportType::from_str(&value.export_type) {
                Ok(export_type) => export_type,
                Err(error) => return Err(anyhow!(error)),
            },
            file_extension: value.file_extension.clone(),
            created_at: match rfc3339_to_date_time(&value.created_at) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            download: ExportDownload {
                filename: value.download.filename.clone(),
                url: value.download.url.clone(),
            },
        })
    }
}

impl From<&Export> for ExportJson {
    fn from(value: &Export) -> Self {
        ExportJson {
            export_type: value.export_type.to_string(),
            file_extension: value.file_extension.clone(),
            created_at: date_time_to_rfc3339(&value.created_at),
            download: ExportDownload {
                filename: value.download.filename.clone(),
                url: value.download.url.clone(),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ExportNoticeCsv {
    pub start_date: String,
    pub end_date: String,
    pub tbnr: String,
    pub street: String,
    pub city: String,
    pub zip: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug)]
pub struct ExportNotice {
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
    pub tbnr: String,
    pub street: String,
    pub city: String,
    pub zip: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl TryFrom<&ExportNoticeCsv> for ExportNotice {
    type Error = anyhow::Error;
    fn try_from(value: &ExportNoticeCsv) -> Result<Self, Self::Error> {
        Ok(ExportNotice {
            start_date: match export_timestamp_to_date_time(&value.start_date) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            end_date: match export_timestamp_to_date_time(&value.end_date) {
                Err(error) => return Err(anyhow!(error)),
                Ok(val) => val,
            },
            tbnr: value.tbnr.clone(),
            street: value.street.clone(),
            city: value.city.clone(),
            zip: value.zip.clone(),
            latitude: value.latitude,
            longitude: value.longitude,
        })
    }
}

impl From<&ExportNotice> for ExportNoticeCsv {
    fn from(value: &ExportNotice) -> Self {
        ExportNoticeCsv {
            start_date: date_time_to_export_timestamp(&value.start_date),
            end_date: date_time_to_export_timestamp(&value.end_date),
            tbnr: value.tbnr.clone(),
            street: value.street.clone(),
            city: value.city.clone(),
            zip: value.zip.clone(),
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}
