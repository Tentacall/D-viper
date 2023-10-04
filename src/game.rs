use crate::snake::Snake;
use crate::utils::{Direction, Position};
use ncurses::{addstr, attroff, attron, mvprintw, A_BOLD, COLOR_PAIR};
use rand::Rng;

pub struct Game {
    username: String,
    pub score: i32,
    pub snake: Snake,
    food: Food,
}

struct Food {
    value: i32,
    icon: String,
    pos: Position,
}

impl Food {
    fn new(value: i32) -> Self {
        Food {
            value,
            icon: "$".to_string(),
            pos: Position::new(12, 10),
        }
    }

    fn display(&self) {
        attron(COLOR_PAIR(1) | A_BOLD());
        mvprintw(self.pos.posy, self.pos.posx, self.icon.as_str());
        attroff(COLOR_PAIR(1) | A_BOLD());
    }

    fn relocate(&mut self, width: i32, height: i32) {
        let mut rng = rand::thread_rng();
        self.pos.posy = rng.gen_range(1..=height - 2);
        self.pos.posx = rng.gen_range(1..=width - 2);
    }
}




impl Game {
    pub fn new(name: String, width: i32, height: i32) -> Self {
        let textures = ("@".to_string(), "O".to_string(), "o".to_string());
        let position = Position::new(width / 2, height / 2);
        Game {
            username: name,
            score: 0,
            snake: Snake::new(position, textures),
            food: Food::new(5),
        }
    }

    pub fn update_score(&mut self, width: i32, height: i32) {
        if let Some(head) = self.snake.snake.front_mut() {
            if head.pos == self.food.pos {
                self.score += self.food.value;
                self.food.relocate(width, height);
                self.snake.extend_back();
            }
        }
    }

    pub fn control_snake(&mut self, ch: i32) {
        match ch {
            119 | 259 => match self.snake.direction {
                Direction::BOTTOM => {}
                _ => {
                    self.snake.direction = Direction::TOP;
                    self.snake.speed = 4;
                }
            }, // w
            97 | 260 => match self.snake.direction {
                Direction::RIGHT => {}
                _ => {
                    self.snake.direction = Direction::LEFT;
                    self.snake.speed = 4;
                }
            }, // A
            115 | 258 => match self.snake.direction {
                Direction::TOP => {}
                _ => {
                    self.snake.direction = Direction::BOTTOM;
                    self.snake.speed = 4;
                }
            }, //s
            100 | 261 => match self.snake.direction {
                Direction::LEFT => {}
                _ => {
                    self.snake.direction = Direction::RIGHT;
                    self.snake.speed = 4;
                }
            }, // D
            _ => {}
        }
    }

    pub fn display(&mut self) {
        let data = format!("USER : {} | SCORE : {}", self.username, self.score);
        addstr(&data);
        self.snake.display();
        self.food.display();
    }
}
