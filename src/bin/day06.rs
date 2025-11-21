use std::{thread, time::Duration};

#[derive(Debug, Clone)]
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
    icon: char,
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
                        icon: *character,
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
                icon: cursor.icon,
                x: cursor.x,
                y: cursor.y.saturating_sub(1),
                direction: cursor.direction,
                in_bounds: cursor.y > 0,
            },
            Direction::South => Cursor {
                icon: cursor.icon,
                x: cursor.x,
                y: cursor.y + 1,
                direction: cursor.direction,
                in_bounds: cursor.y < *y_max - 1,
            },
            Direction::East => Cursor {
                icon: cursor.icon,
                x: cursor.x + 1,
                y: cursor.y,
                direction: cursor.direction,
                in_bounds: cursor.x < *x_max - 1,
            },
            Direction::West => Cursor {
                icon: cursor.icon,
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
                icon: direction_char(Direction::East),
                direction: Direction::East,
                x: cursor.x,
                y: cursor.y,
                in_bounds: cursor.in_bounds,
            },
            Direction::South => Cursor {
                icon: direction_char(Direction::West),
                direction: Direction::West,
                x: cursor.x,
                y: cursor.y,
                in_bounds: cursor.in_bounds,
            },
            Direction::East => Cursor {
                icon: direction_char(Direction::South),
                direction: Direction::South,
                x: cursor.x,
                y: cursor.y,
                in_bounds: cursor.in_bounds,
            },
            Direction::West => Cursor {
                icon: direction_char(Direction::North),
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

// Maybe too fancy? Nested for might be easier to read.
fn total_marker(map: &Vec<Vec<char>>, marker: &char) -> usize {
    map.iter().fold(0, |sum, row| {
        sum + row.iter().filter(|&letter| letter == marker).count()
    })
}

fn draw_screen(map: &Map, x_max: &usize, y_max: &usize) -> () {
    struct DisplayConfig {
        width: usize,
        height: usize,
        pause_len: Duration,
    }

    let config = DisplayConfig {
        width: 100,
        height: 50,
        pause_len: Duration::from_millis(20),
    };

    // Copy map and place cursor icon
    let mut map_copy = map.clone();
    let cursor_x = map_copy.cursor.x;
    let cursor_y = map_copy.cursor.y;
    map_copy.puzzle_map[cursor_y][cursor_x] = map.cursor.icon;

    // --- Calculate display size ---
    let mut local_x_min = 0;
    let mut local_x_max = 0;
    let mut local_y_min = 0;
    let mut local_y_max = 0;

    // Width greater than row len
    if config.width >= *x_max {
        local_x_min = 0;
        local_x_max = *x_max;
    }
    // Cursor too close to west border
    else if cursor_x <= config.width / 2 {
        local_x_min = 0;
        local_x_max = config.width;
    }
    // Cursor too close to east border
    else if cursor_x + (config.width / 2) >= *x_max {
        local_x_min = *x_max - config.width;
        local_x_max = *x_max;
    }
    // Cursor safely inside width boundaries
    else {
        local_x_min = cursor_x - (config.width / 2);
        local_x_max = cursor_x + (config.width / 2);
    }

    // Height greater than col len
    if config.height >= *y_max {
        local_y_min = 0;
        local_y_max = *y_max;
    }
    // Cursor too close to north border
    else if cursor_y <= config.height / 2 {
        local_y_min = 0;
        local_y_max = config.height;
    }
    // Cursor too close to south border
    else if cursor_y + (config.height / 2) >= *y_max {
        local_y_min = *y_max - config.height;
        local_y_max = *y_max;
    }
    // Cursor safely inside height boundaries
    else {
        local_y_min = cursor_y - (config.height / 2);
        local_y_max = cursor_y + (config.height / 2);
    }

    // --- Draw Screen ---
    // Hacky clear screen
    print!("\x1B[2J\x1B[1;1H");

    // Draw map
    for row_vec in local_y_min..local_y_max {
        let row: String = map_copy.puzzle_map[row_vec][local_x_min..local_x_max]
            .iter()
            .collect();
        println!("{}", row);
    }

    // Sleep
    thread::sleep(config.pause_len);
}

fn part_1(input: &str) -> usize {
    let mut m = Map::new(input);
    let x_max = &m.puzzle_map[0].len();
    let y_max = &m.puzzle_map.len();
    let marker = 'x';

    let mut next = Cursor::move_forward(&m.cursor, &x_max, &y_max);
    while next.in_bounds {
        draw_screen(&m, &x_max, &y_max);
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
    draw_screen(&m, &x_max, &y_max);

    total_marker(&m.puzzle_map, &marker)
}

fn main() {
    let input: &str = include_str!("./day06.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}
