//A X - rock - 1
//B Y - paper - 2
//C Z -scissors - 3
//win - 6 - Z
//draw - 3 - Y
//lost - 0 - X

use std::fs;

fn calculate_score_part_two(path: impl Into<String>) -> i32 {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<&str> = content.split("\r\n").collect();
    print!("Lines: {:#?}", lines);
    let score = lines
        .iter()
        .map(|l| {
            let first = l.chars().into_iter().next().unwrap();
            let last = l.chars().into_iter().last().unwrap();
            let score = if last == 'Z' {
                if first == 'A' {
                    6 + 2
                } else if first == 'B' {
                    6 + 3
                } else if first == 'C' {
                    6 + 1
                } else {
                    panic!("!!!!")
                }
            } else if last == 'Y' {
                if first == 'A' {
                    3 + 1
                } else if first == 'B' {
                    3 + 2
                } else if first == 'C' {
                    3 + 3
                } else {
                    panic!("!!!!")
                }
            } else if last == 'X' {
                if first == 'A' {
                    0 + 3
                } else if first == 'B' {
                    0 + 1
                } else if first == 'C' {
                    0 + 2
                } else {
                    panic!("!!!!")
                }
            } else {
                panic!("!!!!!!!!")
            };
            score
        })
        .fold(0, |acc, x| acc + x);
    println!("Score: {}", score);
    score
}

fn calculate_score(path: impl Into<String>) -> i32 {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<&str> = content.split("\r\n").collect();
    print!("Lines: {:#?}", lines);
    println!("Lines size: {:#?}", lines.len());
    let score_for_game = lines
        .clone()
        .iter()
        .map(|e| {
            let first = e.chars().into_iter().next().unwrap();
            let last = e.chars().into_iter().last().unwrap();
            println!("First: {}, Last: {}", first, last);
            let score = if first == 'A' && last == 'X' {
                3
            } else if first == 'B' && last == 'Y' {
                3
            } else if first == 'C' && last == 'Z' {
                3
            } else if first == 'A' && last == 'Y' {
                6
            } else if first == 'A' && last == 'Z' {
                0
            } else if first == 'B' && last == 'X' {
                0
            } else if first == 'B' && last == 'Z' {
                6
            } else if first == 'C' && last == 'X' {
                6
            } else if first == 'C' && last == 'Y' {
                0
            } else {
                panic!("!!!!!!!!!!")
            };
            score
        })
        .fold(0, |acc, x| acc + x);
    println!("Socres for games: {}", score_for_game);
    let score_for_shape = lines
        .iter()
        .map(|e| {
            let last_char = e.chars().into_iter().last().unwrap();
            let score = match last_char {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => panic!("!!!!!1"),
            };
            score
        })
        .fold(0, |acc, x| acc + x);
    println!("Score fore shape: {}", score_for_shape);
    println!("Lines size: {:#?}", lines.len());
    score_for_game + score_for_shape
}

#[cfg(test)]
mod tests {
    use super::{calculate_score, calculate_score_part_two};

    #[test]
    fn calculate_score_test() {
        let score = calculate_score("resources/2-rps.txt");
        assert_eq!(14297, score);
    }

    #[test]
    fn calculate_score_part_two_test() {
        let score = calculate_score_part_two("resources/2-rps.txt");
        assert_eq!(10498, score);
    }
}
