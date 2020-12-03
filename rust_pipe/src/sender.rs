extern crate nix;
extern crate tempdir;

use std::io::prelude::*;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

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

    let tmp_dir = TempDir::new("test_fifo").unwrap();
    let fifo_path = tmp_dir.path().join("foo.pipe");

    match unistd::mkfifo(&fifo_path, stat::Mode::S_IRWXU) {
        Ok(_) => println!("created {:?}", fifo_path),
        Err(err) => println!("Error creating fifo: {}", err),
    }

    let mut file = OpenOptions::new().write(true).open("/tmp/myfifo").await.unwrap();
    loop {
        let mut vec = vec![0u8; 350];
        let len = vec.len();
        let mut bytes = unsafe {
            transmute::<usize, [u8;8]>(len)
        }.to_vec();
        bytes.append(&mut vec);

        file.write_all(&bytes).await;
        //tokio::time::sleep(Duration::from_millis(2)).await;
    }
}