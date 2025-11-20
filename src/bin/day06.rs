#[derive(Debug)]
struct Map {
    puzzle_map: Vec<Vec<char>>,
    cursor: Cursor,
}

impl Map {
    fn new(input: &str) -> Map {
        let puzzle_map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        let cursor: Cursor = Cursor::find_start(&puzzle_map).unwrap();

        Map { puzzle_map, cursor }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cursor {
    symbol: char,
    direction: Direction,
    x: usize,
    y: usize,
    in_bounds: bool,
}

impl Cursor {
    fn find_start(puzzle_map: &Vec<Vec<char>>) -> Option<Cursor> {
        for (y, row) in puzzle_map.iter().enumerate() {
            for (x, character) in row.iter().enumerate() {
                if let Some(direction) = char_direction(character) {
                    return Some(Cursor {
                        symbol: *character,
                        direction,
                        x,
                        y,
                        in_bounds: true,
                    });
                }
            }
        }
        None
    }

    fn move_forward(cursor: &Cursor, x_max: &usize, y_max: &usize) -> Cursor {
        match cursor.direction {
            Direction::North => Cursor {
                symbol: cursor.symbol,
                x: cursor.x,
                y: cursor.y.saturating_sub(1),
                direction: cursor.direction,
                in_bounds: cursor.y > 0,
            },
            Direction::South => Cursor {
                symbol: cursor.symbol,
                x: cursor.x,
                y: cursor.y + 1,
                direction: cursor.direction,
                in_bounds: cursor.y < *y_max - 1,
            },
            Direction::East => Cursor {
                symbol: cursor.symbol,
                x: cursor.x + 1,
                y: cursor.y,
                direction: cursor.direction,
                in_bounds: cursor.x < *x_max - 1,
            },
            Direction::West => Cursor {
                symbol: cursor.symbol,
                x: cursor.x.saturating_sub(1),
                y: cursor.y,
                direction: cursor.direction,
                in_bounds: cursor.x > 0,
            },
        }
    }

    fn rotate_clockwise(cursor: &Cursor) -> Cursor {
        match cursor.direction {
            Direction::North => Cursor {
                symbol: direction_char(Direction::East),
                direction: Direction::East,
                x: cursor.x,
                y: cursor.y,
                in_bounds: cursor.in_bounds,
            },
            Direction::South => Cursor {
                symbol: direction_char(Direction::West),
                direction: Direction::West,
                x: cursor.x,
                y: cursor.y,
                in_bounds: cursor.in_bounds,
            },
            Direction::East => Cursor {
                symbol: direction_char(Direction::South),
                direction: Direction::South,
                x: cursor.x,
                y: cursor.y,
                in_bounds: cursor.in_bounds,
            },
            Direction::West => Cursor {
                symbol: direction_char(Direction::North),
                direction: Direction::North,
                x: cursor.x,
                y: cursor.y,
                in_bounds: cursor.in_bounds,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn char_direction(c: &char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::North),
        'v' => Some(Direction::South),
        '>' => Some(Direction::East),
        '<' => Some(Direction::West),
        _ => None,
    }
}

fn direction_char(direction: Direction) -> char {
    match direction {
        Direction::North => '^',
        Direction::South => 'v',
        Direction::East => '>',
        Direction::West => '<',
    }
}

fn total_marker(map: &Vec<Vec<char>>, marker: &char) -> usize {
    let mut total = 0;
    for line in map {
        for c in line {
            if c == marker {
                total += 1;
            }
        }
    }
    total
}

fn part_1(input: &str) -> usize {
    let mut m = Map::new(input);
    let x_max = &m.puzzle_map[0].len();
    let y_max = &m.puzzle_map.len();
    let marker = 'x';

    let mut next = Cursor::move_forward(&m.cursor, &x_max, &y_max);

    // usize cannot go negative, fix in move_forward()
    while next.in_bounds {
        if m.puzzle_map[next.y][next.x] == '#' {
            next = Cursor::rotate_clockwise(&m.cursor);
            m.cursor = next;
        } else {
            m.puzzle_map[m.cursor.y][m.cursor.x] = marker;
            m.cursor = next;
        }
        next = Cursor::move_forward(&m.cursor, &x_max, &y_max);
    }
    m.puzzle_map[m.cursor.y][m.cursor.x] = marker;
    total_marker(&m.puzzle_map, &marker)
}

fn main() {
    let input: &str = include_str!("./day06.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}
