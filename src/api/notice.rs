use crate::types::{
    notice::{Notice, NoticeJson},
    request::RetrySettings,
};

use super::{
    error::ApiError,
    request::{execute_request, RetryData, DEFAULT_RETRY_SETTINGS},
};

pub async fn get_notice_from_wegli_api(
    api_url: &String,
    api_token: &String,
    notice_token: &String,
    retry_settings: &Option<RetrySettings>,
) -> Result<Notice, ApiError> {
    let retry_data = RetryData {
        retry_count: 0,
        settings: match retry_settings {
            Some(settings) => settings.clone(),
            None => DEFAULT_RETRY_SETTINGS,
        },
    };
    let request_builder = reqwest::Client::new()
        .get(format!("{}{}{}", api_url, "/notices/", notice_token))
        .header("X-API-KEY", api_token);

    let response = match execute_request(&request_builder, &Some(retry_data)).await {
        Err(error) => return Err(error),
        Ok(response) => response,
    };

    match response.json::<NoticeJson>().await {
        Err(error) => return Err(ApiError::Deserialize(error)),
        Ok(val) => match Notice::try_from(&val) {
            Err(error) => {
                return Err(ApiError::Conversion(format!(
                    "failed to convert '{:?}': {}",
                    &val, error
                )))
            }
            Ok(notice) => return Ok(notice),
        },
    };
}

pub async fn get_notices_from_wegli_api(
    api_url: &String,
    api_token: &String,
    retry_settings: &Option<RetrySettings>,
) -> Result<Vec<Notice>, ApiError> {
    let retry_data = RetryData {
        retry_count: 0,
        settings: match retry_settings {
            Some(settings) => settings.clone(),
            None => DEFAULT_RETRY_SETTINGS,
        },
    };
    let request_builder = reqwest::Client::new()
        .get(format!("{}{}", api_url, "/notices"))
        .header("X-API-KEY", api_token);

    let response = match execute_request(&request_builder, &Some(retry_data)).await {
        Err(error) => return Err(error),
        Ok(response) => response,
    };

    match response.json::<Vec<NoticeJson>>().await {
        Err(error) => return Err(ApiError::Deserialize(error)),
        Ok(val) => {
            let mut notices: Vec<Notice> = vec![];
            for item in val {
                match Notice::try_from(&item) {
                    Err(error) => {
                        return Err(ApiError::Conversion(format!(
                            "failed to convert '{:?}': {}",
                            &item, error
                        )))
                    }
                    Ok(notice) => notices.push(notice),
                }
            }
            return Ok(notices);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::get_notice_from_wegli_api;

    #[tokio::test]
    async fn test_get_notice_from_wegli_api() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/notices/abc123")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"{
                    "token": "abc123",
                    "status": "shared",
                    "street": "Hauptstraße 1",
                    "city": "Metropolis",
                    "zip": "12345",
                    "latitude": 71.005523,
                    "longitude": 41.575962,
                    "registration": "XX YYY 123",
                    "color": "silver",
                    "brand": "Chitty Chitty Bang Bang",
                    "charge": {
                        "tbnr": "112454",
                        "description": "Sie parkten verbotswidrig auf dem Gehweg.",
                        "fine": "55.0",
                        "bkat": "§ 12 Abs. 4, § 49 StVO; § 24 Abs. 1, 3 Nr. 5 StVG; 52a BKat",
                        "penalty": null,
                        "fap": null,
                        "points": 0,
                        "valid_from": "2021-11-09T00:00:00.000+01:00",
                        "valid_to": null,
                        "implementation": null,
                        "classification": 5,
                        "variant_table_id": 712031,
                        "rule_id": 272,
                        "table_id": null,
                        "required_refinements": "00000000000000000000000000000000",
                        "number_required_refinements": 0,
                        "max_fine": "0.0",
                        "created_at": "2023-09-18T15:30:27.417+02:00",
                        "updated_at": "2023-09-18T15:30:27.417+02:00"
                    },
                    "tbnr": "112454",
                    "start_date": "2023-10-25T09:23:00.000+01:00",
                    "end_date": "2023-10-25T09:41:00.000+01:00",
                    "note": null,
                    "photos": [
                        {
                            "filename": "20231025_092230.jpg",
                            "url": "https://www.weg.li/rails/active_storage/blobs/redirect/.../20231025_092230.jpg"
                        }
                    ],
                    "created_at": "2023-10-25T09:23:30.830+01:00",
                    "updated_at": "2023-10-25T09:41:42.638+01:00",
                    "sent_at": "2023-10-25T09:42:32.612+01:00",
                    "vehicle_empty": true,
                    "hazard_lights": false,
                    "expired_tuv": false,
                    "expired_eco": false,
                    "over_2_8_tons": false
                }"#,
            )
            .create_async()
            .await;

        let response = get_notice_from_wegli_api(
            &server.url(),
            &"any_api_key".to_string(),
            &"abc123".to_string(),
            &None,
        )
        .await
        .unwrap();
        assert_eq!(&response.zip, &"12345".to_string());
        mock.assert();
    }
}
