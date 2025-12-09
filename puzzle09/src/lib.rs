#[derive(Clone, Copy)]
struct RedTile {
    x: i32,
    y: i32,
}

struct Rectangle {
    t1: RedTile,
    t2: RedTile,
    area: i64,
}

pub fn part1(input: &str) -> i64 {
    let red_tiles: Vec<RedTile> = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .expect("'{line}' - should contain two values")
        })
        .map(|(x, y)| RedTile {
            x: (x.parse::<i32>().expect("'{x}' - should be an integer")),
            y: (y.parse::<i32>().expect("'{y}' - should be an integer")),
        })
        .collect();

    let mut pairs: Vec<Rectangle> = Vec::new();
    for (index, t1) in red_tiles.iter().enumerate() {
        for t2 in red_tiles[index + 1..].iter() {
            let dx = ((t1.x - t2.x).abs() + 1) as i64;
            let dy = ((t1.y - t2.y).abs() + 1) as i64;
            let area = dx * dy;
            pairs.push(Rectangle {
                t1: *t1,
                t2: *t2,
                area: area,
            })
        }
    }

    pairs.sort_by_key(|r| r.area);

    return pairs.last().unwrap().area;
}

pub fn part2(input: &str) -> i64 {
    return 0;
}
