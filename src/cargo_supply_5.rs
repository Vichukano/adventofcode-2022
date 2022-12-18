use std::{collections::HashMap, fs};

use itertools::Itertools;

pub fn read_file(path: impl Into<String>) -> Vec<String> {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<String> = content.split("\r\n").map(|s| s.to_owned()).collect();
    lines
}

fn create_stack(values: impl Into<String>) -> Vec<char> {
    let values: String = values.into();
    let stack = values.chars().into_iter().collect::<Vec<char>>();
    println!("Stack: {:#?}", stack);
    stack
}

fn create_stack_map() -> HashMap<u32, Vec<char>> {
    let one = create_stack("ZJG");
    let two = create_stack("QLRPWFVC");
    let three = create_stack("FPMCLGR");
    let four = create_stack("LFBWPHM");
    let five = create_stack("GCFSVQ");
    let six = create_stack("WHJZMQTL");
    let seven = create_stack("HFSBV");
    let eight = create_stack("FJZS");
    let nine = create_stack("MCDPFHBT");
    let mut map = HashMap::new();
    map.insert(1, one);
    map.insert(2, two);
    map.insert(3, three);
    map.insert(4, four);
    map.insert(5, five);
    map.insert(6, six);
    map.insert(7, seven);
    map.insert(8, eight);
    map.insert(9, nine);
    map
}

fn move_stacks(from: &mut Vec<char>, to: &mut Vec<char>, count: u32) {
    println!("Before move");
    println!("From: {:#?}", from);
    println!("To: {:#?}", to);
    println!("Count: {}", count);
    for _ in 0..count {
        let val = from.pop().unwrap();
        to.push(val);
    }
    println!("After move");
    println!("From: {:#?}", from);
    println!("To: {:#?}", to);
}

fn move_stacks_9001(from: &mut Vec<char>, to: &mut Vec<char>, count: u32) {
    println!("Before move");
    println!("From: {:#?}", from);
    println!("To: {:#?}", to);
    println!("Count: {}", count);
    let mut tmp = Vec::new();    
    for _ in 0..count {
        let val = from.pop().unwrap();
        tmp.push(val);
    }
    tmp.reverse();
    to.append(&mut tmp);
    println!("After move");
    println!("From: {:#?}", from);
    println!("To: {:#?}", to);
}

fn crates_on_top() -> String {
    let lines = read_file("resources/5-cargo-supply.txt");
    let formatted: Vec<String> = lines
        .iter()
        .map(|s| {
            s.replace("move", "")
                .replace("from", ",")
                .replace("to", ",")
        })
        .map(|s| s.to_owned())
        .map(|s| s.replace(" ", ""))
        .collect();
    println!("Formatted: {:#?}", formatted);
    let digits: Vec<Vec<u32>> = formatted
        .iter()
        .map(|s| {
            s.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let mut stack_map: HashMap<u32, Vec<char>> = create_stack_map();
    for d in digits {
        let count = d.get(0).unwrap();
        let from = d.get(1).unwrap();
        let to = d.get(2).unwrap();
        let mut f = stack_map.remove(from).unwrap();
        let mut t = stack_map.remove(to).unwrap();
        move_stacks_9001(&mut f, &mut t, *count);
        stack_map.insert(from.to_owned(), f);
        stack_map.insert(to.to_owned(), t);
    }
    print!("Map after: {:#?}", stack_map);
    let mut result = String::new();
    for key in stack_map.keys().sorted() {
        result.push(stack_map.get(key).unwrap().last().unwrap().to_owned())
    }
    println!("Result: {}", result);
    result
}

mod tests {
    use crate::cargo_supply_5::{crates_on_top, create_stack};

    #[test]
    fn create_stack_test() {
        let mut stack = create_stack("ZJGPWFVC");
        let top = stack.pop();
        assert_eq!(top, Some('C'));
    }

    #[test]
    fn crates_on_top_test_2() {
        let top = crates_on_top();
        assert_eq!(top, "SFTMRHPP");
    }
}
