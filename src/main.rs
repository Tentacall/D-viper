extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::Rng;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct GameWindow {
    window_width: i32,
    window_height: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    posx: i32,
    posy: i32,
}

#[derive(Clone, Copy, Debug)]
enum Part {
    HEAD = 0,
    BODY = 1,
    TAIL = 2,
}

#[derive(Clone, Copy)]
enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct SnakeBodyPart {
    pos: Position,
    part: Part,
}

#[allow(unused)]
struct Snake {
    snake: VecDeque<SnakeBodyPart>,
    pos: Position,
    direction: Direction,
    length: i32,
    texture: (String, String, String),
    snake_hash: HashSet<i32>,
}
struct Game {
    username: String,
    score: i32,
    snake: Snake,
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
            value: value,
            icon: "$".to_string(),
            pos: Position::new(12, 10),
        }
    }

    fn display(&self) -> () {
        attron(COLOR_PAIR(1) | A_BOLD());
        mvprintw(self.pos.posy, self.pos.posx, self.icon.as_str());
        attroff(COLOR_PAIR(1) | A_BOLD());
    }

    fn relocate(&mut self, width: i32, height: i32) -> () {
        let mut rng = rand::thread_rng();
        self.pos.posy = rng.gen_range(1..=height);
        self.pos.posx = rng.gen_range(1..=width);
    }
}

impl Direction {
    fn opposite(self) -> Direction {
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
    fn new(x: i32, y: i32) -> Self {
        Position { posx: x, posy: y }
    }

    fn next(&self, direction: Direction) -> Position {
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
    fn hash(&self) -> i32 {
        self.posx * 1000 + self.posy
    }
}

impl SnakeBodyPart {
    fn new(position: Position, part: Part) -> Self {
        SnakeBodyPart {
            pos: position,
            part: part,
        }
    }

    fn display(&self, texture: (String, String, String)) -> () {
        let ch: String;
        match self.part {
            Part::HEAD => ch = texture.0.clone(),
            Part::BODY => ch = texture.1.clone(),
            Part::TAIL => ch = texture.2.clone(),
        }
        mvprintw(self.pos.posy, self.pos.posx, ch.as_str());
    }
}

impl Snake {
    fn new(position: Position, texture: (String, String, String)) -> Self {
        let p1 = SnakeBodyPart::new(position, Part::HEAD);
        let p2 = SnakeBodyPart::new(position.next(Direction::RIGHT), Part::BODY);
        let p3 = SnakeBodyPart::new(p2.pos.next(Direction::RIGHT), Part::TAIL);
        let hash_set: HashSet<i32> = HashSet::from([p1.pos.hash(), p2.pos.hash(), p3.pos.hash()]);
        let snake: VecDeque<SnakeBodyPart> = VecDeque::from([p1, p2, p3]);
        Snake {
            snake: snake,
            pos: position,
            direction: Direction::RIGHT,
            length: 1,
            texture: texture,
            snake_hash: hash_set,
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

    fn extend_back(&mut self) -> () {
        if let Some(curr_tail) = self.snake.back_mut() {
            curr_tail.part = Part::BODY;
            let p: SnakeBodyPart =
                SnakeBodyPart::new(curr_tail.pos.next(self.direction), Part::HEAD);

            self.snake.push_back(p);
        }
    }

    fn propagate(&mut self) -> Result<(), String> {
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

    fn display(&self) -> () {
        attron(A_BOLD());
        for part in self.snake.iter() {
            part.display(self.texture.clone());
        }
        attroff(A_BOLD());
    }
}

impl GameWindow {
    fn new() -> Self {
        initscr();
        setlocale(LcCategory::all, "");
        start_color();

        use_default_colors();

        raw();
        keypad(stdscr(), true);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        timeout(0);
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        // defining color
        init_pair(1, COLOR_RED, COLOR_BLACK);

        GameWindow {
            window_width: max_x,
            window_height: max_y,
        }
    }

    fn exit(self) {
        endwin();
    }

    fn new_menu(&mut self, height: i32, width: i32, title: String) -> () {
        let posy = (self.window_height - height) / 2;
        let posx = (self.window_width - width) / 2;
        let win: *mut i8 = newwin(height, width, posy, posx);
        box_(win, 0, 0);

        let p = (width - (title.len() as i32)) / 2;
        mvwprintw(win, 1, p, title.as_str());
        wrefresh(win);
        napms(2000);
        getch();
        delwin(win);
    }
}

impl Game {
    fn new(name: String, width: i32, height: i32) -> Self {
        let textures = ("@".to_string(), "O".to_string(), "o".to_string());
        let position = Position::new(width / 2, height / 2);
        Game {
            username: name,
            score: 0,
            snake: Snake::new(position, textures),
            food: Food::new(5),
        }
    }

    fn update_score(&mut self, width: i32, height: i32) -> () {
        if let Some(head) = self.snake.snake.front_mut() {
            if head.pos == self.food.pos {
                self.score += self.food.value;
                self.food.relocate(width, height);
                self.snake.extend_back();
            }
        }
    }

    fn control_snake(&mut self, ch: i32) {
        match ch {
            119 | 259 => match self.snake.direction {
                Direction::BOTTOM => {}
                _ => self.snake.direction = Direction::TOP,
            }, // w
            97 | 260 => match self.snake.direction {
                Direction::RIGHT => {}
                _ => self.snake.direction = Direction::LEFT,
            }, // A
            115 | 258 => match self.snake.direction {
                Direction::TOP => {}
                _ => self.snake.direction = Direction::BOTTOM,
            }, //s
            100 | 261 => match self.snake.direction {
                Direction::LEFT => {}
                _ => self.snake.direction = Direction::RIGHT,
            }, // D
            _ => {}
        }
    }

    fn display(&mut self) -> () {
        let data = format!("USER : {} | SCORE : {}", self.username, self.score);
        addstr(&data);
        self.snake.display();
        self.food.display();
    }
}

fn main() {
    let mut window = GameWindow::new();
    let mut game: Game = Game::new(
        "Rupak".to_string(),
        window.window_width,
        window.window_height,
    );
    loop {
        clear();
        game.display();
        game.update_score(window.window_width, window.window_height);
        refresh();

        match game.snake.propagate() {
            Ok(()) => {}
            Err(_) => {
                break;
            }
        };
        napms(100);
        match getch() {
            ERR => {}
            27 => break,
            n => game.control_snake(n),
        }
    }
    let msg: String = format!("Score : {}", game.score);
    window.new_menu(6, 30, msg);
    window.exit();
    println!("Game Over\nScore : {}", game.score);
}
