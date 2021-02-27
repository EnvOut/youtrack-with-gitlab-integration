#![feature(box_syntax)]
#[warn(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;

use log::LevelFilter;
use env_logger::WriteStyle;
use youtrack_tools::rest_api::client::YoutrackClient;
use youtrack_tools::rest_api::base::ops::BaseOps;
use youtrack_tools::rest_api::json_models::issue::field::IssueStateType;
use indoc::indoc;
use crate::api::{index, server};
use config::{Source, Value};

mod api;
mod settings;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter(None, LevelFilter::Info)
        .write_style(WriteStyle::Always)
        .init();

    api::server().await.await
}