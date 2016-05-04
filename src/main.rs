extern crate rustc_serialize;

use std::{fs, env, process, str};
use std::string::String;
use std::io::{BufReader, Write, Read, Seek, SeekFrom};
use rustc_serialize::hex::{FromHex, FromHexError};

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("usage: bgrep <pattern> <file>");
        process::exit(1);
    }

    let pattern = match args[1].from_hex() {
        Ok(bytes) => bytes,
        _ => {
            println_stderr!("Invalid hex sequence specified");
            process::exit(1);
        }
    };

    let file_path = &args[2];

    let file = fs::File::open(&file_path);
    if file.is_err() {
        println_stderr!("Could not open {}: {}", file_path, file.err().unwrap());
        process::exit(1);
    }

    let mut reader = BufReader::new(file.unwrap());

    if find_pattern(&mut reader, &pattern) {
        println!("Found a match in {}", file_path);
    }
}

fn find_pattern<R: Seek + Read>(source: &mut R, pattern: &Vec<u8>) -> bool {
    let mut offset = 0;
    let end_offset = source.seek(SeekFrom::End(0)).unwrap();

    let mut buffer = vec![0; pattern.len()];

    source.seek(SeekFrom::Start(0)).ok();
    while offset < end_offset {
        offset = source.seek(SeekFrom::Current(0)).unwrap();

        source.read(&mut buffer).ok();

        println!("buffer: {:X}, pattern: {:}", buffer.as_slice()[0], str::from_utf8(pattern).unwrap());

        if buffer.as_slice() == pattern.as_slice() {
            return true;
        }
    }

    return false;
}
