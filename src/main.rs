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
use std::env;

fn main() {

    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: pwcheck <FILE> [PASSWORD]");
        return;
    }

    // consumes preceding elements
    let fname = args.nth(1)
        .unwrap();

    let metadata = match fs::metadata(& fname) {
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

    let hash = util::sha1_hash(pwd);
    println!("SHA1 {}", hash);

    let lines = (metadata.len() as f64 / 42 as f64).ceil() as u64;
    println!("{} password hashes in file.", lines);

    let t = time::precise_time_s();
    
    let start = 0;
    let end = lines - 1;

    let mut file_reader = util::get_file_reader(fname)
        .unwrap();
    let result = util::get_idx(hash, & mut file_reader, start, end);
    let diff = time::precise_time_s() - t;

    let idx = match result {
        Ok(v) => v,
        Err(e) => {
            println!("{} in {} seconds.", e, diff);
            return;
        }
    };

    println!("Found at line {} in {} seconds.", idx, diff);

}
