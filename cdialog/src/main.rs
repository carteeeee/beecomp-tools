// written by carter. 20122024
// "free your thoughts and watch them fly"
use std::env;
use std::fs::read;

const START: usize = 8433290;
const LEN: usize = 33428;

fn main() {
    let args: Vec<String> = env::args().collect();
    let romname = args.get(1).expect("you didn't specify a rom!");

    let rom = read(romname).expect("could not read rom!");

    // bro just said "7"
    for i in 0..LEN {
        print!("{}", rom[START + i] as char);
    }

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
