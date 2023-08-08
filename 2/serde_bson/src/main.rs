use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Seek, Write},
};

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("temp.txt")?;
    let mut writer = BufWriter::new(&file);

    let mut n: u64 = 0;
    for i in 0..1000 {
        let a = Move {
            direction: Direction::Left,
            distance: i,
        };

        let j = serde_json::to_string(&a)?;

        n += writer.write(j.as_bytes())? as u64;
        let _ = writer.seek(std::io::SeekFrom::Start(n));
    }
    let _ = writer.flush()?;

    let file = File::open("temp.txt")?;

    let mut reader = BufReader::new(file);

    let deserializer = serde_json::Deserializer::from_reader(&mut reader);

    let iterator = deserializer.into_iter::<Move>();

    for item in iterator {
        println!("{:?}", item?);
    }

    Ok(())
}
