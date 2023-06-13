

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub posx: i32,
    pub posy: i32,
}

#[derive(Clone, Copy)]
pub enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}


impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::TOP => Direction::BOTTOM,
            Direction::BOTTOM => Direction::TOP,
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT,
        }
    }
}

impl Position {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Position { posx: x, posy: y }
    }

    pub fn next(&self, direction: Direction) -> Position {
        // Position { posx: self.posx, posy: self.posy - 1 }
        match direction {
            Direction::BOTTOM => Position {
                posx: self.posx,
                posy: self.posy - 1,
            },
            Direction::LEFT => Position {
                posx: self.posx + 1,
                posy: self.posy,
            },
            Direction::RIGHT => Position {
                posx: self.posx - 1,
                posy: self.posy,
            },
            Direction::TOP => Position {
                posx: self.posx,
                posy: self.posy + 1,
            },
        }
    }

    #[inline]
    pub fn hash(&self) -> i32 {
        self.posx * 1000 + self.posy
    }
}