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
use std::process;
use std::cmp::Ordering;

fn main() {

    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: pwcheck <FILE> [PASSWORD]");
        process::exit(0);
    }

    // consumes preceding elements
    let fname = args.nth(1)
        .expect("Fail reading filename");

    let mut pwd = String::new();

    match args.nth(0) {
        Some(v) => pwd = v,
        None => {
            println!("Enter plain password:");
            io::stdin()
                .read_line(&mut pwd)
                .expect("Read fail");
        }
    }

    let mut hasher = Sha1::new();
    hasher.input_str(pwd.trim());
    
    let hash = hasher.result_str()
        .to_uppercase();

    let metadata = fs::metadata(fname.clone())
        .expect("Failed to read file metadata");

    let chars = metadata.len();
    let lines = (chars as f64 / 42 as f64).ceil() as u64;
    
    println!("{} password hashes in file.", lines);
    println!("SHA1 {}", hash);

    let file = File::open(fname)
        .expect("Failed to open file for reading");
    
    let mut reader = BufReader::new(file);

    let mut _start = 0;
    let mut _end = lines - 1;
    let mut new_pos = _end;
    let mut old_pos = _start;
    let mut found = false;

    while !found && (new_pos != old_pos) {
        
        old_pos = new_pos;
        new_pos = (_end + _start) / 2;

        reader.seek(io::SeekFrom::Start(new_pos * 42))
            .expect("Seek fail");

        let mut line = String::new();
        reader.read_line(&mut line)
            .expect("Failed to read line");

        line = line.trim().to_string();
        let cmp = hash.cmp(&line);

        match cmp {
            Ordering::Greater => _start = new_pos,
            Ordering::Less => _end = new_pos,
            Ordering::Equal => {
                found = true;
            }
        }

        // println!("{}\t{}\t{:?}", new_pos, line.trim(), cmp);

    }

    if found {
        println!("Found at line: {}", new_pos);
    } else {
        println!("Not found!");
    }
}
