use std::fs;

pub fn read_file(path: impl Into<String>) -> Vec<String> {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<String> = content.split("\r\n").map(|s| s.to_owned()).collect();
    lines
}

fn count_fully_ranged_pairs() -> u32 {
    let lines = read_file("resources/4-camp-cleanup.txt");
    println!("Lines: {:#?}", lines);
    let formatted: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.replace("-", ","))
        .map(|l| {
            l.split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    println!("Formatted: {:#?}", formatted);
    let mut intersection_count = 0;
    for x in formatted.iter() {
        let one = x.get(0).unwrap();
        let two = x.get(1).unwrap();
        let three = x.get(2).unwrap();
        let four = x.get(3).unwrap();
        if one <= three && two >= four {
            intersection_count += 1;
        } else if one >= three && two <= four {
            intersection_count += 1;
        }
    }
    println!("Count of intersection: {}", intersection_count);
    intersection_count
}

fn count_of_overlapped() -> u32 {
    let lines = read_file("resources/4-camp-cleanup.txt");
    let formatted: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.replace("-", ","))
        .map(|l| {
            l.split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    println!("Formatted: {:#?}", formatted);
    let mut overlapped_count = 0;
    for x in formatted.iter() {
        let one = x.get(0).unwrap();
        let two = x.get(1).unwrap();
        let three = x.get(2).unwrap();
        let four = x.get(3).unwrap();
        if two < three {
        } else if one > four {
        } else {
            overlapped_count += 1;
        }
    }
    //2-4, 6-8
    //7-9, 3-6
    println!("Count of overlapped: {}", overlapped_count);
    overlapped_count
}

#[cfg(test)]
mod tests {
    use crate::camp_cleanup_4::{count_fully_ranged_pairs, count_of_overlapped};

    #[test]
    fn count_fully_ranged_pairs_test() {
        let count = count_fully_ranged_pairs();
        assert_eq!(547, count);
    }

    #[test]
    fn count_of_overlapped_test() {
        let count = count_of_overlapped();
        assert_eq!(843, count);
    }
}
