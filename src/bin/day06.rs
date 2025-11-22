use std::{thread, time::Duration};

#[derive(Debug, Clone)]
struct Map {
    puzzle_map: Vec<Vec<char>>,
    cursor: Cursor,
    _x_max: usize,
    _y_max: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let puzzle_map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        let cursor: Cursor = Cursor::find_start(&puzzle_map).unwrap();
        let _x_max = puzzle_map[0].len();
        let _y_max = puzzle_map.len();

        Map {
            puzzle_map,
            cursor,
            _x_max,
            _y_max,
        }
    }

    fn peek_next(&self) -> Option<char> {
        match self.cursor.direction {
            Direction::North => self
                .puzzle_map
                .get(self.cursor.y - 1)?
                .get(self.cursor.x)
                .copied(),
            Direction::South => self
                .puzzle_map
                .get(self.cursor.y + 1)?
                .get(self.cursor.x)
                .copied(),
            Direction::East => self
                .puzzle_map
                .get(self.cursor.y)?
                .get(self.cursor.x + 1)
                .copied(),
            Direction::West => self
                .puzzle_map
                .get(self.cursor.y)?
                .get(self.cursor.x - 1)
                .copied(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cursor {
    icon: char,
    direction: Direction,
    x: usize,
    y: usize,
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
                    });
                }
            }
        }
        None
    }

    fn move_forward(&mut self) -> () {
        match self.direction {
            Direction::North => self.y = self.y.saturating_sub(1),
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
            Direction::West => self.x = self.x.saturating_sub(1),
        }
    }

    fn rotate_clockwise(&mut self) -> () {
        match self.direction {
            Direction::North => {
                self.icon = direction_char(Direction::East);
                self.direction = Direction::East;
            }
            Direction::South => {
                self.icon = direction_char(Direction::West);
                self.direction = Direction::West;
            }
            Direction::East => {
                self.icon = direction_char(Direction::South);
                self.direction = Direction::South;
            }
            Direction::West => {
                self.icon = direction_char(Direction::North);
                self.direction = Direction::North;
            }
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

fn draw_screen(map: &Map) -> () {
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
    let mut _local_x_min = 0;
    let mut _local_x_max = 0;
    let mut _local_y_min = 0;
    let mut _local_y_max = 0;

    // Width greater than row len
    if config.width >= map_copy._x_max {
        _local_x_min = 0;
        _local_x_max = map_copy._x_max;
    }
    // Cursor too close to west border
    else if cursor_x <= config.width / 2 {
        _local_x_min = 0;
        _local_x_max = config.width;
    }
    // Cursor too close to east border
    else if cursor_x + (config.width / 2) >= map_copy._x_max {
        _local_x_min = map_copy._x_max - config.width;
        _local_x_max = map_copy._x_max;
    }
    // Cursor safely inside width boundaries
    else {
        _local_x_min = cursor_x - (config.width / 2);
        _local_x_max = cursor_x + (config.width / 2);
    }

    // Height greater than col len
    if config.height >= map_copy._y_max {
        _local_y_min = 0;
        _local_y_max = map_copy._y_max;
    }
    // Cursor too close to north border
    else if cursor_y <= config.height / 2 {
        _local_y_min = 0;
        _local_y_max = config.height;
    }
    // Cursor too close to south border
    else if cursor_y + (config.height / 2) >= map_copy._y_max {
        _local_y_min = map_copy._y_max - config.height;
        _local_y_max = map_copy._y_max;
    }
    // Cursor safely inside height boundaries
    else {
        _local_y_min = cursor_y - (config.height / 2);
        _local_y_max = cursor_y + (config.height / 2);
    }

    // --- Draw Screen ---
    // Hacky clear screen
    print!("\x1B[2J\x1B[1;1H");

    // Draw map
    for row_vec in _local_y_min.._local_y_max {
        let row: String = map_copy.puzzle_map[row_vec][_local_x_min.._local_x_max]
            .iter()
            .collect();
        println!("{}", row);
    }

    // Sleep
    thread::sleep(config.pause_len);
}

fn part_1(input: &str) -> usize {
    let mut m = Map::new(input);
    let marker = 'x';

    while let Some(character) = m.peek_next() {
        draw_screen(&m);
        if character == '#' {
            m.cursor.rotate_clockwise();
        }
        m.puzzle_map[m.cursor.y][m.cursor.x] = marker;
        m.cursor.move_forward();
    }
    // Fix off by 1 error
    m.puzzle_map[m.cursor.y][m.cursor.x] = marker;
    draw_screen(&m);

    total_marker(&m.puzzle_map, &marker)
}

fn main() {
    let input: &str = include_str!("./day06.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}
