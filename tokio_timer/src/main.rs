use env_logger::Env;

#[warn(unused_imports)]
use log::{debug, error, info, trace, warn};

use std::env;
use std::env::args;
use std::ffi::OsStr;
use std::path::Path;
use std::str::FromStr;
// use std::time::Instant;
use tokio::time::{interval, sleep, sleep_until, Duration, Instant};

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

    info!("tokio::time::sleep start.");
    let start = Instant::now();
    sleep(Duration::from_millis(1000)).await;
    info!("tokio::time::sleep end. {}ms", start.elapsed().as_millis());

    info!("tokio::time::sleep_until start.");
    let start = Instant::now();
    sleep_until(Instant::now() + Duration::from_millis(1000)).await;
    info!(
        "tokio::time::sleep_until end. {}ms",
        start.elapsed().as_millis()
    );

    // interval は最後のinterval発生から指定時間が過ぎれば発生する。
    // 初回はすぐ発生し、
    info!("tokio::time::interval start.");
    let start = Instant::now();
    let mut interval = interval(Duration::from_millis(1000));

    interval.tick().await; // 初回はすぐ発生。
    info!(
        "tokio::time::interval 1 end. {}ms",
        start.elapsed().as_millis()
    );

    interval.tick().await; // 2回目は１回目interval終了＋1000msで発生。
    info!(
        "tokio::time::interval 2 end. {}ms",
        start.elapsed().as_millis()
    );

    sleep(Duration::from_millis(500)).await; // 途中500msあけてみる。

    interval.tick().await; // 前回のinterval＋1000msで発生するはず
    info!(
        "tokio::time::interval 3 end. {}ms",
        start.elapsed().as_millis()
    );

    sleep(Duration::from_millis(1500)).await; // 途中1500msあけてみる。

    interval.tick().await; // 途中1500ms経過(>1000)なので、すぐに発生する。
    info!(
        "tokio::time::interval 4 end. {}ms",
        start.elapsed().as_millis()
    );

    info!("fin. [{}]", prog());
}
