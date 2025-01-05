use base64::prelude::*;
use inflate::inflate_bytes;
use serde_json::Value;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = fs::read_dir(args.get(1).expect("you didn't specify a directory!"))
        .expect("couldn't read dir");

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
                used.insert(u32::from_be_bytes(command[20..24].try_into().unwrap()));
            }
        }

        println!(
            "{}",
            used.iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
