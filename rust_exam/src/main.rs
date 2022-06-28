use env_logger::Env;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use rust_exam::Point3D;
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
    box_clonetest();
}

fn type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}

fn box_clonetest() {
    let mut box1: Box<Point3D> = Box::new(Point3D::new(1, 2, 3));
    let box2 = box1.clone();
    // let box2: Box<i32> = Box::new(*box1 + 1.into());
    // box1.x += 1;
    // box1.y += 1;
    // box1.z += 1;

    *box1 += Point3D::new(10, 20, 30);

    // Primitive Type pointer
    // pointer - Rust https://doc.rust-lang.org/std/primitive.pointer.html#common-ways-to-create-raw-pointers

    // let handle_ptr: *const Handle = &*box1;
    debug!("box1={}", box1);
    debug!("box2={}", box2);

    // let ptr1 = Box::into_raw(box1);
    // let ptr2 = Box::into_raw(box2);
    // debug!("box1={} {:p}", *box1, *box1);
    // debug!("box2={} {:p}", *box2, ptr2);

    // box1 = Box::new(*box1 + 1);
    let box1ptr = Box::into_raw(box1);
    *box1 += Point3D::new(10, 20, 30);

    let my_speed: Box<i32> = Box::new(88);
    let my_speedptr: *mut i32 = Box::into_raw(my_speed);
    unsafe {
        debug!("my_speedptr= {:p}", my_speedptr);
        debug!("my_speedptr= {}", *my_speedptr);
    }
}
