use std::fs;
use std::io::{self, Read};
use std::path::Path;

enum Direction {
    Left,
    Right,
}

fn main() -> io::Result<()> {
    let path = Path::new("src/input.txt");
    let mut file = fs::File::open(path)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let lines: Vec<&str> = data.lines().collect();

    let mut pos = 50;
    let mut zero_counter = 0;
    for line in &lines {
        let direction = if let Some(first_char) = line.chars().next() {
            match first_char {
                'L' => {
                    Direction::Left
                }
                'R' => {
                    Direction::Right
                }
                _ => {
                    println!("Found invalid firstchar: {first_char}");
                    std::process::exit(1)
                }
            }
        } else {
            println!("Line is empty");
            std::process::exit(1)
        };
        let rest = &line[1..];
        let value: i32 = rest.parse().unwrap();
        // Now you can use the direction variable
        match direction {
            Direction::Left => {
                println!("Line starts with Left");
                pos = (pos - value) % 100;
            }
            Direction::Right => {
                println!("Line starts with Right");
                pos = (pos + value) % 100;
            }
        }
        if pos == 0 {
            zero_counter += 1;
        }
    }
    println!("{zero_counter}");
    Ok(())
}
