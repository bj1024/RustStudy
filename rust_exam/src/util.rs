use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::str::FromStr;
use std::thread;

#[allow(dead_code)]
pub fn type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}

#[allow(dead_code)]
pub fn prog() -> String {
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
pub fn threadsig() -> String {
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
