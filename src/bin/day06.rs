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

#[derive(Debug)]
struct Cursor {
    cursor: char,
    coordinate: Coordinate,
    direction: Direction,
}

impl Cursor {
    fn find_start(puzzle_map: &Vec<Vec<char>>) -> Option<Cursor> {
        for (y, row) in puzzle_map.iter().enumerate() {
            for (x, character) in row.iter().enumerate() {
                if let Some(direction) = Direction::char_direction(character) {
                    return Some(Cursor {
                        cursor: *character,
                        coordinate: Coordinate { x, y },
                        direction,
                    });
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
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
            'v' | 'V' => Some(Direction::South),
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            _ => None,
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut m = Map::new(input);
    println!("{:?}", m.cursor.coordinate);
    0
}

fn main() {
    let input: &str = include_str!("./day06.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}
