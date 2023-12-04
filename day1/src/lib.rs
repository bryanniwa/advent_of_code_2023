pub fn part_one(lines: &[String]) {
    let sum: u32 = lines
        .iter()
        .map(|line| line.trim_matches(|c: char| !c.is_numeric()))
        .map(|trimmed| {
            trimmed.chars().next().unwrap().to_digit(10).unwrap() * 10
                + trimmed.chars().last().unwrap().to_digit(10).unwrap()
        })
        .sum();
    println!("{sum}")
}

pub fn part_two(lines: &[String]) {
    use regex::Regex;
    use std::collections::HashMap;

    let number_map = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
        ("0", 0),
    ]);

    let one = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();
    let two = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();

    let sum: u32 = lines
        .iter()
        .map(|line| {
            let first = one.captures(line).unwrap();
            let second = two.captures(line).unwrap();
            (first, second)
        })
        .map(|captures| {
            let (cap1, cap2) = captures;

            let first = cap1.get(1).unwrap().as_str();
            let second = cap2.get(1).unwrap().as_str();
            number_map.get(first).unwrap() * 10 + number_map.get(second).unwrap()
        })
        .sum();

    println!("{sum}")
}
