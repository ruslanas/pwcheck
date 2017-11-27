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