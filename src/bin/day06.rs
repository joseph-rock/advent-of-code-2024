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
}

impl Cursor {
    fn find_start(puzzle_map: &Vec<Vec<char>>) -> Option<Cursor> {
        for (y, row) in puzzle_map.iter().enumerate() {
            for (x, character) in row.iter().enumerate() {
                if let Some(direction) = Direction::char_direction(character) {
                    return Some(Cursor {
                        symbol: *character,
                        direction,
                        x,
                        y,
                    });
                }
            }
        }
        None
    }

    // Fix to handle possiblility of usize going negative
    fn move_forward(cursor: &Cursor) -> Cursor {
        let symbol = cursor.symbol;
        let direction = cursor.direction;
        match cursor.direction {
            Direction::North => Cursor {
                symbol,
                x: cursor.x,
                y: cursor.y - 1,
                direction,
            },
            Direction::South => Cursor {
                symbol,
                x: cursor.x,
                y: cursor.y + 1,
                direction,
            },
            Direction::East => Cursor {
                symbol,
                x: cursor.x + 1,
                y: cursor.y,
                direction,
            },
            Direction::West => Cursor {
                symbol,
                x: cursor.x - 1,
                y: cursor.y,
                direction,
            },
        }
    }

    fn rotate_clockwise(cursor: &Cursor) -> Cursor {
        // Todo: update symbol when rotated
        let symbol = cursor.symbol;
        let x = cursor.x;
        let y = cursor.y;
        match cursor.direction {
            Direction::North => Cursor {
                symbol,
                direction: Direction::East,
                x,
                y,
            },
            Direction::South => Cursor {
                symbol,
                direction: Direction::West,
                x,
                y,
            },
            Direction::East => Cursor {
                symbol,
                direction: Direction::South,
                x,
                y,
            },
            Direction::West => Cursor {
                symbol,
                direction: Direction::North,
                x,
                y,
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

impl Direction {
    fn char_direction(c: &char) -> Option<Direction> {
        match c {
            '<' => Some(Direction::West),
            'v' => Some(Direction::South),
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            _ => None,
        }
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

    let mut next = Cursor::move_forward(&m.cursor);

    // usize cannot go negative, fix in move_forward()
    while next.x < *x_max && next.y < *y_max
    // && next.coordinate.y >= 0
    // && next.coordinate.x >= 0
    {
        if m.puzzle_map[next.y][next.x] == '#' {
            next = Cursor::rotate_clockwise(&m.cursor);
            m.cursor = next;
        } else {
            m.puzzle_map[m.cursor.y][m.cursor.x] = marker;
            m.cursor = next;
        }
        next = Cursor::move_forward(&m.cursor);
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
