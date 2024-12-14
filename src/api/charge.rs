use crate::types::{
    charge::{Charge, ChargeJson},
    request::RetrySettings,
};

use super::{
    error::ApiError,
    request::{execute_request, RetryData, DEFAULT_RETRY_SETTINGS},
};

pub async fn get_charge_from_wegli_api(
    api_url: &String,
    api_token: &String,
    tbnr: &String,
    retry_settings: &Option<RetrySettings>,
) -> Result<Charge, ApiError> {
    let retry_data = RetryData {
        retry_count: 0,
        settings: match retry_settings {
            Some(settings) => settings.clone(),
            None => DEFAULT_RETRY_SETTINGS,
        },
    };
    let request_builder = reqwest::Client::new()
        .get(format!("{}{}{}", api_url, "/charges/", tbnr))
        .header("X-API-KEY", api_token);

    let response = match execute_request(&request_builder, &Some(retry_data)).await {
        Err(error) => return Err(error),
        Ok(response) => response,
    };

    match response.json::<ChargeJson>().await {
        Err(error) => return Err(ApiError::Deserialize(error)),
        Ok(val) => match Charge::try_from(&val) {
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

pub async fn get_charges_from_wegli_api(
    api_url: &String,
    api_token: &String,
    retry_settings: &Option<RetrySettings>,
) -> Result<Vec<Charge>, ApiError> {
    let retry_data = RetryData {
        retry_count: 0,
        settings: match retry_settings {
            Some(settings) => settings.clone(),
            None => DEFAULT_RETRY_SETTINGS,
        },
    };
    let request_builder = reqwest::Client::new()
        .get(format!("{}{}", api_url, "/charges"))
        .header("X-API-KEY", api_token);

    let response = match execute_request(&request_builder, &Some(retry_data)).await {
        Err(error) => return Err(error),
        Ok(response) => response,
    };

    match response.json::<Vec<ChargeJson>>().await {
        Err(error) => return Err(ApiError::Deserialize(error)),
        Ok(val) => {
            let mut charges: Vec<Charge> = vec![];
            for item in val {
                match Charge::try_from(&item) {
                    Err(error) => {
                        return Err(ApiError::Conversion(format!(
                            "failed to convert '{:?}': {}",
                            &item, error
                        )))
                    }
                    Ok(charge) => charges.push(charge),
                }
            }
            return Ok(charges);
        }
    };
}

#[cfg(test)]
mod tests {

    use super::{get_charge_from_wegli_api, get_charges_from_wegli_api};

    #[tokio::test]
    async fn test_get_charge_from_wegli_api() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/charges/101000")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
                {
                    "tbnr": "101000",
                    "description": "Sie kamen von der Fahrbahn ab und verursachten Sachschaden.",
                    "fine": "35.0",
                    "bkat": "§ 1 Abs. 2, § 49 StVO; § 24 Abs. 1, 3 Nr. 5 StVG; -- BKat",
                    "penalty": null,
                    "fap": null,
                    "points": 0,
                    "valid_from": "2021-07-28T00:00:00.000+02:00",
                    "valid_to": null,
                    "implementation": null,
                    "classification": 4,
                    "variant_table_id": null,
                    "rule_id": 2,
                    "table_id": null,
                    "required_refinements": "00000000000000000000000000000000",
                    "number_required_refinements": 0,
                    "max_fine": "0.0",
                    "created_at": "2023-09-18T15:30:14.053+02:00",
                    "updated_at": "2023-09-18T15:30:14.053+02:00"
                }"#,
            )
            .create_async()
            .await;

        let response = get_charge_from_wegli_api(
            &server.url(),
            &"any_api_key".to_string(),
            &"101000".to_string(),
            &None,
        )
        .await
        .unwrap();
        assert_eq!(&response.fine, &35.0);
        mock.assert();
    }

    #[tokio::test]
    async fn test_get_charges_from_wegli_api() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/charges")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
                [
                    {
                        "tbnr": "101000",
                        "description": "Sie kamen von der Fahrbahn ab und verursachten Sachschaden.",
                        "fine": "35.0",
                        "bkat": "§ 1 Abs. 2, § 49 StVO; § 24 Abs. 1, 3 Nr. 5 StVG; -- BKat",
                        "penalty": null,
                        "fap": null,
                        "points": 0,
                        "valid_from": "2021-07-28T00:00:00.000+02:00",
                        "valid_to": null,
                        "implementation": null,
                        "classification": 4,
                        "variant_table_id": null,
                        "rule_id": 2,
                        "table_id": null,
                        "required_refinements": "00000000000000000000000000000000",
                        "number_required_refinements": 0,
                        "max_fine": "0.0",
                        "created_at": "2023-09-18T15:30:14.053+02:00",
                        "updated_at": "2023-09-18T15:30:14.053+02:00"
                    },
                    {
                        "tbnr": "101006",
                        "description": "Sie gerieten ins Schleudern und verursachten Sachschaden.",
                        "fine": "35.0",
                        "bkat": "§ 1 Abs. 2, § 49 StVO; § 24 Abs. 1, 3 Nr. 5 StVG; -- BKat",
                        "penalty": null,
                        "fap": null,
                        "points": 0,
                        "valid_from": "2021-07-28T00:00:00.000+02:00",
                        "valid_to": null,
                        "implementation": null,
                        "classification": 4,
                        "variant_table_id": null,
                        "rule_id": 2,
                        "table_id": null,
                        "required_refinements": "00000000000000000000000000000000",
                        "number_required_refinements": 0,
                        "max_fine": "0.0",
                        "created_at": "2023-09-18T15:30:14.065+02:00",
                        "updated_at": "2023-09-18T15:30:14.065+02:00"
                    }
                ]"#,
            )
            .create_async()
            .await;

        let response = get_charges_from_wegli_api(&server.url(), &"any_api_key".to_string(), &None)
            .await
            .unwrap();
        assert_eq!(&response[0].fine, &35.0);
        mock.assert();
    }
}
