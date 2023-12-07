use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Id {
    x: usize,
    y: usize,
    id: u32,
}

impl Id {
    fn len(&self) -> usize {
        self.id.to_string().len()
    }

    // we might get some duplicates here, but that's fine
    fn get_adjacent(&self) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        let max_x = self.x + self.len();
        let min_x = if self.x > 0 { self.x - 1 } else { 0 };
        let min_y = if self.y > 0 { self.y - 1 } else { 0 };

        for x in min_x..=max_x {
            adjacent.push((x, min_y));
            adjacent.push((x, self.y + 1));
        }

        adjacent.push((max_x, self.y));
        adjacent.push((min_x, self.y));

        adjacent
    }
}

pub fn part_one(lines: &[String]) {
    let mut symbols = HashMap::new();
    let mut ids = Vec::new();

    extract_elements(lines, &mut symbols, &mut ids, false);

    let mut sum = 0;
    for id in ids {
        let val = id.id;
        for (x, y) in id.get_adjacent() {
            if symbols.contains_key(&(x, y)) {
                sum += val;
                break;
            }
        }
    }

    println!("Part one: {}", sum);
}

fn extract_elements(
    lines: &[String],
    symbols: &mut HashMap<(usize, usize), Vec<Id>>,
    ids: &mut Vec<Id>,
    p2: bool,
) {
    lines.iter().enumerate().for_each(|(y, line)| {
        let mut chars = line.chars().enumerate();
        while let Some((x, c)) = chars.next() {
            if c.is_ascii_digit() {
                let mut num = Id {
                    x,
                    y,
                    id: c.to_digit(10).unwrap(),
                };

                for (x, c) in chars.by_ref() {
                    if c.is_ascii_digit() {
                        let val = c.to_digit(10).unwrap();
                        num.id = num.id * 10 + val;
                    } else {
                        if c != '.' {
                            if p2 && c != '*' {
                                break;
                            }
                            symbols.insert((x, y), Vec::new());
                        }
                        break;
                    }
                }

                ids.push(num);
            } else if c != '.' {
                if p2 && c != '*' {
                    continue;
                }
                symbols.insert((x, y), Vec::new());
            }
        }
    });
}

pub fn part_two(lines: &[String]) {
    let mut symbols = HashMap::new();
    let mut ids = Vec::new();

    extract_elements(lines, &mut symbols, &mut ids, true);

    for id in ids {
        for (x, y) in id.get_adjacent() {
            if let Some(v) = symbols.get_mut(&(x, y)) {
                v.push(id.clone());
            }
        }
    }

    let sum = symbols
        .values()
        .filter(|v| v.len() == 2)
        .fold(0, |acc, v| acc + v[0].id * v[1].id);

    println!("Part two: {}", sum);
}
