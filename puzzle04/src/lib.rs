#[derive(PartialEq)]
enum Tile {
    Filled,
    Empty,
}

impl Tile {
    pub fn from_char(field: char) -> Option<Tile> {
        match field {
            '@' => Some(Tile::Filled),
            '.' => Some(Tile::Empty),
            _ => None,
        }
    }
}

enum DirectionVector {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl DirectionVector {
    pub fn all() -> [DirectionVector; 8] {
        [
            DirectionVector::Up,
            DirectionVector::Down,
            DirectionVector::Left,
            DirectionVector::Right,
            DirectionVector::UpLeft,
            DirectionVector::UpRight,
            DirectionVector::DownLeft,
            DirectionVector::DownRight,
        ]
    }

    pub fn dir(&self) -> (i32, i32) {
        match self {
            DirectionVector::Up => (-1, 0),
            DirectionVector::Down => (1, 0),
            DirectionVector::Left => (0, -1),
            DirectionVector::Right => (0, 1),
            DirectionVector::UpLeft => (-1, -1),
            DirectionVector::UpRight => (-1, 1),
            DirectionVector::DownLeft => (1, -1),
            DirectionVector::DownRight => (1, 1),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let matrix = create_matrix(input);
    let matrix_height = matrix.len();
    let matrix_width = matrix.first().unwrap().len();

    let mut accessible_rolls = 0;

    for row in 0..matrix_height {
        for col in 0..matrix_width {
            if matrix[row][col] == Tile::Filled {
                let mut neighbour_count = 0;

                for dir in DirectionVector::all() {
                    let (dir_row, dir_col) = dir.dir();

                    let check_row = row as i32 + dir_row;
                    let check_col = col as i32 + dir_col;

                    if check_row >= 0
                        && check_row < (matrix_height as i32)
                        && check_col >= 0
                        && check_col < (matrix_width as i32)
                    {
                        if matrix[check_row as usize][check_col as usize] == Tile::Filled {
                            neighbour_count += 1;
                        }
                    }
                }

                if neighbour_count < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }

    accessible_rolls
}

pub fn part2(input: &str) -> i64 {
    0
}

fn create_matrix(input: &str) -> Vec<Vec<Tile>> {
    return input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| Tile::from_char(c).expect("should be either"))
                .collect()
        })
        .collect();
}
