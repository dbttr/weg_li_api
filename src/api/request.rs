use reqwest::Response;

use crate::types::request::RetrySettings;

use super::error::ApiError;
use std::{thread, time};

#[derive(Debug, Clone)]
pub struct RetryData {
    pub settings: RetrySettings,
    pub retry_count: u32,
}

pub const DEFAULT_RETRY_SETTINGS: RetrySettings = RetrySettings {
    initial_backoff_ms: 300,
    max_retries: 5,
    backoff_multiplier: 2,
};

fn get_retry_after_header(response: &Response) -> Option<u64> {
    match response
        .headers()
        .iter()
        .find(|header| header.0 == "Retry-After")
    {
        None => None,
        Some((_, header_value)) => match header_value.to_str() {
            Err(_) => None,
            Ok(value_str) => match value_str.parse::<u64>() {
                Err(_) => None,
                Ok(val) => Some(val),
            },
        },
    }
}

pub async fn execute_request(
    request_builder: &reqwest::RequestBuilder,
    retry_data: &Option<RetryData>,
) -> Result<reqwest::Response, ApiError> {
    let local_request_builder = match request_builder.try_clone() {
        None => {
            // if request is streaming body, cloning won't work
            return Err(ApiError::RequestBuilderClone());
        }
        Some(val) => val,
    };
    let response = match local_request_builder.send().await {
        Err(error) => return Err(ApiError::Reqwest(error)),
        Ok(val) => val,
    };

    if !response.status().is_success() {
        if [429, 503].contains(&response.status().as_u16()) {
            match get_retry_after_header(&response) {
                None => return Err(ApiError::ApiRequestsWait(None)),
                Some(retry_after_value) => {
                    return Err(ApiError::ApiRequestsWait(Some(retry_after_value)))
                }
            }
        }
        match retry_data {
            None => return Err(ApiError::UnexpectedStatusCode(response.status())),
            Some(retry_data) => {
                let mut iter_retry_data = retry_data.clone();
                if iter_retry_data.retry_count == iter_retry_data.settings.max_retries
                    || iter_retry_data.settings.max_retries == 0
                {
                    return Err(ApiError::UnexpectedStatusCode(response.status()));
                }
                iter_retry_data.retry_count += 1;

                thread::sleep(time::Duration::from_millis(
                    iter_retry_data.settings.initial_backoff_ms
                        * match iter_retry_data
                            .settings
                            .backoff_multiplier
                            .checked_pow(iter_retry_data.retry_count)
                        {
                            None => {
                                return Err(ApiError::BackoffOverflow(
                                    "exceeded maximum backoff value".to_string(),
                                ))
                            }
                            Some(exp) => exp,
                        },
                ));

                return Box::pin(execute_request(request_builder, &Some(iter_retry_data))).await;
            }
        }
    }

    Ok(response)
}
