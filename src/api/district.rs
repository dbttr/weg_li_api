use url::Url;

use crate::types::{
    district::{District, DistrictJson},
    request::RetrySettings,
};

use super::{
    error::ApiError,
    request::{execute_request, RetryData, DEFAULT_RETRY_SETTINGS},
};

pub async fn get_district_from_wegli_api(
    api_url: &Url,
    api_token: &String,
    zip: &String,
    retry_settings: &Option<RetrySettings>,
) -> Result<District, ApiError> {
    let retry_data = RetryData {
        retry_count: 0,
        settings: match retry_settings {
            Some(settings) => settings.clone(),
            None => DEFAULT_RETRY_SETTINGS,
        },
    };
    let request_builder = reqwest::Client::new()
        .get(format!("{}{}{}", api_url, "districts/", zip))
        .header("X-API-KEY", api_token);

    let response = match execute_request(&request_builder, &Some(retry_data)).await {
        Err(error) => return Err(error),
        Ok(response) => response,
    };

    match response.json::<DistrictJson>().await {
        Err(error) => return Err(ApiError::Deserialize(error)),
        Ok(val) => match District::try_from(&val) {
            Err(error) => {
                return Err(ApiError::Conversion(format!(
                    "failed to convert '{:?}': {}",
                    &val, error
                )))
            }
            Ok(charge) => return Ok(charge),
        },
    };
}

pub async fn get_districts_from_wegli_api(
    api_url: &Url,
    api_token: &String,
    retry_settings: &Option<RetrySettings>,
) -> Result<Vec<District>, ApiError> {
    let retry_data = RetryData {
        retry_count: 0,
        settings: match retry_settings {
            Some(settings) => settings.clone(),
            None => DEFAULT_RETRY_SETTINGS,
        },
    };
    let request_builder = reqwest::Client::new()
        .get(format!("{}{}", api_url, "districts"))
        .header("X-API-KEY", api_token);

    let response = match execute_request(&request_builder, &Some(retry_data)).await {
        Err(error) => return Err(error),
        Ok(response) => response,
    };

    match response.json::<Vec<DistrictJson>>().await {
        Err(error) => return Err(ApiError::Deserialize(error)),
        Ok(val) => {
            let mut districts: Vec<District> = vec![];
            for item in val {
                match District::try_from(&item) {
                    Err(error) => {
                        return Err(ApiError::Conversion(format!(
                            "failed to convert '{:?}': {}",
                            &item, error
                        )))
                    }
                    Ok(district) => districts.push(district),
                }
            }
            return Ok(districts);
        }
    };
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use url::Url;

    use super::{get_district_from_wegli_api, get_districts_from_wegli_api};

    #[tokio::test]
    async fn test_get_district_from_wegli_api() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/districts/91443")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
                {
                    "name": "Scheinfeld",
                    "zip": "91443",
                    "email": "info@vgem.scheinfeld.de",
                    "prefixes": [
                        "NEA",
                        "SEF",
                        "UFF"
                    ],
                    "latitude": 49.6653406,
                    "longitude": 10.462567,
                    "aliases": [],
                    "personal_email": false,
                    "created_at": "2024-03-13T04:43:59.602+01:00",
                    "updated_at": "2024-03-13T22:12:03.399+01:00"
                }"#,
            )
            .create_async()
            .await;

        let response = get_district_from_wegli_api(
            &Url::from_str(&server.url()).unwrap(),
            &"any_api_key".to_string(),
            &"91443".to_string(),
            &None,
        )
        .await
        .unwrap();
        assert_eq!(&response.latitude, &49.6653406);
        mock.assert();
    }

    #[tokio::test]
    async fn test_get_districtss_from_wegli_api() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/districts")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
                [
                    {
                        "name": "Scheinfeld",
                        "zip": "91443",
                        "email": "info@vgem.scheinfeld.de",
                        "prefixes": [
                            "NEA",
                            "SEF",
                            "UFF"
                        ],
                        "latitude": 49.6653406,
                        "longitude": 10.462567,
                        "aliases": [],
                        "personal_email": false,
                        "created_at": "2024-03-13T04:43:59.602+01:00",
                        "updated_at": "2024-03-13T22:12:03.399+01:00"
                    },
                    {
                        "name": "Meinerzhagen",
                        "zip": "58540",
                        "email": "post@meinerzhagen.de",
                        "prefixes": [
                            "MK"
                        ],
                        "latitude": 51.1206595,
                        "longitude": 7.7331115,
                        "aliases": null,
                        "personal_email": false,
                        "created_at": "2019-09-24T14:56:35.624+02:00",
                        "updated_at": "2020-03-06T18:02:53.389+01:00"
                    }
                ]"#,
            )
            .create_async()
            .await;

        let response = get_districts_from_wegli_api(
            &Url::from_str(&server.url()).unwrap(),
            &"any_api_key".to_string(),
            &None,
        )
        .await
        .unwrap();
        assert_eq!(&response[0].zip, &"91443".to_string());
        mock.assert();
    }
}
