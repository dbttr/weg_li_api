use std::{fs, path::PathBuf};

use anyhow::anyhow;
use url::Url;

use crate::types::{
    export::{Export, ExportJson},
    request::RetrySettings,
};

use super::{
    error::ApiError,
    request::{execute_request, RetryData, DEFAULT_RETRY_SETTINGS},
    util::{download_to_dir, unzip_archive},
};

pub async fn get_exports_from_wegli_api(
    api_url: &Url,
    api_token: &String,
    public: bool,
    retry_settings: &Option<RetrySettings>,
) -> Result<Vec<Export>, ApiError> {
    let retry_data = RetryData {
        retry_count: 0,
        settings: match retry_settings {
            Some(settings) => settings.clone(),
            None => DEFAULT_RETRY_SETTINGS,
        },
    };
    let request_builder = reqwest::Client::new()
        .get(format!(
            "{}{}{}",
            api_url,
            "exports",
            if public { "/public" } else { "" }
        ))
        .header("X-API-KEY", api_token);

    let response = match execute_request(&request_builder, &Some(retry_data)).await {
        Err(error) => return Err(error),
        Ok(response) => response,
    };

    match response.json::<Vec<ExportJson>>().await {
        Err(error) => return Err(ApiError::Deserialize(error)),
        Ok(val) => {
            let mut exports: Vec<Export> = vec![];
            for item in val {
                match Export::try_from(&item) {
                    Err(error) => {
                        return Err(ApiError::Conversion(format!(
                            "failed to convert '{:?}': {}",
                            &item, error
                        )))
                    }
                    Ok(export) => exports.push(export),
                }
            }
            return Ok(exports);
        }
    };
}

pub async fn download_latest_export_from_wegli(
    api_url: &Url,
    api_token: &String,
    path: &Path,
    public: bool,
    unzip: bool,
    retry_settings: &Option<RetrySettings>,
) -> Result<PathBuf, anyhow::Error> {
    let last_export =
        match get_exports_from_wegli_api(api_url, api_token, public, retry_settings).await {
            Err(error) => return Err(anyhow!(error)),
            Ok(mut exports) => {
                exports.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                dbg!(&exports);
                match exports.first().cloned() {
                    None => return Err(anyhow!("no export found")),
                    Some(export) => export,
                }
            }
        };

    let download_path = match download_to_dir(&path, &last_export.download.url).await {
        Err(error) => return Err(anyhow!(error)),
        Ok(val) => val,
    };

    if unzip {
        let csv_path = match unzip_archive(&download_path, &path) {
            Err(error) => return Err(anyhow!(error)),
            Ok(_) => {
                let paths = match fs::read_dir(&path) {
                    Err(error) => return Err(anyhow!(error)),
                    Ok(paths) => paths,
                };
                let mut found_csv: Option<PathBuf> = None;
                for dir_entry in paths {
                    match dir_entry {
                        Err(error) => return Err(anyhow!(error)),
                        Ok(dir_entry) => {
                            let file_name = match dir_entry.file_name().into_string() {
                                Err(os_string) => {
                                    return Err(anyhow!(
                                        "could not convert to string: {:?}",
                                        os_string
                                    ))
                                }
                                Ok(val) => val,
                            };
                            if file_name.to_lowercase().ends_with(".csv") {
                                found_csv = Some(dir_entry.path())
                            }
                        }
                    }
                }
                match found_csv {
                    Some(val) => val,
                    None => return Err(anyhow!("could not find csv in: {:?}", &path)),
                }
            }
        };
        return Ok(csv_path);
    }

    Ok(download_path)
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use url::Url;

    use super::get_exports_from_wegli_api;

    #[tokio::test]
    async fn test_get_exports_from_wegli_api() {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/exports/public")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
                [
                    {
                        "export_type": "notices",
                        "file_extension": "csv",
                        "created_at": "2022-11-14T03:01:58.056+01:00",
                        "download": {
                            "filename": "notices-46.zip",
                            "url": "https://www.weg.li/rails/active_storage/blobs/redirect/.../notices-46.zip"
                        }
                    },
                    {
                        "export_type": "notices",
                        "file_extension": "csv",
                        "created_at": "2022-11-21T03:02:19.396+01:00",
                        "download": {
                            "filename": "notices-47.zip",
                            "url": "https://www.weg.li/rails/active_storage/blobs/redirect/.../notices-47.zip"
                        }
                    }
                ]
                "#,
            )
            .create_async()
            .await;

        let response = get_exports_from_wegli_api(
            &Url::from_str(&server.url()).unwrap(),
            &"any_api_key".to_string(),
            true,
            &None,
        )
                .await
                .unwrap();
        assert_eq!(
            &response[1].download.filename,
            &"notices-47.zip".to_string()
        );
        mock.assert();
    }
}
