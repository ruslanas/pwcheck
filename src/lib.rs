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

use std::path::Path;
use std::io;
use std::fs;
use std::fs::File;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::Ordering;

pub fn find_in_path(path: &Path, hash: &String) {
    if path.is_dir() {
        each(path, |p| {
            find_in_path(p, hash);
        });
        return;
    }

    let lines = match line_count(path) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    if lines < 1 {
        return;
    }
    println!("{} rows in file `{:?}`.", lines, path);

    let mut file_reader = get_file_reader(&path).expect("Fail");
    let result = get_idx(&hash, &mut file_reader, 0, lines - 1);

    let idx = match result {
        Ok(v) => v,
        Err(_e) => {
            println!("{}", _e);
            return;
        }
    };

    println!("Found at line #{}", idx);
}

pub fn each<F>(path: &Path, cb: F)
where
    F: Fn(&Path),
{
    if path.is_file() {
        return cb(path);
    }
    let _: Vec<_> = path.read_dir()
        .expect("Nasty!")
        .map(|x| cb(x.expect("Sh..!").path().as_path()))
        .collect();
}

pub fn get_idx(
    hash: &String,
    reader: &mut BufReader<File>,
    from: u64,
    to: u64,
) -> Result<u64, String> {
    let mut from = from;
    let mut to = to;
    let mut found = false;
    let mut pos = from;
    let mut old_pos = to + 1;

    while !found && (pos != old_pos) {
        old_pos = pos;
        pos = (from + to) / 2;

        let line = read_line(reader, pos)?;

        match hash.cmp(&line) {
            Ordering::Greater => from = pos,
            Ordering::Less => to = pos,
            Ordering::Equal => found = true,
        }
    }

    if !found {
        return Err("Not found".to_string());
    }

    Ok(pos)
}

pub fn get_file_reader(path: &Path) -> Result<BufReader<File>, io::Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn input(mut pwd: &mut String, msg: &str) {
    println!("{}", msg);
    io::stdin().read_line(&mut pwd).unwrap();
}

pub fn line_count(path: &Path) -> Result<u64, io::Error> {
    let meta = fs::metadata(path)?;
    Ok((meta.len() as f64 / 42 as f64).ceil() as u64)
}

pub fn read_line(reader: &mut BufReader<File>, pos: u64) -> Result<String, String> {
    reader
        .seek(io::SeekFrom::Start(pos * 42))
        .expect("Seek fail");

    let mut buff = [0; 42];
    reader.read(&mut buff).unwrap();

    to_string_validate(buff)
}

fn to_string_validate(buff: [u8; 42]) -> Result<String, String> {
    let mut s = String::new();
    for i in 0..40 {
        let c = buff[i];
        if !((c > 47 && c < 58) || (c > 64 && c < 71)) {
            return Err(format!(
                "Invalid ASCII #{}",
                (c as u16).to_string().as_str()
            ));
        }
        s.push(c as char);
    }

    Ok(s)
}

pub fn trim(input: &String) -> String {
    input.trim_right_matches("\r\n").to_string()
}

pub fn sha1_hash(pwd: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(pwd.as_str());

    hasher.result_str().to_uppercase()
}
