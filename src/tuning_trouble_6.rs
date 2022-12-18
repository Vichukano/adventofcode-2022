use std::collections::{HashSet, VecDeque};

use crate::utils::read_file_to_string;

fn find_marker() -> usize {
    let content = read_file_to_string("resources/6-tuning-trouble.txt");
    let mut content: VecDeque<char> = content.chars().into_iter().collect();
    let mut sink: Vec<char> = Vec::new();
    println!("content len: {}", content.len());
    while !content.is_empty() {
        let chunk: HashSet<char> = content.iter().take(4).map(|c| c.to_owned()).collect();
        if chunk.len() == 4 {
            println!("Found!!!, chunk: {:#?}", chunk);
            break;
        } else {
            let first = content.pop_front().unwrap();
            sink.push(first);
        }
    }
    println!("sink len : {}", sink.len());
    sink.len() + 4
}

fn find_mesage() -> usize {
    let content = read_file_to_string("resources/6-tuning-trouble.txt");
    let mut content: VecDeque<char> = content.chars().into_iter().collect();
    let mut sink: Vec<char> = Vec::new();
    println!("content len: {}", content.len());
    while !content.is_empty() {
        let chunk: HashSet<char> = content.iter().take(14).map(|c| c.to_owned()).collect();
        if chunk.len() == 14 {
            println!("Found!!!, chunk: {:#?}", chunk);
            break;
        } else {
            let first = content.pop_front().unwrap();
            println!("pop char: {}, content len: {}", first, content.len());
            sink.push(first);
        }
    }
    println!("sink len : {}", sink.len());
    sink.len() + 14
}

mod tests {
    use crate::tuning_trouble_6::{find_marker, find_mesage};

    #[test]
    fn find_marker_test() {
        let score = find_marker();
        assert_eq!(1034, score);
    }

    #[test]
    fn find_mesage_test() {
        let score = find_mesage();
        assert_eq!(2472, score);
    }
}
