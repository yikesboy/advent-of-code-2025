pub fn part1(input: &str) -> i32 {
    let batteries = input.lines();
    let mut total_output_joltage = 0;

    for battery in batteries {
        let joltage_ratings: Vec<u32> = battery.chars().filter_map(|c| c.to_digit(10)).collect();
        let len = joltage_ratings.len();

        let mut max_rating_after: Vec<i32> = vec![0; len];
        let mut max_rating_seen = 0;
        for i in (0..len).rev() {
            max_rating_after[i] = max_rating_seen;

            let current = joltage_ratings[i] as i32;
            max_rating_seen = max_rating_seen.max(current);
        }

        let mut largest = 0;
        for i in 0..len - 1 {
            let first_digit_candidate = joltage_ratings[i].to_string();
            let second_digit_candidate = max_rating_after[i].to_string();
            let candidate_str = first_digit_candidate + second_digit_candidate.as_str();
            let candidate: i32 = candidate_str.parse().expect("should be a number");

            largest = largest.max(candidate);
        }

        total_output_joltage += largest;
    }

    total_output_joltage
}

pub fn part2(input: &str) -> i64 {
    0
}
