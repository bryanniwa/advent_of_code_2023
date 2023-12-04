use regex::Regex;
use std::collections::HashMap;

pub fn part_one(lines: &[String]) {
    let known_values = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let re = Regex::new(r"(\d+) (blue|red|green)(,|;)?").unwrap();

    let sum: usize = lines
        .iter()
        .zip(1..=lines.len())
        .filter(|(line, _)| {
            re.captures_iter(line).all(|c| {
                let color = c.get(2).unwrap().as_str();
                let num = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
                &num <= known_values.get(color).unwrap_or(&0)
            })
        })
        .map(|(_, i)| i)
        .sum();

    println!("Part one: {}", sum);
}

pub fn part_two(lines: &[String]) {
    use std::cmp;
    let re = Regex::new(r"(\d+) (blue|red|green)(,|;)?").unwrap();

    let sum: u32 = lines
        .iter()
        .map(|line| {
            let mut local_maxes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            re.captures_iter(line).for_each(|c| {
                let color = c.get(2).unwrap().as_str();
                let num = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                local_maxes.insert(color, cmp::max(*local_maxes.get(color).unwrap(), num));
            });

            local_maxes.values().product::<u32>()
        })
        .sum();

    println!("Part two: {sum}");
}
