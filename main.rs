use regex::Regex;
use regex::Captures;

fn main() {
    let re = Regex::new(r"^<([0-9]{1,3})>(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\+\d{2}:\d{2}) ([\S]+) ([\S]+): ([\S\s]+)").unwrap();
    let text = r"<7>2022-09-27T11:47:56+08:00 LOCAL-192-168-97-55.boyaa.com fishServer-th-dev-[19023]: 2022-09-27T11:47:56.893+0800      DEBUG   comm/redis_cache.go:54 comm.(*RedisCache).Load  redis get data got string redis Redis<192.168.97.200:19013 db:0> key fish_jackpot|user|263201";

    for cap in re.captures_iter(text) {
        println!("-->>1 {:?} -->>2 {:?} -->>3 {:?} -->>4 {:?} -->>5 {:?}", &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);

        println!("one...");

        let ree = Regex::new(r"\[.*\]").unwrap();
        let fname = ree.replace(&cap[4], |caps: &Captures| {
          println!("caps {:?}", caps);
          format!("{}.log", &caps[0])
        });
        println!("file name {}", fname);
    }
}



use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
use std::{fs, io::Write};

// Create a new log writer. The first argument is anything resembling a path. The
// basename is used for naming the log files.
//
// Here we choose to limit logs by 10 lines, and have at most 2 rotated log files. This
// makes the total amount of log files 3, since the original file is present as well.

fn main() {
    let mut log = FileRotate::new(
        "logs/my-log",
        AppendCount::new(2),
        ContentLimit::Lines(3),
        Compression::None,
        #[cfg(unix)]
        None,
    );

    // Write a bunch of lines
    writeln!(log, "Line 1: Hello World!");
    for idx in 2..11 {
        writeln!(log, "Line {}", idx);
    }
}


use std::fs::File;
use std::io;
use std::io::Write;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:6308").await?;
    let path = "log.txt";
    let mut buf = [0; 4096];
    let mut output = File::create(path)?;

    loop {
        let (len, _addr) = sock.recv_from(&mut buf).await?;
        write!(output, "{}", String::from_utf8_lossy(&buf[..len]),).unwrap();
    }
}
