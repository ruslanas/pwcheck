/*
Copyright © 2017 Ruslanas Balčiūnas
Email: ruslanas.com@gmail.com

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate crypto;

use std::io;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::cmp::Ordering;

fn main() {

    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: pwcheck <FILE> [PASSWORD]");
        return;
    }

    // consumes preceding elements
    let fname = args.nth(1)
        .unwrap();

    let metadata = fs::metadata(& fname);
    let metadata = match metadata {
        Ok(data) => data,
        Err(e) => {
            println!("Metadata error: {:?}", e.kind());
            return;
        }
    };

    if metadata.is_dir() {
        println!("File expected. Directory found.");
        return;
    }

    let mut pwd: String = match args.nth(0) {
        Some(v) => v.trim_right_matches("\r\n").to_string(),
        None => "".to_string()
    };

    if pwd.is_empty() {
        input(& mut pwd, "Enter plain password:");
        pwd = pwd.trim_right_matches("\r\n").to_string();
    }

    let lines = (metadata.len() as f64 / 42 as f64).ceil() as u64;
    println!("{} password hashes in file.", lines);

    let hash = sha1_hash(pwd);
    println!("SHA1 {}", hash);

    let file = File::open(& fname);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file [{:?}]", e.kind());
            return;
        }
    };
    
    let mut reader = BufReader::new(file);

    let mut start = 0;
    let mut end = lines - 1;
    let mut new_pos = end;
    let mut old_pos = start;
    let mut found = false;

    while !found && (new_pos != old_pos) {
        
        old_pos = new_pos;
        new_pos = (end + start) / 2;

        reader.seek(io::SeekFrom::Start(new_pos * 42))
            .expect("Seek fail");

        let mut line = String::new();
        reader.read_line(&mut line)
            .expect("Read line fail");

        let cmp = hash.cmp(& line.trim().to_string());

        match cmp {
            Ordering::Greater => start = new_pos,
            Ordering::Less => end = new_pos,
            Ordering::Equal => found = true
        }

    }

    if found {
        println!("Found at line: {}", new_pos);
    } else {
        println!("Not found!");
    }
}

fn input(mut pwd: & mut String, msg: & str) {
    println!("{}", msg);
    io::stdin()
        .read_line(& mut pwd)
        .unwrap();
}

fn sha1_hash(pwd: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(pwd.as_str());
    
    return hasher.result_str()
        .to_uppercase();
}