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
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // args
    // TODO: check arg number and act accordingly
    let args: Vec<String> = env::args().collect();
    let names_file_name = &args[1];
    let surnames_file_name = &args[2];

    // open file to write
    let newfile = Path::new("wordlist.txt");
    let display = newfile.display();

    let mut outfile = match File::create(&newfile) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(outfile) => outfile,
    };

    // read surnames file
    if let Ok(surnames) = read_lines(surnames_file_name) {
        for line in surnames {
            if let Ok(surname) = line {
                if let Ok(names) = read_lines(names_file_name) {
                    for nameline in names {
                        if let Ok(name) = nameline {
                            match outfile.write_all(surname.as_bytes()) {
                                Err(why) => panic!("couldn't write to {}: {}", display, why),
                                Ok(_) => (),
                            }
                            match outfile.write_all(b" ") {
                                Err(why) => panic!("couldn't write to {}: {}", display, why),
                                Ok(_) => (),
                            }
                            match outfile.write_all(name.as_bytes()) {
                                Err(why) => panic!("couldn't write to {}: {}", display, why),
                                Ok(_) => (),
                            }
                            match outfile.write_all(b"\n") {
                                Err(why) => panic!("couldn't write to {}: {}", display, why),
                                Ok(_) => (),
                            }
                        }
                    }
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
