use puzzle03::{part1, part2};

#[test]
fn test_part1_example() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(part1(input), 357);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part1(input), 17311);
}

#[test]
fn test_part2_example() {
    let input = "";
    assert_eq!(part2(input), 0);
}
