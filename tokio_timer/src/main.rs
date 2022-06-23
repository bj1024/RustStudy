use env_logger::Env;

#[warn(unused_imports)]
use log::{debug, error, info, trace, warn};

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::str::FromStr;
use std::{env::args, time::Duration};
use tokio::time::delay_for;

fn prog() -> String {
    env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .unwrap_or(String::from_str("-").unwrap())
}

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::Builder::from_env(env)
        .format_module_path(false)
        .format_level(true)
        .format_timestamp_millis()
        .format_target(false)
        .format_indent(Some(20))
        .init();

    info!("start. [{}]", prog());

    info!("delay_for start.");
    delay_for(Duration::from_millis(100)).await;
    info!("delay_for end.");

    info!("fin. [{}]", prog());
}
