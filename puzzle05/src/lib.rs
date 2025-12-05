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
    let parts = input.split_once("\n\n").expect("Should contain two parts");

    let mut fresh_ingredient_ranges: Vec<(i64, i64)> = parts
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

    fresh_ingredient_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged = vec![fresh_ingredient_ranges[0]];
    for (start, end) in fresh_ingredient_ranges {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        if start <= last_end {
            merged[last_idx] = (last_start, last_end.max(end))
        } else {
            merged.push((start, end));
        }
    }

    let mut result = 0;
    for (start, end) in merged {
        result += end - start + 1
    }

    return result;
}
