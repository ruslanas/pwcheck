use std::io;
use std::fs::File;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::io::BufReader;
use std::io::prelude::*;

pub fn input(mut pwd: & mut String, msg: & str) {
    println!("{}", msg);
    io::stdin()
        .read_line(& mut pwd)
        .unwrap();
}

pub fn read_line(reader: &mut BufReader<File>, pos: u64) -> String {

    reader.seek(io::SeekFrom::Start(pos * 42))
        .expect("Seek fail");

    let mut line = String::new();
    reader.read_line(& mut line)
        .expect("Read line fail");
    
    return line;
}

pub fn sha1_hash(pwd: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(pwd.as_str());
    
    return hasher.result_str()
        .to_uppercase();
}
