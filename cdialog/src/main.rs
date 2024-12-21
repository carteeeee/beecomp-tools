// written by carter. 20241220
// "free your thoughts and watch them fly"
use std::{env, fs};

mod texttable;
const TEXT_START: usize = 8433290;
const TEXT_LEN: usize = 33428;
const DATA_START: usize = 8466718;

fn add_newlines(input: &str) -> String {
    let mut lines = input
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();
    if let Some(last_line) = lines.pop() {
        lines
            .iter_mut()
            .for_each(|line| *line = format!("{}\\n\\", line));
        lines.push(last_line);
    }
    lines.join("\n")
}

macro_rules! u16frombytes {
    ($bytes:expr, $offset:literal) => {
        (($bytes[$offset] as u16) << 8) + ($bytes[$offset + 1] as u16)
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let romname = args.get(1).expect("you didn't specify a rom!");
    let output = args.get(2).map_or("dialog.c", |v| v);

    let rom = fs::read(romname).expect("could not read rom!");

    let table = texttable::texttable();
    let mut dialogstext = Vec::new();
    let mut dialogtext = String::new();
    let mut dialogsdata = Vec::new();
    for i in 0..TEXT_LEN {
        let character = rom[TEXT_START + i];
        if character == texttable::END {
            dialogstext.push(dialogtext);
            dialogtext = String::new();
        } else {
            dialogtext.push_str(table[&character]);
        }
    }

    for i in 0..dialogstext.len() {
        let offset = DATA_START + (i * 16);
        let data = &rom[offset..(offset + 16)];
        dialogsdata.push((
            1u32,                    // unused, seems to always be 1 and i am lazy
            data[4],                 // linesPerBox
            u16frombytes!(&data, 6), // leftOffset
            u16frombytes!(&data, 8), // width
        ));
    }

    let mut index = 0u8;
    let cdialog = dialogstext
        .iter()
        .map(|dialogtext| {
            index += 1;
            let data = dialogsdata[(index - 1) as usize];
            format!(
                "DEFINE_DIALOG(DIALOG_{:03}, {}, {}, {}, {}, _(\"\\\n{}\"))",
                index - 1,
                data.0,
                data.1,
                data.2,
                data.3,
                add_newlines(dialogtext),
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    fs::write(output, cdialog).expect("could not write dialog!");
}
