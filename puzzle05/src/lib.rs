pub fn part1(input: &str) -> i32 {
    let parts = input.split_once("\n\n").expect("Should contain two parts");

    let fresh_ingredient_ranges: Vec<(i64, i64)> = parts
        .0
        .lines()
        .map(|line| line.split_once("-").unzip())
        .map(|tuple| {
            (
                tuple.0.unwrap().parse::<i64>().expect("should be num"),
                tuple.1.unwrap().parse::<i64>().expect("should be num"),
            )
        })
        .collect();

    let available_ingredient_ids: Vec<i64> = parts
        .1
        .lines()
        .map(|line| line.parse().expect(""))
        .collect();

    let mut result = 0;
    for id in available_ingredient_ids {
        for (start, end) in fresh_ingredient_ranges.clone() {
            if id >= start && id <= end {
                result += 1;
                break;
            }
        }
    }

    return result;
}

pub fn part2(input: &str) -> i64 {
    0
}
