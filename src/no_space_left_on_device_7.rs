use std::{
    cell::{Ref, RefCell},
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use crate::utils;

#[derive(Debug)]
enum Entity {
    Command(String),
    Dir(String),
    File(String, u64),
}
#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}
#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    child_dirs: HashMap<String, Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn files_size(&self) -> u64 {
        let size = self.files.iter().map(|f| f.size).fold(0, |acc, x| acc + x);
        size
    }
}

fn read_dir_tree(path: impl Into<String>) -> () {
    let path = path.into();
    let content = utils::read_file_lines(path);
    let entities: Vec<Entity> = content.iter().map(|c| entity(c)).collect();
    let mut current_dir = Rc::new(RefCell::new(Directory {
        name: "/".to_owned(),
        files: Vec::new(),
        child_dirs: HashMap::new(),
        parent: None,
    }));
    let root = Rc::clone(&current_dir);
    let mut all_dirs: HashMap<String, Rc<RefCell<Directory>>> = HashMap::new();
    for e in entities.iter() {
        println!("Start to process entity: {:#?}", e);
        match e {
            Entity::Command(c) => {
                let command = c.to_owned();
                if command == "ls" {
                    println!("Got ls command, skip");
                } else {
                    let split: Vec<String> = command.split(" ").map(|s| s.to_owned()).collect();
                    let first = split.get(0).unwrap().to_owned();
                    let second = split.get(1).unwrap().to_owned();
                    if first == "cd" {
                        let current = Rc::clone(&current_dir);
                        let current = current.borrow();
                        let current_name = current.name.clone();
                        println!("Got cd to: {}, current dir: {}", second, current_name);
                        if second == ".." {
                            let curr = Rc::clone(&current_dir);
                            let curr = curr.borrow();
                            let parrent = Rc::clone(curr.parent.as_ref().unwrap());
                            println!("Parrent dir: {:#?}", parrent.borrow().name);
                            current_dir = parrent;
                        } else {
                            let curr = Rc::clone(&current_dir);
                            let curr = curr.borrow();
                            let name = curr.name.clone();
                            let dirs_name: Vec<String> = curr
                                .child_dirs
                                .clone()
                                .iter()
                                .map(|e| e.0.to_owned())
                                .collect();
                            print!(
                                "Current dir with name: {}, child dirs: {:#?}",
                                name, dirs_name
                            );
                            let dir = curr.child_dirs.get(&second).unwrap();
                            current_dir = dir.to_owned();
                        }
                    } else {
                        println!("Got ls command: {}", command);
                    }
                }
            }
            Entity::Dir(name) => {
                let name = name.to_owned();
                let dir_name = name.clone();
                let dir_name_name = dir_name.clone();
                println!("Directory with name: {}", name);
                let dir = Directory {
                    name: name,
                    files: Vec::new(),
                    child_dirs: HashMap::new(),
                    parent: Some(Rc::clone(&current_dir)),
                };
                let dir_copy = Rc::new(RefCell::new(dir));
                let current = Rc::clone(&current_dir);
                current
                    .borrow_mut()
                    .child_dirs
                    .insert(dir_name, Rc::clone(&dir_copy));
                all_dirs.insert(dir_name_name, dir_copy);
            }
            Entity::File(name, size) => {
                let name = name.to_owned();
                let size = size.to_owned();
                println!("File with name: {} and size: {}", name, size);
                let file = File { name, size };
                let current = Rc::clone(&current_dir);
                current.borrow_mut().files.push(file);
            }
        }
    }
    let file_system = Rc::clone(&root);
    let file_system = file_system.borrow();
    let mut dir_size: HashMap<String, u64> = HashMap::new();
    dir_and_size(root, &mut dir_size);
    println!("DIR_SIZE: {:#?}", dir_size);
    let mut sum = 0;
    for d in all_dirs.iter() {
        let d = d.1.borrow();
        let size = d.files_size();
        if size <= 100000 {
            sum = sum + size;
        }
    }
    println!("SUM IS: {}", sum);
}

fn dir_and_size(root: Rc<RefCell<Directory>>, acc: &mut HashMap<String, u64>) {
    let childs = Rc::clone(&root);
    let childs = childs.borrow();
    let name = childs.name.clone();
    let size = childs.files_size();
    println!("Dir name: {}, size: {}", name, size);
    acc.insert(name, size);
    let childs: Vec<Rc<RefCell<Directory>>> =
        childs.child_dirs.values().map(|v| v.to_owned()).collect();
    for c in childs {
        dir_and_size(c, acc);
    }
}

fn calculate(dir_size: HashMap<String, u64>) -> u64 {
    let map = dir_size;
    let mut summ = 0;
    let sizes: Vec<u64> = map.iter().map(|m| m.1.to_owned()).collect();
    for s in sizes {
        if s <= 100000 {
            summ = summ + s;
        }
    }
    println!("Summ is: {}", summ);
    summ
}

fn entity(line: impl Into<String>) -> Entity {
    let line: String = line.into();
    let entity = if line.starts_with("$") {
        Entity::Command(line.replace("$", "").trim().to_owned())
    } else if line.starts_with("dir") {
        Entity::Dir(line.replace("dir", "").trim().to_owned())
    } else {
        let split = line
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let size = split
            .get(0)
            .map(|s| s.to_owned())
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let name = split.get(1).map(|s| s.to_owned()).unwrap();
        Entity::File(name, size)
    };
    entity
}

#[cfg(test)]
mod tests {
    use crate::no_space_left_on_device_7::read_dir_tree;

    #[test]
    fn read_dir_tree_test() {
        let _ = read_dir_tree("resources/7-no-space-left.txt");
        assert_eq!(14297, 14297);
    }
}
