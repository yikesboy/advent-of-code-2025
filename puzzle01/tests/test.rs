use puzzle01::{part1, part2};

#[test]
fn test_part1_example() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    assert_eq!(part1(input), 3);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part1(input), 1180)
}

#[test]
fn test_part2_example() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    assert_eq!(part2(input), 6);
}

#[test]
fn test_part2_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part2(input), 6892);
}
