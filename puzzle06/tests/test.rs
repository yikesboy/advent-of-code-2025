use puzzle06::{part1, part2};

#[test]
fn test_part1_example() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    assert_eq!(part1(input), 4277556);
}

#[test]
fn test_part1_input() {
    let input = include_str!("../src/input.txt");
    assert_eq!(part1(input), 4719804927602);
}

#[test]
fn test_part2_example() {
    let input = "";
    assert_eq!(part2(input), 0);
}
