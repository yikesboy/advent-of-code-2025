use puzzle02::{part1, part2};

#[test]
fn test_part1_example() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(part1(input), 1227775554);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part1(input), 28146997880);
}

#[test]
fn test_part2_example() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(part2(input), 4174379265);
}

#[test]
fn test_part2_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part2(input), 40028128307);
}
