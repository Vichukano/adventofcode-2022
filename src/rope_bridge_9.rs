#![allow(dead_code)]

use std::fs;

struct Rope {
    head: Head,
    tail: Tail,
}

struct Tail {
    position: Position,
}

impl Tail {
    fn new() -> Self {
        Tail {
            position: Position { x: 0, y: 0 },
        }
    }

    fn change_position(&mut self, position: Position) {
        self.position = position;
    }
}

struct Head {
    position: Position,
    prevs: Vec<Position>,
}

impl Head {
    fn new() -> Self {
        Head {
            position: Position { x: 0, y: 0 },
            prevs: Vec::new(),
        }
    }

    fn change_position(&mut self, position: Position) {
        let current = self.position;
        self.prevs.push(current);
        self.position = position;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Up(i32),
    Left(i32),
    Down(i32),
    Right(i32),
}

fn moves_count() -> u64 {
    let input = fs::read_to_string("resources/9-rope-bridge.txt").unwrap();
    let mut input: Vec<String> = input.split("\n").map(|s| s.trim().to_owned()).collect();
    input.pop();
    let moves = map_to_moves(input);
    let tail = Tail::new();
    let head = Head::new();
    let mut rope = Rope { tail, head };
    for m in moves.iter() {
        let mv = m.to_owned();
        match mv {
            Move::Up(x) => {
                let current_position = rope.head.position;
                let new_position = Position {
                    x: current_position.x,
                    y: current_position.y + x,
                };
                rope.head.change_position(new_position);
            }
            Move::Left(x) => {
                let current_position = rope.head.position;
                let new_position = Position {
                    x: current_position.x - x,
                    y: current_position.y,
                };
                rope.head.change_position(new_position);
            }
            Move::Down(x) => {
                let current_position = rope.head.position;
                let new_position = Position {
                    x: current_position.x,
                    y: current_position.y - x,
                };
                rope.head.change_position(new_position);
            }
            Move::Right(x) => {
                let current_position = rope.head.position;
                let new_position = Position {
                    x: current_position.x + x,
                    y: current_position.y,
                };
                rope.head.change_position(new_position);
            }
        }
    }
    let count = rope.head.prevs.len();
    println!("Count of moves: {}", count);
    count.try_into().unwrap()
}

fn calculate_tail_position(head_new_pos: &Position, tail_cur_pos: &Position) -> Position {
    let mut tail_pos = Position {
        x: tail_cur_pos.x,
        y: tail_cur_pos.y,
    };
    let delta_x = head_new_pos.x - tail_cur_pos.x;
    let delta_y = head_new_pos.y - tail_cur_pos.y;
    let need_change_x = delta_x > 1 || delta_x < -1;
    let need_change_y = delta_y > 1 || delta_y < -1;
    if need_change_x {
        if head_new_pos.x > 0 {
            tail_pos.x = head_new_pos.x - 1;
        } else {
            tail_pos.x -= head_new_pos.x + 1;
        }
        let is_on_the_line = head_new_pos.y == tail_cur_pos.y;
        if !is_on_the_line {
            tail_pos.y = head_new_pos.y;
        }
    }
    if need_change_y {
        if head_new_pos.y > 0 {
            tail_pos.y = head_new_pos.y - 1;
        } else {
            tail_pos.y = head_new_pos.y + 1;
        }
        let is_on_the_line = head_new_pos.x == tail_cur_pos.x;
        if !is_on_the_line {
            tail_pos.x = head_new_pos.x;
        }
    }
    tail_pos
}

fn map_to_moves(input: Vec<String>) -> Vec<Move> {
    let moves: Vec<Move> = input
        .iter()
        .map(|l| {
            let split: Vec<String> = l.split(" ").map(|s| s.trim().to_owned()).collect();
            let direction: &str = split.get(0).unwrap();
            let count = split.get(1).unwrap();
            let count: i32 = count.parse::<i32>().unwrap();
            let direction = match direction {
                "L" => Move::Left(count),
                "U" => Move::Up(count),
                "D" => Move::Down(count),
                "R" => Move::Right(count),
                _ => panic!("Unknown move!"),
            };
            direction
        })
        .collect();
    moves
}

#[cfg(test)]
mod tests {

    use crate::rope_bridge_9::calculate_tail_position;
    use crate::rope_bridge_9::map_to_moves;
    use crate::rope_bridge_9::moves_count;

    use super::Move;
    use super::Position;

    #[test]
    fn map_to_moves_test() {
        let input = vec![
            "U 2".to_owned(),
            "D 3".to_owned(),
            "L 1".to_owned(),
            "R 4".to_owned(),
        ];
        let result = map_to_moves(input);
        assert_eq!(&Move::Up(2), result.get(0).unwrap());
        assert_eq!(&Move::Down(3), result.get(1).unwrap());
        assert_eq!(&Move::Left(1), result.get(2).unwrap());
        assert_eq!(&Move::Right(4), result.get(3).unwrap());
    }

    #[test]
    fn moves_count_test() {
        let count = moves_count();
        assert_eq!(2000, count);
    }

    #[test]
    fn calculate_tail_position_test() {
        let head_position = Position::new(0, 0);
        let tail_position = Position::new(0, 0);
        let new_tail_position = calculate_tail_position(&head_position, &tail_position);
        assert_eq!(Position::new(0, 0), new_tail_position);
        let head_position = Position::new(0, 1);
        let tail_position = Position::new(0, 0);
        let new_tail_position = calculate_tail_position(&head_position, &tail_position);
        assert_eq!(Position::new(0, 0), new_tail_position);
        let head_position = Position::new(1, 0);
        let tail_position = Position::new(0, 0);
        let new_tail_position = calculate_tail_position(&head_position, &tail_position);
        assert_eq!(Position::new(0, 0), new_tail_position);
        let head_position = Position::new(1, 1);
        let tail_position = Position::new(0, 0);
        let new_tail_position = calculate_tail_position(&head_position, &tail_position);
        assert_eq!(Position::new(0, 0), new_tail_position);
        let head_position = Position::new(1, 2);
        let tail_position = Position::new(0, 0);
        let new_tail_position = calculate_tail_position(&head_position, &tail_position);
        assert_eq!(Position::new(1, 1), new_tail_position);
        let head_position = Position::new(-1, -2);
        let tail_position = Position::new(0, 0);
        let new_tail_position = calculate_tail_position(&head_position, &tail_position);
        assert_eq!(Position::new(-1, -1), new_tail_position);
        let head_position = Position::new(3, 5);
        let tail_position = Position::new(2, 0);
        let new_tail_position = calculate_tail_position(&head_position, &tail_position);
        assert_eq!(Position::new(3, 4), new_tail_position);
    }
}
