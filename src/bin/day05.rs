use std::collections::HashMap;

struct PageOrdering {
    rules: HashMap<String, Vec<String>>,
    updates: Vec<String>,
}

fn main() {
    let input: &str = include_str!("./day05.txt");
    let pt1 = part_1(input);
    //    let pt2 = part_2(input);
    //    println!("Part 1: {pt1}");
    //    println!("Part 2: {pt2}");
}

fn part_1(input: &str) {
    let foo = parse(input);
    println!("{:?}", foo);
}

fn parse(input: &str) {
    let mut rules: HashMap<&str, &mut Vec<&str>> = HashMap::new();
    for line in input.lines() {
        if line.contains("|") {
            let rule_line: Vec<&str> = line.split("|").collect();
            let before = rule_line[0];
            let after = rule_line[1];
            let rule = rules.get(before);
            match rule {
                Some(vec) => {
                    vec.push(after);
                    rules.insert(before, vec);
                }
                None => {
                    let mut vec: Vec<&str> = Vec::new();
                    vec.push(after);
                    rules.insert(before, vec);
                }
            }
        }
    }
}
