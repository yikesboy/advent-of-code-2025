#[derive(PartialEq)]
enum Tile {
    Start,
    Empty,
    Splitter,
    Beam,
}

impl Tile {
    pub fn from_char(tile: char) -> Self {
        match tile {
            'S' => Tile::Start,
            '.' => Tile::Empty,
            '^' => Tile::Splitter,
            '|' => Tile::Beam,
            _ => panic!("unknown tile value"),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut diagram: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
        .collect();

    let (index, _) = diagram
        .first()
        .expect("should have first row")
        .iter()
        .enumerate()
        .find(|(_, tile)| **tile == Tile::Start)
        .expect("should find start");

    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push((index, 0));

    let diagram_width = diagram[0].len();
    let diagram_height = diagram.len();
    let mut visited = vec![vec![false; diagram_width]; diagram_height];

    let mut times_split = 0;
    while let Some((x, y)) = queue.pop() {
        if visited[y][x] {
            continue;
        } else {
            visited[y][x] = true;
        }

        match diagram[y][x] {
            Tile::Start | Tile::Beam | Tile::Empty => {
                let new_y = y + 1;
                if let Some(row) = diagram.get(new_y) {
                    if let Some(_) = row.get(x) {
                        queue.push((x, new_y));
                    }
                }
            }
            Tile::Splitter => {
                let left_x = x - 1;
                let right_x = x + 1;
                times_split += 1;

                if let Some(row) = diagram.get(y) {
                    if let Some(_) = row.get(left_x) {
                        queue.push((left_x, y));
                    }
                    if let Some(_) = row.get(right_x) {
                        queue.push((right_x, y));
                    }
                }
            }
        }
    }

    return times_split;
}

pub fn part2(input: &str) -> i64 {
    return 0;
}
