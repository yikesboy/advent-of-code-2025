pub fn part1(input: &str) -> i64 {
    let mut lines: Vec<&str> = input.lines().collect();
    let operations: Vec<char> = lines
        .pop()
        .expect("should contain operations line")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let number_lines: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    return operations
        .iter()
        .enumerate()
        .fold(0, |acc, (index, op)| match op {
            '+' => {
                acc + number_lines
                    .iter()
                    .fold(0, |inner_acc, curr| inner_acc + curr[index])
            }
            '*' => {
                acc + number_lines
                    .iter()
                    .fold(1, |inner_acc, curr| inner_acc * curr[index])
            }
            _ => panic!("unknwon operator"),
        });
}

pub fn part2(input: &str) -> i64 {
    0
}
