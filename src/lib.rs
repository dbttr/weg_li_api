#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod api;
pub mod types;

pub use api::WegLiApiClient;
