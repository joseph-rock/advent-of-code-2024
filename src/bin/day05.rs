use std::collections::HashMap;

struct PrintQueue {
    rules: HashMap<usize, Vec<usize>>,
    lists: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> PrintQueue {
    let split_input: Vec<&str> = input.split("\n\n").collect();

    let raw_rules: Vec<&str> = split_input[0].lines().collect();
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    for r in raw_rules {
        let rule_split: Vec<&str> = r.split("|").collect();
        let before = rule_split[0].parse::<usize>().unwrap();
        let after = rule_split[1].parse::<usize>().unwrap();
        if let Some(rule) = rules.get_mut(&after) {
            rule.push(before);
        } else {
            rules.insert(after, vec![before]);
        }
    }

    let raw_lists: Vec<&str> = split_input[1].lines().collect();
    let lists: Vec<Vec<usize>> = raw_lists
        .into_iter()
        .map(|list| {
            list.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    PrintQueue { rules, lists }
}

fn middle_number(list: &Vec<usize>) -> Option<usize> {
    if list.is_empty() {
        None
    } else {
        let middle_index = list.len() / 2;
        list.get(middle_index).copied()
    }
}

fn ordered(list: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut check_list: Vec<usize> = vec![];

    for num in list {
        if check_list.contains(&num) {
            return false;
        } else {
            let num_rules = rules.get(&num).unwrap().clone();
            check_list.extend(num_rules.iter().cloned());
        }
    }

    true
}

fn part_1(input: &str) -> usize {
    let queue = parse_input(input);
    let mut total = 0;

    for list in queue.lists {
        if ordered(&list, &queue.rules) {
            total += middle_number(&list).unwrap();
        }
    }
    total
}

fn main() {
    let input: &str = include_str!("./day05.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}
