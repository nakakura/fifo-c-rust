extern crate nix;
extern crate tempdir;

use std::io::prelude::*;
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use tokio::fs::OpenOptions;
use tokio::prelude::*;
use nix::sys::stat;
use nix::sys::wait::waitpid;
use nix::unistd;
use nix::unistd::{fork, getpid, getppid, ForkResult};
use tempdir::TempDir;
use std::intrinsics::transmute;

#[tokio::main]
async fn main() {
    println!("Main({}) stared", getpid());

    /*
    let tmp_dir = TempDir::new("test_fifo").unwrap();
    let fifo_path = tmp_dir.path().join("foo.pipe");

    match unistd::mkfifo(&fifo_path, stat::Mode::S_IRWXU) {
        Ok(_) => println!("created {:?}", fifo_path),
        Err(err) => println!("Error creating fifo: {}", err),
    }
     */

    let mut file = OpenOptions::new().read(true).open("/tmp/myfifo2").await.unwrap();
    let original = SystemTime::now();
    let mut log_file = tokio::fs::File::create("/home/nakakura/log.txt").await.unwrap();

    let mut len: u64 = 0;
    loop {
        let len = file.read_u64_le().await.unwrap() as usize;
        println!("read len {}", len);
        if len <= 0 {
            continue;
            tokio::time::sleep(Duration::from_millis(1));
        }
        let mut buf = vec![0u8; len];
        let read_len = file.read(&mut buf).await.unwrap();
        println!("read len {}", read_len);
        /*
        for i in 0..len {
            buf[i] = file.read_i32_le().await.unwrap();
        }
         */
        let now = SystemTime::now();
        let duration_from_original = now.duration_since(original).unwrap();
        let data = (duration_from_original.as_micros() as f64) / 1000000 as f64;
        let message = format!("{}\n", data);

        log_file.write_all(message.as_bytes()).await;
        tokio::time::sleep(Duration::from_millis(5));
    }
}