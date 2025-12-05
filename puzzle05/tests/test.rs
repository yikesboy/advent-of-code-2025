use puzzle05::{part1, part2};

#[test]
fn test_part1_example() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(part1(input), 3);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part1(input), 737);
}

#[test]
fn test_part2_example() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(part2(input), 14);
}

#[test]
fn test_part2_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part2(input), 357485433193284);
}
