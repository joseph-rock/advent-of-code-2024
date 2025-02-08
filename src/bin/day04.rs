struct Wordsearch {
    matrix: Vec<Vec<char>>,
}

impl Wordsearch {
    fn new(input: &str) -> Wordsearch {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            matrix.push(chars);
        }

        Wordsearch { matrix }
    }

    fn slice(&self, x: usize, y: usize, x_dir: i32, y_dir: i32, len: usize) -> Option<String> {
        let mut slice = String::new();

        for offset in 0..len {
            let next_y = y as i32 + (offset as i32 * y_dir);
            let row = self.matrix.get(next_y as usize);
            match row {
                Some(row) => {
                    let next_x = x as i32 + (offset as i32 * x_dir);
                    let c = row.get(next_x as usize);
                    match c {
                        Some(c) => slice.push(*c),
                        None => return None,
                    }
                }
                None => return None,
            }
        }
        Some(slice)
    }

    fn slice_east(&self, x: usize, y: usize, len: usize) -> Option<String> {
        self.slice(x, y, 1, 0, len)
    }

    fn slice_southeast(&self, x: usize, y: usize, len: usize) -> Option<String> {
        self.slice(x, y, 1, 1, len)
    }

    fn slice_south(&self, x: usize, y: usize, len: usize) -> Option<String> {
        self.slice(x, y, 0, 1, len)
    }

    fn slice_southwest(&self, x: usize, y: usize, len: usize) -> Option<String> {
        self.slice(x, y, -1, 1, len)
    }

    fn search_word(&self, x: usize, y: usize, search_word: &str) -> usize {
        let rev_search_word: String = search_word.chars().rev().collect();
        let mut matches = 0;
        let mut match_word = |word: &str| {
            if word == search_word || word == rev_search_word {
                matches += 1;
            }
        };

        let east = self.slice_east(x, y, search_word.len());
        match east {
            Some(word) => match_word(&word),
            None => (),
        }
        let south_east = self.slice_southeast(x, y, search_word.len());
        match south_east {
            Some(word) => match_word(&word),
            None => (),
        }
        let south = self.slice_south(x, y, search_word.len());
        match south {
            Some(word) => match_word(&word),
            None => (),
        }
        let south_west = self.slice_southwest(x, y, search_word.len());
        match south_west {
            Some(word) => match_word(&word),
            None => (),
        }

        matches
    }

    fn x_max(&self) -> usize {
        self.matrix[0].len()
    }

    fn y_max(&self) -> usize {
        self.matrix.len()
    }
}

fn main() {
    let input: &str = include_str!("./day04.txt");
    let pt1 = part_1(input);
    //let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    //println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let matrix = Wordsearch::new(input);
    let search_word = "XMAS";
    let mut total = 0;
    for yi in 0..matrix.y_max() {
        for xi in 0..matrix.x_max() {
            total += matrix.search_word(xi, yi, search_word);
        }
    }
    total
}
