use base64::prelude::*;
use inflate::inflate_bytes;
use serde_json::Value;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::{env, fs};

const SSEGMENTTABLE: usize = 0x33b400;
const NUM_BANKS: usize = 32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mem =
        fs::read(args.get(1).expect("you didn't specify a ram dump!")).expect("couldn't read ram");
    let dir = fs::read_dir(args.get(2).expect("you didn't specify a directory!"))
        .expect("couldn't read dir");
    let seg = args
        .get(3)
        .expect("you didn't specify segmented/virt addresses!")
        == "true";

    let mut banks: Vec<u32> = Vec::with_capacity(NUM_BANKS);
    banks.resize(NUM_BANKS, 0);
    for i in 0..NUM_BANKS {
        let offset = SSEGMENTTABLE + (i * 4);
        banks[i] = u32::from_be_bytes(mem[offset..(offset + 4)].try_into().unwrap());
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
                if seg {
                    used.insert(u32::from_be_bytes(command[20..24].try_into().unwrap()));
                } else {
                    let offset = banks[command[20] as usize];

                    used.insert(
                        ((command[21] as u32) << 16)
                            + ((command[22] as u32) << 8)
                            + (command[23] as u32)
                            + offset,
                    );
                }
            }
        }
    }

    println!("{}", used.len());

    let mut vecused = used
        .iter()
        .map(|v| format!("{:#010x}", v))
        .collect::<Vec<String>>();
    vecused.sort(); // this isn't ideal, but it prevents personalization

    println!("{}", vecused.join("\n"));
}
