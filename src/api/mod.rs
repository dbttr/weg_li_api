mod charge;
mod district;
pub mod error;
mod export;
mod notice;
mod request;
mod util;

use std::path::PathBuf;

use charge::{get_charge_from_wegli_api, get_charges_from_wegli_api};
use district::{get_district_from_wegli_api, get_districts_from_wegli_api};
use error::ApiError;
use export::{download_latest_export_from_wegli, get_exports_from_wegli_api};
use notice::{get_notice_from_wegli_api, get_notices_from_wegli_api};

use crate::types::{
    charge::Charge, district::District, export::Export, notice::Notice, request::RetrySettings,
};

pub struct WegLiApiClient {
    api_url: String,
    api_token: String,
    /// Retry settings for exponential backoff are activated by default (initial_backoff_ms: 300, max_retries: 5, backoff_multiplier: 2).
    /// If you do not want to retry, provide a retry_settings argument with max_retries set to 0.
    pub retry_settings: Option<RetrySettings>,
}

impl WegLiApiClient {
    pub fn new(
        api_url: &String,
        api_token: &String,
        retry_settings: Option<RetrySettings>,
    ) -> Self {
        WegLiApiClient {
            api_url: api_url.to_string(),
            api_token: api_token.to_string(),
            retry_settings,
        }
    }
    /// Get a single notice of the authenticated user by its token
    pub async fn get_notice(&self, notice_token: &String) -> Result<Notice, ApiError> {
        return get_notice_from_wegli_api(
            &self.api_url,
            &self.api_token,
            notice_token,
            &self.retry_settings,
        )
        .await;
    }
    /// Get all notices of the authenticated user
    pub async fn get_notices(&self) -> Result<Vec<Notice>, ApiError> {
        return get_notices_from_wegli_api(&self.api_url, &self.api_token, &self.retry_settings)
            .await;
    }
    /// Get a single charge by its tbnr
    pub async fn get_charge(&self, tbnr: &String) -> Result<Charge, ApiError> {
        return get_charge_from_wegli_api(
            &self.api_url,
            &self.api_token,
            tbnr,
            &self.retry_settings,
        )
        .await;
    }
    /// Get all charges
    pub async fn get_charges(&self) -> Result<Vec<Charge>, ApiError> {
        return get_charges_from_wegli_api(&self.api_url, &self.api_token, &self.retry_settings)
            .await;
    }
    /// Get a single district by zip code
    pub async fn get_district(&self, zip: &String) -> Result<District, ApiError> {
        return get_district_from_wegli_api(
            &self.api_url,
            &self.api_token,
            zip,
            &self.retry_settings,
        )
        .await;
    }
    /// Get all districts
    pub async fn get_districts(&self) -> Result<Vec<District>, ApiError> {
        return get_districts_from_wegli_api(&self.api_url, &self.api_token, &self.retry_settings)
            .await;
    }
    /// Get metadata of exports of the currently authenticated user
    pub async fn get_user_exports(&self) -> Result<Vec<Export>, ApiError> {
        return get_exports_from_wegli_api(
            &self.api_url,
            &self.api_token,
            false,
            &self.retry_settings,
        )
        .await;
    }
    /// Get metadata of all public exports
    pub async fn get_public_exports(&self) -> Result<Vec<Export>, ApiError> {
        return get_exports_from_wegli_api(
            &self.api_url,
            &self.api_token,
            true,
            &self.retry_settings,
        )
        .await;
    }
    /// Download the latest notice export archive
    ///
    /// The `path` is where the zip archive is downloaded and extracted if `unzip` is `true`.
    ///
    /// `public` gets the publicly available export if set to `true`, otherwise the authenticated user's ones.
    ///
    /// Returns the path to the zip file if `unzip` is `false``, otherwise the path to the first (and as of current weg.li behavior only) .csv file extracted.
    pub async fn download_latest_export(
        &self,
        path: &String,
        public: bool,
        unzip: bool,
    ) -> Result<PathBuf, anyhow::Error> {
        return download_latest_export_from_wegli(
            &self.api_url,
            &self.api_token,
            path,
            public,
            unzip,
            &self.retry_settings,
        )
        .await;
    }
}
