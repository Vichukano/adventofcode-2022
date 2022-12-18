use std::fs;

pub fn read_file_lines(path: impl Into<String>) -> Vec<String> {
    let file_path = path.into();
    let content = fs::read_to_string(file_path.as_str())
        .expect(format!("Failed to read file from path: {}", file_path).as_str());
    let lines: Vec<String> = content.split("\r\n").map(|s| s.to_owned()).collect();
    lines
}

pub fn read_file_to_string(path: impl Into<String>) -> String {
    let file_path = path.into();
    fs::read_to_string(file_path).expect("Failed to read file")
}
