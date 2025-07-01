use std::{
    fs::File,
    io::{Write, stdin},
};

fn main() {
    let mut f = File::create("/Users/dadigua/Desktop/mcp-server/log.log").unwrap();
    let stdin = stdin();
    stdin.lines().for_each(|l| {
        if let Ok(l) = l {
            println!("from server {l}");
            write!(f, "{}\n", l).unwrap();
        } else {
            eprintln!("error!")
        }
    });
}
