use std::{fs::File, io::{BufWriter, Write, BufReader}};

use serde::{Serialize, Deserialize};
use eyre::Result;

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
    let a = Move {
        direction: Direction::Left,
        distance: 2,
    };

    let j = serde_json::to_string(&a)?;

    let file = File::open("temp.txt")?;
    let mut writer = BufWriter::new(&file);
    let mut reader = BufReader::new(&file);

    let _ = writer.write(j.as_bytes());

    let j: Move = serde_json::from_reader(&mut reader)?;

    println!("{:?}", j);

    Ok(())
}
