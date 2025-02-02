use regex::Regex;

#[derive(Debug)]
struct Instruction {
    name: String,
    left: usize,
    right: usize,
}

fn main() {
    let input: &str = include_str!("./day03.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let re = Regex::new(
        r"(?x)
        (mul\()
        (?<left>\d+)
        (\,)
        (?<right>\d+)
        (\))",
    )
    .unwrap();

    let mut total: usize = 0;

    for line in input.lines() {
        let instructions: Vec<Instruction> = re.captures_iter(line).map(|inst| {
            let name = String::new();
            let left = inst.name("left").unwrap().as_str().parse::<usize>().unwrap();
            let right = inst.name("right").unwrap().as_str().parse::<usize>().unwrap();
            Instruction{
                name,
                left,
                right,
            }
        }).collect();

        total += instructions.into_iter().fold(0, |total, inst| total + (inst.left * inst.right));
    }

    total
}
fn part_2(input: &str) -> usize {
    let re = Regex::new(
        r"(?x)
        (?<name>[\w|\']+)
        (\()
        (?<left>\d*)
        ([\,]*)
        (?<right>\d*)
        (\))",
    )
    .unwrap();

    let mut total: usize = 0;
    let mut instruction_list: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let instructions: Vec<Instruction> = re.captures_iter(line).map(|inst| {
            let name = inst.name("name").unwrap().as_str();
            let left = inst.name("left").unwrap().as_str().parse::<usize>().unwrap_or(0);
            let right = inst.name("right").unwrap().as_str().parse::<usize>().unwrap_or(0);
            Instruction{
                name: name.to_string(),
                left,
                right,
            }
        }).collect();
        instruction_list.extend(instructions);
    }

    let mut flag = true;
    for inst in instruction_list {
        if inst.name.ends_with("do") {
            flag = true;
        }
        else if inst.name.ends_with("don't") {
            flag = false;
        }

        if flag && inst.name.ends_with("mul") {
            total += inst.left * inst.right;
        }
        
    }

    total
}
