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
    let mut lines: Vec<&str> = input.lines().collect();
    let operations_line: &str = lines.pop().expect("should contain operations line");
    let number_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut operation_indices: Vec<(usize, char)> = operations_line
        .char_indices()
        .filter(|(_, c)| !c.is_whitespace())
        .collect();

    operation_indices.push((operations_line.len(), ' '));

    let mut result = 0;
    for window in operation_indices.windows(2) {
        let (start_col, op) = window[0];
        let (end_col, _) = window[1];

        let values = (start_col..end_col).filter_map(|col| {
            let col_str: String = number_lines
                .iter()
                .filter_map(|row| row.get(col))
                .filter(|c| !c.is_whitespace())
                .collect();

            col_str.parse::<i64>().ok()
        });

        match op {
            '+' => result += values.fold(0, |acc, curr| acc + curr),
            '*' => result += values.fold(1, |acc, curr| acc * curr),
            _ => panic!("unknown op"),
        }
    }

    return result;
}
