use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn calculate_priority(path: impl Into<String>) -> u32 {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<&str> = content.split("\r\n").collect();
    print!("Lines: {:#?}", lines);
    let diff: Vec<char> = lines
        .iter()
        .map(|s| split_string_into_chunks(*s))
        .map(|ch| {
            let f: Vec<char> = ch.0.chars().collect();
            let s: Vec<char> = ch.1.chars().collect();
            let mut dif = '1';
            for c in s.iter() {
                if f.contains(c) {
                    dif = *c;
                    break;
                }
            }
            dif
        })
        .collect();
    print!("Diff: {:#?}", diff);
    let dic = dictionary();
    let score = diff
        .iter()
        .map(|c| dic.get(c).unwrap())
        .fold(0, |acc, x| acc + x);
    println!("total score: {}", score);
    score
}

fn score_second(path: impl Into<String>) -> u32 {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<&str> = content.split("\r\n").collect();
    let mut for_split = lines.into_iter().peekable();
    let mut chanks: Vec<Vec<&str>> = Vec::new();
    while for_split.peek().is_some() {
        let chunk: Vec<&str> = for_split.by_ref().take(3).collect();
        println!("{:?}", chunk);
        chanks.push(chunk);
    }
    let dic = dictionary();
    let score = chanks
        .iter()
        .map(|c| {
            let chunk = c;
            let f: HashSet<char> = String::from(*chunk.get(0).unwrap())
                .chars()
                .into_iter()
                .collect();
            let s: HashSet<char> = String::from(*chunk.get(1).unwrap())
                .chars()
                .into_iter()
                .collect();
            let t: HashSet<char> = String::from(*chunk.get(2).unwrap())
                .chars()
                .into_iter()
                .collect();
            let mut first_intersection: HashSet<char> = HashSet::new();
            let mut second_intersection: HashSet<char> = HashSet::new();
            for x in f.intersection(&s) {
                println!("Intersection 1: {}", x);
                first_intersection.insert(x.to_owned());
            }
            for y in f.intersection(&t) {
                println!("Intersection 2: {}", y);
                second_intersection.insert(y.to_owned());
            }
            let mut fin: Vec<char> = Vec::new();
            for z in first_intersection.intersection(&second_intersection) {
                println!("Intersection 3: {}", z);
                fin.push(z.to_owned());
            }
            let score = dic.get(fin.get(0).unwrap()).unwrap();
            score.to_owned()
        })
        .fold(0, |acc, x| acc + x);
    score
}

fn split_string_into_chunks(str: impl Into<String>) -> (String, String) {
    let s: String = str.into();
    let first: String = s.chars().into_iter().take(s.len() / 2).collect();
    let second: String = s.replace(&first, "");
    println!("First: {}, Second: {}", first, second);
    (first, second)
}

pub fn intersection(chars: Vec<Vec<char>>) -> Vec<char> {
    let mut intersect_result: Vec<char> = chars[0].clone();
    for temp_vec in chars {
        let unique_a: HashSet<char> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }
    println!("Intercection: {:#?}", intersect_result);
    intersect_result
}

fn dictionary() -> HashMap<char, u32> {
    let mut dic = HashMap::new();
    dic.insert('a', 1);
    dic.insert('b', 2);
    dic.insert('c', 3);
    dic.insert('d', 4);
    dic.insert('e', 5);
    dic.insert('f', 6);
    dic.insert('g', 7);
    dic.insert('h', 8);
    dic.insert('i', 9);
    dic.insert('j', 10);
    dic.insert('k', 11);
    dic.insert('l', 12);
    dic.insert('m', 13);
    dic.insert('n', 14);
    dic.insert('o', 15);
    dic.insert('p', 16);
    dic.insert('q', 17);
    dic.insert('r', 18);
    dic.insert('s', 19);
    dic.insert('t', 20);
    dic.insert('u', 21);
    dic.insert('v', 22);
    dic.insert('w', 23);
    dic.insert('x', 24);
    dic.insert('y', 25);
    dic.insert('z', 26);

    dic.insert('A', 27);
    dic.insert('B', 28);
    dic.insert('C', 29);
    dic.insert('D', 30);
    dic.insert('E', 31);
    dic.insert('F', 32);
    dic.insert('G', 33);
    dic.insert('H', 34);
    dic.insert('I', 35);
    dic.insert('J', 36);
    dic.insert('K', 37);
    dic.insert('L', 38);
    dic.insert('M', 39);
    dic.insert('N', 40);
    dic.insert('O', 41);
    dic.insert('P', 42);
    dic.insert('Q', 43);
    dic.insert('R', 44);
    dic.insert('S', 45);
    dic.insert('T', 46);
    dic.insert('U', 47);
    dic.insert('V', 48);
    dic.insert('W', 49);
    dic.insert('X', 50);
    dic.insert('Y', 51);
    dic.insert('Z', 52);
    dic
}

#[cfg(test)]
mod tests {
    use crate::rucksacks_3::score_second;

    use super::calculate_priority;

    #[test]
    fn calculate_priority_test() {
        let score = calculate_priority("resources/3-rucksacks.txt");
        assert_eq!(8039, score);
    }

    #[test]
    fn score_second_test() {
        let score = score_second("resources/3-rucksacks.txt");
        assert_eq!(2510, score);
    }
}
