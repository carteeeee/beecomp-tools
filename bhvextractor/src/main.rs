// written by carter. 20250106
// "i don't have a quote to put here!"
use base64::prelude::*;
use inflate::inflate_bytes;
use serde_json::Value;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::{env, fs};

const SSEGMENTTABLE: usize = 0x33b400;
const NUM_BANKS: usize = 32;

fn skip_last<T>(mut iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let last = iter.next();
    iter.scan(last, |state, item| std::mem::replace(state, Some(item)))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = fs::read_dir(args.get(1).expect("you didn't specify a directory"))
        .expect("couldn't read dir");

    let memraw = args.get(2).expect("you didn't specify ramdump");
    let memop = if memraw == "seg" {
        None
    } else {
        Some(fs::read(memraw).expect("couldn't read ram"))
    };

    let symraw = args.get(3).expect("you didn't specify sym");
    let mut tempstr = String::new();
    let symop = if symraw == "addr" {
        None
    } else {
        // i'm sorry.
        tempstr.clone_from(&fs::read_to_string(symraw).expect("couldn't read sym.txt"));
        Some(
            skip_last(tempstr.split('\n'))
                .filter_map(|v| {
                    let mut splitma = v.split_whitespace();
                    let addr = splitma.next().unwrap();
                    let name = splitma.next().unwrap();
                    if name.starts_with(".") {
                        return None;
                    }
                    Some((u32::from_str_radix(addr, 16).unwrap(), name))
                })
                .collect::<Vec<_>>(),
        )
    };

    let mut banks: Vec<u32> = Vec::with_capacity(NUM_BANKS);
    banks.resize(NUM_BANKS, 0);
    if let Some(mem) = &memop {
        for i in 0..NUM_BANKS {
            let offset = SSEGMENTTABLE + (i * 4);
            banks[i] = u32::from_be_bytes(mem[offset..(offset + 4)].try_into().unwrap());
        }
    }

    let mut used: HashSet<u32> = HashSet::new();
    for entry in dir {
        let path = entry.unwrap().path();

        if !path.is_file() {
            continue;
        };

        if path.extension() != Some(OsStr::new("lvl64")) {
            continue;
        };

        let flated = fs::read(path).expect("could not read lvl64");
        let lvlbytes = inflate_bytes(&flated).expect("could not decompress lvl64");
        let lvlstr = String::from_utf8(lvlbytes).expect("could not parse lvl64 as utf8");
        let json: Value =
            serde_json::from_str::<Value>(&lvlstr).expect("could not parse lvl64 as json");
        let level = &json["Content"]["$values"][0];
        //println!("parsing level {}", level["LevelID"]);
        let areas = level["Areas"].as_array().unwrap();

        for area in areas {
            let objects = area["Objects"].as_array().unwrap();
            for object in objects {
                let command = BASE64_STANDARD
                    .decode(object["Buffer"].as_str().unwrap())
                    .unwrap();
                if let &Some(_) = &memop {
                    let offset = banks[command[20] as usize];

                    used.insert(
                        ((command[21] as u32) << 16)
                            + ((command[22] as u32) << 8)
                            + (command[23] as u32)
                            + offset,
                    );
                } else {
                    used.insert(u32::from_be_bytes(command[20..24].try_into().unwrap()));
                }
            }
        }
    }

    println!("{}", used.len());

    let mut vecused = Vec::from_iter(used);
    vecused.sort(); // this isn't ideal, but it prevents personalization

    let out = vecused
        .iter_mut()
        .map(|v| {
            if let Some(sym) = &symop {
                let mut name = "unknown";
                for b in sym {
                    if b.0 == *v {
                        name = b.1
                    }
                }

                format!("{:#010x} {}", v, name)
            } else {
                format!("{:#010x}", v)
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", out);
}
