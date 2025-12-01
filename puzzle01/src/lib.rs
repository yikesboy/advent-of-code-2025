pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn apply(self, value: i32) -> i32 {
        match self {
            Side::Left => -1 * value,
            Side::Right => value,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        if let Some(first_char) = s.chars().next() {
            match first_char {
                'L' => Some(Side::Left),
                'R' => Some(Side::Right),
                _ => None,
            }
        } else {
            return None;
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut start: i32 = 50;
    let mut result: i32 = 0;

    for line in input.lines() {
        if let Some(side) = Side::from_str(line) {
            if let Ok(value) = line[1..].trim().parse::<i32>() {
                let side_value = side.apply(value);
                start = (start + side_value) % 100;
                if start == 0 {
                    result += 1;
                }
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i32 {
    0
}
