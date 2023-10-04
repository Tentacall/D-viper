use ncurses::{attroff, attron, mvprintw, A_BOLD};
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::utils::{Direction, Position};

#[derive(Clone, Copy, Debug)]
enum Part {
    HEAD = 0,
    BODY = 1,
    TAIL = 2,
}

#[derive(Debug)]
pub struct SnakeBodyPart {
    pub pos: Position,
    part: Part,
}

pub struct Snake {
    pub snake: VecDeque<SnakeBodyPart>,
    pub direction: Direction,
    pub speed: i32,
    texture: (String, String, String),
    snake_hash: HashSet<i32>,
}

impl SnakeBodyPart {
    fn new(position: Position, part: Part) -> Self {
        SnakeBodyPart {
            pos: position,
            part,
        }
    }

        let ch: String;
        match self.part {
            Part::HEAD => ch = texture.0.clone(),
            Part::BODY => ch = texture.1.clone(),
            Part::TAIL => ch = texture.2.clone(),
        }
    fn display(&self, texture: (String, String, String)) {
        mvprintw(self.pos.posy, self.pos.posx, ch.as_str());
    }
}


impl Snake {
    pub fn new(position: Position, texture: (String, String, String)) -> Self {
        let p1 = SnakeBodyPart::new(position, Part::HEAD);
        let p2 = SnakeBodyPart::new(position.next(Direction::RIGHT), Part::BODY);
        let p3 = SnakeBodyPart::new(p2.pos.next(Direction::RIGHT), Part::TAIL);
        let hash_set: HashSet<i32> = HashSet::from([p1.pos.hash(), p2.pos.hash(), p3.pos.hash()]);
        let snake: VecDeque<SnakeBodyPart> = VecDeque::from([p1, p2, p3]);
        Snake {
            snake,
            direction: Direction::RIGHT,
            texture,
            snake_hash: hash_set,
            speed: 1,
        }
    }

    fn extend_front(&mut self) -> Result<(), String> {
        if let Some(curr_head) = self.snake.front_mut() {
            curr_head.part = Part::BODY;
            let p: SnakeBodyPart =
                SnakeBodyPart::new(curr_head.pos.next(self.direction.opposite()), Part::HEAD);

            let msg: String = format!("hash = {}", p.pos.hash());
            if self.snake_hash.contains(&p.pos.hash()) {
                return Err(msg);
            } else {
                self.snake_hash.insert(p.pos.hash());
            }
            self.snake.push_front(p);
        }
        Ok(())
    }

    pub fn extend_back(&mut self) {
        if let Some(curr_tail) = self.snake.back_mut() {
            curr_tail.part = Part::BODY;
            let p: SnakeBodyPart =
                SnakeBodyPart::new(curr_tail.pos.next(self.direction), Part::HEAD);

            self.snake.push_back(p);
        }
    }

    pub fn propagate(&mut self) -> Result<(), String> {
        match self.extend_front() {
            Ok(()) => {}
            Err(error) => {
                return Err(error);
            }
        };

        if let Some(curr_tail) = self.snake.back_mut() {
            self.snake_hash.remove(&curr_tail.pos.hash());
        }

        self.snake.pop_back();

        if let Some(curr_tail) = self.snake.back_mut() {
            curr_tail.part = Part::TAIL;
        }
        Ok(())
    }

    pub fn display(&self) {
        attron(A_BOLD());
        for part in self.snake.iter() {
            part.display(self.texture.clone());
        }
        attroff(A_BOLD());
    }
}
