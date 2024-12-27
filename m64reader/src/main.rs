// written by carter. 20241226
// "LOOK AT ME BITCHES I TURNED OUT FINE"
use std::env;
use std::fs::{create_dir, File};
use std::io::{BufReader, Read, Result, Seek, SeekFrom, Write};
use std::path::Path;

const MUSIC_START: u32 = 0x1210000;

fn read_u32(reader: &mut impl Read) -> Result<u32> {
    let mut buf: [u8; 4] = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let romname = args.get(1).expect("you didn't specify a rom!");
    let output = args.get(2).map_or(Path::new("output"), |v| Path::new(v));

    create_dir(output)?;

    let f = File::open(romname)?;
    let mut reader = BufReader::new(f);

    reader.seek_relative(MUSIC_START.into())?;
    let num = read_u32(&mut reader)? & 0xffff;
    let mut music: Vec<(u32, u32)> = Vec::with_capacity(num as usize);

    for _ in 0..num {
        music.push((read_u32(&mut reader)?, read_u32(&mut reader)?));
    }

    reader.seek_relative(4)?;

    for i in 0..(num as usize) {
        reader.seek(SeekFrom::Start((MUSIC_START + music[i].0).into()))?;
        let mut buf = Vec::with_capacity(music[i].1 as usize);
        buf.resize(music[i].1 as usize, 0);
        reader.read_exact(&mut buf)?;
        let mut out = File::create(output.join(format!("{:03}.m64", i)))?;
        out.write_all(&buf)?;
    }

    Ok(())
}
