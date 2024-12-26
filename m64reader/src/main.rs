// written by carter. 20241226
// "LOOK AT ME BITCHES I TURNED OUT FINE"
use std::{env, fs, io};

const MUSIC_START: usize = 0x1210000;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let romname = args.get(1).expect("you didn't specify a rom!");

    let f = fs::File::open(romname)?;
    let mut reader = io::BufReader::new(f);

    Ok(())
}
