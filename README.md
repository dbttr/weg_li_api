# weg_li_api

`weg_li_api` aims to make working with the [weg.li API](https://www.weg.li/api-docs/index.html) more convenient in Rust.

It currently focuses on the read APIs, implementing the defined GET methods.

## Basic usage

1. Get a personalized API token from [weg.li](https://www.weg.li/). You can find it on your [Profile page](https://www.weg.li/user) after creating an account.

2. Create a [WegLiApiClient](api/struct.WegLiApiClient.html).

   ```rust
   let client = weg_li_api::WegLiApiClient::new(
       &"https://www.weg.li/api".to_string(),
       &"your_personal_api_token".to_string(),
   );
   ```

3. Execute API calls

   ```rust
   let charges = client.get_charges().await?;
   let my_zip = "20095";
   let district = client.get_district(&my_zip.to_owned()).await?;
   ```

## Get notice export archive

Most functions interact with a single REST API endpoint. There also is a convenience function to download the latest notices export zip archive and unzip it if desired.

```rust
// if the unzip argument is false, returns path of the zip file
let notices_zip_path = client.download_latest_export(&"/tmp/weg_li".to_owned(), true, false).await?;
// if the unzip argument is true, returns path of the extracted .csv
let unzipped_csv_path = client.download_latest_export(&"/tmp/weg_li".to_owned(), true, true).await?;
```

You can then process the data as you wish. For parsing, `weg_li_api` provides structs of the export notice format you could e.g. use like this with the [csv crate](https://crates.io/crates/csv):

```rust
use weg_li_api::types::export::{ExportNotice, ExportNoticeCsv};

let mut reader = csv::ReaderBuilder::new()
    .has_headers(true)
    .from_path(&notice_csv_file_path)?;
for notice_line_result in reader.deserialize() {
    // data as in csv
    let notice_json: ExportNoticeCsv = notice_line_result?;
    // datetime fields converted to chrono DateTime<FixedOffset>
    let notice = ExportNotice::try_from(&notice_json)?;
}
```

# License

This project is licensed under the [MIT license](LICENSE.md).
