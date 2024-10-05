use clap::{arg, command, Parser};
use std::{collections::HashMap, fs::metadata, hash::Hash, io};

/*
    This is a utlility file which contains various helper-functions used throughout this project.

    Everything here is documented with a breif description of what it does.
*/

// Clap's CLI argument parser

/// Compresses an input file into an output (with extension .pkzip)
#[derive(Debug, Parser)]
#[command(about)]
pub struct Args {
    /// File to compress
    #[arg(required = true)]
    pub file: String,
}

/// Enumerates duplicates within a `Vec<T>` to `Vec<(T, usize)`, count starts at `0`.
pub fn enumerate_duplicates<T>(v: Vec<T>) -> Vec<(T, usize)>
where
    T: Eq + Hash + Clone,
{
    let mut map: HashMap<T, usize> = HashMap::new();
    v.into_iter()
        .map(|f| {
            if map.contains_key(&f) {
                *map.get_mut(&f).unwrap() += 1;
            } else {
                map.insert(f.clone(), 0);
            }
            let count = *map.get(&f).unwrap();
            (f, count)
        })
        .collect()
}

/// Helper function to convert a `u32` in base-10 to a different base (usually base-36)
pub fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x /= radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

/// Retrieves the least used byte in a Vec<u8>. An unused byte will always be returned if present.
pub fn get_least_used_byte(input: &Vec<u8>) -> u8 {
    // First we run through with a bitmask
    let mut upper = u128::MIN;
    let mut lower = u128::MIN;

    input.iter().for_each(|&b| match b {
        0..=127 => lower |= (0x1 as u128) << b,
        128..=255 => upper |= (0x1 as u128) << (b - 128),
    });

    if lower != u128::MAX {
        for i in 0..=127 {
            if (!lower >> i) % 2 == 1 {
                return i as u8;
            }
        }
    }

    if upper != u128::MAX {
        for i in 0..=127 {
            if (!lower >> i) % 2 == 1 {
                return i as u8 + 128;
            }
        }
    }

    // If the bitmask failed (aka all 256 bytes were present, use the more expensive HashMap appraoch)
    let mut map: HashMap<u8, usize> = HashMap::new();

    input.iter().for_each(|&b| {
        map.entry(b).and_modify(|v| *v += 1).or_insert(1);
    });

    map.into_iter()
        .min_by_key(|(_byte, count)| *count)
        .unwrap_or((0, 0)) // This default will almost never get reached since it means the input is empty
        .0
}

pub fn insert_before_target(input: &mut Vec<u8>, target_byte: u8, insert_byte: u8) {
    let mut index = 0;
    while index < input.len() {
        if input[index] == target_byte {
            // Insert a backslash before the target byte
            input.insert(index, insert_byte);
            // Move past the inserted backslash and delim
            index += 2;
        } else {
            index += 1;
        }
    }
}

/// Gets the file size given a path. Unified functionality across different OS's.
pub fn get_file_size(path: &str) -> io::Result<u64> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        return Ok(metadata(path)?.file_size());
    }

    #[cfg(target_os = "linux")]
    {
        use std::os::linux::fs::MetadataExt;
        return Ok(metadata(path)?.st_size());
    }
}

/// Returns the index of the first instance of an object
pub fn index_of<T>(v: &Vec<T>, obj: &T) -> Option<usize>
where
    T: Eq,
{
    for index in 0..v.len() {
        if v[index] == *obj {
            return Some(index);
        }
    }
    None
}