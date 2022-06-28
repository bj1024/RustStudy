use env_logger::Env;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::str::FromStr;
use std::thread;

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

#[allow(dead_code)]
fn threadsig() -> String {
    // let mut name = thread::current()
    //     .name()
    //     .unwrap_or("unknown thread")
    //     .to_string();
    // let threadid = format!("{:?}", thread::current().id());

    format!(
        "{},{:?}",
        thread::current().name().unwrap_or("unknown thread"),
        thread::current().id()
    )
    .to_string()

    // name.push_str(threadid.as_str());
    // name
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}
impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D { x, y, z }
    }
}

fn main() {
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

    // let boxi = Box::new(1);
    // box_test();
}

fn type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}
