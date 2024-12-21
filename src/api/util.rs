use futures_util::StreamExt;
use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use url::Url;

use super::error::{DownloadError, UnzipError};

pub async fn download_to_dir(path: &Path, url: &String) -> Result<PathBuf, DownloadError> {
    let url = match Url::parse(&url) {
        Err(error) => return Err(DownloadError::UrlParse(error)),
        Ok(val) => val,
    };

    let fpath = std::path::Path::new(path).join(
        match url.path_segments().and_then(|segments| segments.last()) {
            None => "file.zip",
            Some(val) => val,
        },
    );
    let mut tmp_file = match File::create(&fpath) {
        Err(error) => return Err(DownloadError::Io(error)),
        Ok(val) => tokio::fs::File::from(val),
    };

    let response = match reqwest::get(url).await {
        Err(error) => return Err(DownloadError::Reqwest(error)),
        Ok(val) => val,
    };

    let mut byte_stream = response.bytes_stream();

    while let Some(item) = byte_stream.next().await {
        let reader = match item {
            Ok(val) => val,
            Err(error) => return Err(DownloadError::Reqwest(error)),
        };

        match tokio::io::copy(&mut reader.as_ref(), &mut tmp_file).await {
            Err(error) => return Err(DownloadError::Io(error)),
            Ok(_) => (),
        };
    }

    return Ok(fpath);
}

pub fn unzip_archive(zip_path: &Path, unzip_dir_path: &Path) -> Result<(), UnzipError> {
    let zipfile = match File::open(zip_path) {
        Err(error) => return Err(UnzipError::Io(error)),
        Ok(file) => file,
    };
    let mut archive = match zip::ZipArchive::new(zipfile) {
        Err(error) => return Err(UnzipError::Zip(error)),
        Ok(val) => val,
    };
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Err(error) => return Err(UnzipError::Zip(error)),
            Ok(val) => val,
        };
        let outpath = match file.enclosed_name() {
            Some(path) => std::path::Path::new(unzip_dir_path).join(path),
            None => continue,
        };
        if file.is_dir() {
            match fs::create_dir_all(&outpath) {
                Err(error) => return Err(UnzipError::Io(error)),
                Ok(_) => {}
            };
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    match fs::create_dir_all(p) {
                        Err(error) => return Err(UnzipError::Io(error)),
                        Ok(_) => {}
                    };
                }
            }
            let mut outfile = match fs::File::create(&outpath) {
                Err(error) => return Err(UnzipError::Io(error)),
                Ok(val) => val,
            };
            match io::copy(&mut file, &mut outfile) {
                Err(error) => return Err(UnzipError::Io(error)),
                Ok(_) => {}
            };
        }
    }

    Ok(())
}
