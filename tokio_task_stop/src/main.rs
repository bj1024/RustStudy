use env_logger::Env;

use log::{debug, error, info, trace, warn};
use rand::prelude::ThreadRng;
use tokio::join;
use tokio::sync::mpsc::{channel, Sender};
use tokio::task::JoinHandle;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
// use std::time::Instant;
use lazy_static::lazy_static;
use rand::Rng;
use std::thread;
use tokio::runtime::{Builder, Runtime};
use tokio::time::{sleep, Duration, Instant};

lazy_static! {
    static ref T_RUNTIME_CHILDS: Runtime = Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("my-child_tasks")
        .enable_all()
        .build()
        .unwrap();
}

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
    // let runtime = Builder::new_multi_thread()
    //     .worker_threads(1)
    //     .thread_name("my-main_tasks")
    //     .enable_all()
    //     .build()
    //     .unwrap();

    // let child_runtime = Builder::new_multi_thread()
    //     .worker_threads(2)
    //     .thread_name("my-child_tasks")
    //     .enable_all()
    //     .build()
    //     .unwrap();

    let server_proc_handle = T_RUNTIME_CHILDS.spawn(async move {
        server_proc().await;
    });

    // drop(send);
    // let _ = recv.recv().await; // sendが全部消えるまで待つ。recvがエラーになり、検知する。

    // sleep(Duration::from_millis(3000)).await;
    server_proc_handle.await.expect("server_task await failed.");

    // runtime.shutdown_background();
    // let _ = tokio::join!(server_proc_handle);

    info!("fin. [{}]", prog());
}

async fn server_proc() {
    let mut count = 0;

    let (send, mut recv) = channel(1);
    // let mut childtasks: Vec<JoinHandle<()> = vec![];
    for i in 0..10 {
        count += 1;
        debug!("[{}] spawn child[{}].", threadsig(), i);
        let send_c = send.clone();
        // tokio::spawn(async move {
        // let i_c = ;

        T_RUNTIME_CHILDS.spawn(async move {
            info!("[{}] child[{}] start.", threadsig(), i);
            child_proc(count, send_c).await;
            info!("[{}] child[{}] end.", threadsig(), i);
        });
    }

    // sleep(Duration::from_millis(2000)).await;
    // runtime.shutdown_background();
    drop(send);
    info!("[{}] waiting childs.", threadsig());

    let _ = recv.recv().await; // sendが全部消えるまで待つ。recvがエラーになり、検知する。
    debug!("[{}] exit server_proc.", threadsig());
}

async fn child_proc(id: i32, _sender: Sender<()>) {
    let start = Instant::now();
    // let mut rng = rand::thread_rng();
    let sleeptime: u64 = ThreadRng::default().gen_range(1000..2000);
    // let sleeptime: u64 = 1000;
    sleep(Duration::from_millis(sleeptime)).await;
    info!(
        "[{}] child_proc[{}] end. {}ms",
        threadsig(),
        id,
        start.elapsed().as_millis()
    );
}
