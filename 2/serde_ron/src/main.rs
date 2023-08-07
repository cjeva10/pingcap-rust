use eyre::Result;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use std::string::String;

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
    let a: Move = Move {
        direction: Direction::Left,
        distance: 2,
    };

    let mut file = Vec::new();

    let r = ron::to_string(&a)?;

    let mut writer = BufWriter::new(&mut file);

    let _ = writer.write(r.as_bytes())?;
    // explicitly drop writer so that borrow checker knows that the next line can own file
    drop(writer);

    let a: Move = ron::from_str(&String::from_utf8(file)?)?;

    println!("{:?}", a);

    Ok(())
}
