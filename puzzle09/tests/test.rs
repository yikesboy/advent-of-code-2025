use puzzle09::{part1, part2};

#[test]
fn test_part1_example() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    assert_eq!(part1(input), 50);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part1(input), 0);
}
