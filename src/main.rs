//    Copyright (C) 2023 @servo
//    https://github.com/servomekanism/wordlister
//
//    wordlister - create a combined "name surname" list that can be
//    used to bruteforce usernames. Created to practice rust programming.
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
//    For more see the file 'LICENSE' for copying permission.

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process;

fn main() -> std::io::Result<()> {
    // todo: add arg checks information
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: {} <names.txt> <surnames.txt> <output.txt>", args[0]);
        process::exit(1);
    }

    let names = File::open(&args[1])?;
    let surnames = File::open(&args[2])?;
    let output_file = File::create(&args[3])?;

    let names_reader = BufReader::new(names);
    let mut surnames_reader = BufReader::new(surnames);
    let mut writer = BufWriter::new(output_file);

    for line1 in names_reader.lines() {
        let line1 = line1?; // makes line1 immutable again
        for line2 in surnames_reader.by_ref().lines() {
            let line2 = line2?; // makes line2 immutable again
            writer.write_all(format!("{} {}\n", line1, line2).as_bytes())?;
        }
        surnames_reader.seek(std::io::SeekFrom::Start(0))?;
    }
    Ok(())
}
