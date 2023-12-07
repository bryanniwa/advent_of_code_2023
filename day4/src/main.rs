use std::{env, io};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_input() -> io::Result<Vec<String>> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?)
    }

    Ok(lines)
}

fn main() -> io::Result<()> {
    let lines = get_input()?;
    
    day4::part_one(&lines);
    day4::part_two(&lines);

    Ok(())
}
