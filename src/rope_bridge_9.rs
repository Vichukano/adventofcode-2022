#![allow(dead_code)]

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
}

impl Head {
    fn new() -> Self {
        Head {
            position: Position { x: 0, y: 0 },
        }
    }

    fn change_position(&mut self, position: Position) {
        self.position = position;
    }
}

struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }
}

enum Move {
    Up(u32),
    Left(u32),
    Down(u32),
    Right(u32),
}
