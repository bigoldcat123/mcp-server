use std::{
    io::{BufRead, BufReader, Write, stdin},
    process::{self, Stdio},
    thread,
};

fn main() {
    let mut x = process::Command::new("/Users/dadigua/Desktop/mcp-server/target/debug/server")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // let e = x.wait().unwrap();
    // println!("{:?}", e);

    let out = x.stdout.take().unwrap();

    thread::spawn(|| {
        println!("{:?}", "waitin!");
        let reader = BufReader::new(out);
        reader.lines().for_each(|l| {
            println!("echo ! {:?}", l.unwrap());
        });
    });
    let mut in_ = x.stdin.take().unwrap();
    let stdin = stdin();
    stdin.lines().for_each(|l| {
        let l = l.unwrap();
        write!(in_, "{} \n", l).unwrap();
    });
    // 等待子进程退出
    let status = x.wait().expect("Failed to wait on child");
    println!("Child exited with: {}", status);
}
