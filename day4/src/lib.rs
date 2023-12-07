pub fn part_one(lines: &[String]) {
    let mut sum = 0;

    for line in lines {
        let mut local_sum = 0;
        let line = line.split(':').skip(1).collect::<Vec<&str>>();
        let line = line[0].split('|').collect::<Vec<&str>>();

        let winning_numbers = line[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        for num in line[1].split(' ').filter(|s| !s.is_empty()) {
            if winning_numbers.contains(&num) {
                if local_sum == 0 {
                    local_sum += 1;
                } else {
                    local_sum *= 2;
                }
            }
        }

        sum += local_sum;
    }

    println!("Sum: {}", sum);
}

pub fn part_two(lines: &[String]) {
    println!("Not done yet");
}
