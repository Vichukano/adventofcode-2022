#![allow(dead_code)]
use std::{collections::HashSet, f32::consts::E};

use crate::utils;

//30373
//25512
//65332
//33549
//35390
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Tree {
    x: u32,
    y: u32,
    height: u32,
}

impl Tree {
    fn new(height: u32, x: u32, y: u32) -> Self {
        Tree { height, x, y }
    }
}

fn solve_visible(input: &str) -> usize {
    let lines = utils::read_file_lines(input);
    let mut visible: HashSet<Tree> = HashSet::new();
    let trees_horizon = raw_to_trees(&lines);
    let trees_vertix = raw_to_trees_vert(&lines);
    for vec in trees_horizon.iter() {
        count_visible_for_tree(vec, &mut visible);
        let vec_rev: Vec<Tree> = vec.iter().rev().map(|e| e.to_owned()).collect();
        count_visible_for_tree(&vec_rev, &mut visible);
    }
    println!("Vertical visible count: {}", visible.len());
    for vec in trees_vertix.iter() {
        count_visible_for_tree(vec, &mut visible);
        let vec_rev: Vec<Tree> = vec.iter().rev().map(|e| e.to_owned()).collect();
        count_visible_for_tree(&vec_rev, &mut visible);
    }
    println!("Horizon visible count: {}", visible.len());
    let total = visible.len();
    println!("TOTAL: {}", total);
    total
}

fn create_vectors_tree(size: u32) -> Vec<Vec<Tree>> {
    let mut root = Vec::new();
    for _ in 0..size {
        let inner: Vec<Tree> = Vec::new();
        root.push(inner);
    }
    root
}

fn count_visible_for_tree(line: &Vec<Tree>, visible: &mut HashSet<Tree>) {
    'outer: for i in line.iter().enumerate() {
        let position: u32 = i.0.try_into().unwrap();
        let element = i.1.to_owned();
        let mut score = 0;
        for idx in 0..position {
            let idx: usize = idx.try_into().unwrap();
            let inner: &Tree = line.get(idx).unwrap();
            //println!("Inner: {}, Element: {}", inner, element);
            score += 1;
            if element.height > inner.height {
                //println!("Element: {} > inner: {}", element, inner);
            } else {
                //println!("Element: {} not visible", element);
                continue 'outer;
            }
        }
        //println!("Element: {} visible in line!!!", element);
        visible.insert(element);
    }
}

fn raw_to_trees_vert(raw: &Vec<String>) -> Vec<Vec<Tree>> {
    let size: u32 = raw.get(0).unwrap().len().try_into().unwrap();
    let mut vec_tree = create_vectors_tree(size);
    for en in raw.iter().enumerate() {
        let y: u32 = en.0.try_into().unwrap();
        let line_dig: Vec<u32> =
            en.1.to_owned()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
        for inner in line_dig.iter().enumerate() {
            let x: u32 = inner.0.try_into().unwrap();
            let height = inner.1.to_owned();
            vec_tree
                .get_mut(inner.0)
                .unwrap()
                .push(Tree::new(height, x, y));
        }
    }
    vec_tree
}

fn raw_to_trees(raw: &Vec<String>) -> Vec<Vec<Tree>> {
    let mut trees = Vec::new();
    for en in raw.iter().enumerate() {
        let mut line: Vec<Tree> = Vec::new();
        let y: u32 = en.0.try_into().unwrap();
        let line_dig: Vec<u32> =
            en.1.to_owned()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
        for inner in line_dig.iter().enumerate() {
            let x: u32 = inner.0.try_into().unwrap();
            let height = inner.1.to_owned();
            line.push(Tree::new(height, x, y))
        }
        trees.push(line);
    }
    trees
}

mod tests {
    use std::collections::HashSet;

    use crate::treetop_tree_house_8::*;

    use super::raw_to_trees;

    #[test]
    fn raw_to_trees_test() {
        let input = vec!["12345".to_owned(), "67895".to_owned(), "54321".to_owned()];
        let trees = raw_to_trees(&input);
        assert_eq!(3, trees.len());
        let seven = trees.get(1).unwrap().get(1).unwrap();
        assert_eq!(7, seven.height);
        assert_eq!(1, seven.x);
        assert_eq!(1, seven.y);
        let last = trees.last().unwrap().last().unwrap();
        assert_eq!(1, last.height);
        assert_eq!(4, last.x);
        assert_eq!(2, last.y);
    }

    #[test]
    fn raw_to_trees_vert_test() {
        let input = vec!["12345".to_owned(), "67895".to_owned(), "54321".to_owned()];
        let trees = raw_to_trees_vert(&input);
        assert_eq!(5, trees.len());
        let seven = trees.get(1).unwrap().get(1).unwrap();
        assert_eq!(7, seven.height);
        assert_eq!(1, seven.x);
        assert_eq!(1, seven.y);
        let last = trees.last().unwrap().last().unwrap();
        assert_eq!(1, last.height);
        assert_eq!(4, last.x);
        assert_eq!(2, last.y);
    }

    #[test]
    fn count_visible_for_tree_test() {
        //2, 1, 2, 4, 5, 4, 9, 0, 3, 3, 4
        let mut visible: HashSet<Tree> = HashSet::new();
        let input = vec![
            Tree::new(2, 0, 0),
            Tree::new(1, 1, 0),
            Tree::new(2, 2, 0),
            Tree::new(4, 3, 0),
            Tree::new(5, 4, 0),
            Tree::new(4, 5, 0),
            Tree::new(9, 6, 0),
            Tree::new(0, 7, 0),
            Tree::new(3, 8, 0),
            Tree::new(3, 9, 0),
            Tree::new(4, 10, 0),
        ];
        count_visible_for_tree(&input, &mut visible);
        assert_eq!(4, visible.len());
        let input_rec: Vec<Tree> = input.iter().rev().map(|t| t.to_owned()).collect();
        count_visible_for_tree(&input_rec, &mut visible);
        assert_eq!(5, visible.len());
    }

    #[test]
    fn solve_visible_test() {
        let count = solve_visible("resources/8-treetop-tree-house.txt");
        assert_eq!(1827, count);
        let count = solve_visible("resources/tree-test.txt");
        assert_eq!(21, count);
    }
}
