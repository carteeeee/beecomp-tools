// written by carter. 20241220
// "free your thoughts and watch them fly"
use std::env;
use std::fs::read;

mod texttable;
const TEXT_START: usize = 8433290;
const TEXT_LEN: usize = 33428;
const DATA_START: usize = 8466718;

fn stu(slice: &[u8]) -> u16 {
    slice[1] as u16 + ((slice[0] as u16) << 8)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let romname = args.get(1).expect("you didn't specify a rom!");

    let rom = read(romname).expect("could not read rom!");

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
            1u32,              // unused, seems to always be 1 and i am lazy
            data[4],           // linesPerBox
            stu(&data[6..8]),  // leftOffset
            stu(&data[8..10]), // width
        ));
    }

    let mut index = 0u8;
    let cdialog: Vec<String> = dialogstext
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
                dialogtext
                    .lines()
                    .map(|line| format!("{}\\n\\", line))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
        })
        .collect();
    println!("{}", cdialog[0]);

    //println!("{:?}", dialogs);

    /* 		function storeDialogue(storeHere, head, maxSize) {
        let phrase = "";

        for (let i = 0, textIndex = 0; i < maxSize; i++) {
            let character = rom[head + i];

            if (textTable[character] == undefined) {
                // Definitely might not be needed.
                console.log(character);
            }

            if (character == 255) {
                output.text[storeHere][textIndex] = phrase;
                textIndex++;
                phrase = "";
            } else {
                phrase += textTable[character];
            }
        }
    }*/
}
