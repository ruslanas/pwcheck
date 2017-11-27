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
extern crate time;

mod util;

use std::fs;
use std::fs::File;
use std::io::BufReader;
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
        Some(v) => util::trim(v),
        None => String::new()
    };

    if pwd.is_empty() {
        util::input(& mut pwd, "Enter plain password:");
        pwd = util::trim(pwd);
    }

    let lines = (metadata.len() as f64 / 42 as f64).ceil() as u64;
    println!("{} password hashes in file.", lines);

    let hash = util::sha1_hash(pwd);
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
    let mut pos = end;
    let mut old_pos = start;
    let mut found = false;

    let t = time::precise_time_s();
    
    while !found && (pos != old_pos) {
        
        old_pos = pos;
        pos = (end + start) / 2;

        let line = match util::read_line(& mut reader, pos) {
            Ok(v) => v,
            Err(e) => {
                println!("Corrupted row: #{}", pos);
                println!("Error: {}", e);
                return;
            }
        };

        match hash.cmp(& line) {
            Ordering::Greater => start = pos,
            Ordering::Less => end = pos,
            Ordering::Equal => found = true
        }

    }

    let diff = time::precise_time_s() - t;

    if found {
        println!("Found at line: {} in {} seconds.", pos, diff);
    } else {
        println!("Not found in {} seconds.", diff);
    }
}
