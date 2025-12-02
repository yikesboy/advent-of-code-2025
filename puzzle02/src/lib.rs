pub fn part1(input: &str) -> i64 {
    let range_strs: Vec<&str> = input.split(',').collect();
    let mut sum_invalid_ids = 0;

    for range in range_strs {
        let (range_start, range_end) = range.split_once('-').expect("should be just one -");
        let range_start: i64 = range_start.trim().parse().expect("Invalid range start.");
        let range_end: i64 = range_end.trim().parse().expect("Invalid range end.");

        for i in range_start..=range_end {
            let current = i.to_string();
            let len = current.chars().count();

            if len % 2 != 0 {
                continue;
            }

            let half = len / 2;
            let first_half = &current.as_str()[0..half];
            let second_half = &current.as_str()[half..];

            if first_half == second_half {
                sum_invalid_ids += i;
            }
        }
    }

    return sum_invalid_ids;
}

pub fn part2(input: &str) -> i64 {
    let range_strs: Vec<&str> = input.split(',').collect();
    let mut sum_invalid_ids = 0;

    for range in range_strs {
        let (range_start, range_end) = range.split_once('-').expect("should be just one -");
        let range_start: i64 = range_start.trim().parse().expect("Invalid range start.");
        let range_end: i64 = range_end.trim().parse().expect("Invalid range end.");

        for i in range_start..=range_end {
            let current = i.to_string();
            let len = current.chars().count();

            for period in 1..=len / 2 {
                let pattern = &current.as_str()[..period];
                let mut is_repeated = true;

                for i in (period..len).step_by(period) {
                    if i + period > len || &current[i..i + period] != pattern {
                        is_repeated = false;
                        break;
                    }
                }

                if is_repeated && len % period == 0 {
                    sum_invalid_ids += i;
                    break;
                }
            }
        }
    }
    return sum_invalid_ids;
}
