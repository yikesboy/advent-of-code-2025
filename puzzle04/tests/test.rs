use puzzle04::{part1, part2};

#[test]
fn test_part1_example() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    assert_eq!(part1(input), 13);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part1(input), 1474);
}

#[test]
fn test_part2_example() {
    let input = "";
    assert_eq!(part2(input), 0);
}
