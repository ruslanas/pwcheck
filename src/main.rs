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

use std::env;
use std::path::Path;

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: pwcheck <FILE|DIR> [PASSWORD]");
        return;
    }

    // consumes preceding elements
    let fname = args.nth(1).unwrap();

    let path = Path::new(&fname);

    if !path.exists() {
        println!("File or directory not found.");
        return;
    }

    let mut pwd = args.nth(0).unwrap_or(String::new());

    if util::trim(&pwd).is_empty() {
        util::input(&mut pwd, "Enter plain password:");
        pwd = util::trim(&pwd);
    }

    let hash = util::sha1_hash(pwd);
    println!("SHA1 {}", hash);

    let t = time::precise_time_s();
    util::find_in_path(path, &hash);
    let diff = time::precise_time_s() - t;

    println!("Done in {} seconds.", diff);
}
