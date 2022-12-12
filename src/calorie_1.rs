use std::{cmp::Reverse, fs};

fn max_calorie_counting(path: impl Into<String>) -> u32 {
    let total = total_calories_by_elf(path);
    let max = total.iter().max().unwrap().clone();
    max
}

fn total_three_calorie_counting(path: impl Into<String>) -> u32 {
    let mut total = total_calories_by_elf(path);
    total.sort_by_key(|v| Reverse(*v));
    let total_three = total.iter().take(3).fold(0, |acc, x| acc + x);
    total_three
}

fn total_calories_by_elf(path: impl Into<String>) -> Vec<u32> {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<String> = content
        .split("\r\n\r\n")
        .map(|l| l.replace("\r\n", ","))
        .collect();
    let total: Vec<u32> = lines
        .iter()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect();
    total
}

#[cfg(test)]
mod tests {
    use crate::calorie_1::{max_calorie_counting, total_three_calorie_counting};

    #[test]
    fn max_calorie_counting_test() {
        let max_calories = max_calorie_counting("resources/1-calorie-counting.txt");
        assert_eq!(72718, max_calories);
    }

    #[test]
    fn total_calories_by_elf_test() {
        let total_three = total_three_calorie_counting("resources/1-calorie-counting.txt");
        assert_eq!(213089, total_three);
    }
}
