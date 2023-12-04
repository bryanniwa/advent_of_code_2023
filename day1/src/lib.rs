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

    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();

    let sum: u32 = lines
        .iter()
        // .map(|line| {
        //     println!("{}", line);
        //     line
        // })
        .map(|line| re.find_iter(line))
        .map(|mut captures| {
            let first = match captures.next() {
                Some(first) => first.as_str(),
                None => unreachable!("first"),
            };
            let second = match captures.last() {
                Some(second) => second.as_str(),
                None => first,
            };

            let res = number_map.get(first).unwrap() * 10 + number_map.get(second).unwrap();
            // println!("{res}");
            res
        })
        .sum();

    println!("{sum}")
}
