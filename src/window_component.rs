use ncurses::{wattroff, wattron, box_, delwin, mvwprintw, newwin, wrefresh, A_BOLD, COLOR_PAIR, KEY_ENTER, wclear };
use crate::utils::Position;


pub struct Choice
{
    text: String,
    handler: Box<dyn FnMut() -> ()>,
}


pub struct Component {
    win: *mut i8,
    height: i32,
    width: i32,
    cur_y : i32,
    options: Vec<Choice>,
    option_len: usize,
    option_selected: usize,
    title :String,
    title_pos: Position
}

impl Choice{
    pub fn new(text: String, handler: Box<dyn FnMut() -> ()>) -> Self {
        Choice {
            text: text,
            handler: handler,
        }
    }

    fn call_handler(&mut self) -> () {
        (self.handler)()
    }
}

impl Component {
    pub fn new(height: i32, width: i32, window_height: i32, window_width: i32) -> Self {
        let posy = (window_height - height) / 2;
        let posx = (window_width - width) / 2;
        Component {
            win: newwin(height, width, posy, posx),
            height: height,
            width: width,
            cur_y: 1,
            options: Vec::new(),
            option_len: 0,
            option_selected: 0,
            title : String::new(),
            title_pos : Position::new(0, 0),
        }
    }

    pub fn set_border(&self) {
        box_(self.win, 0, 0);
    }

    pub fn set_title(&mut self, title: String) {
        let p = (self.width - (title.len() as i32)) / 2;
        self.title = title;
        self.title_pos.posx = p;
        self.title_pos.posy = 1;
    }

    pub fn add_choice(&mut self, choice : Choice) {
        self.options.push(choice);
        self.option_len += 1;
        if self.option_len as i32 == self.height {
            self.height += 2;
        }
        if self.option_len == 1 {
            self.option_selected = 0;
        }
    }

    pub fn display(&mut self) {
        wattron(self.win,COLOR_PAIR(1) | A_BOLD());
        mvwprintw(self.win, self.title_pos.posy, self.title_pos.posx, self.title.as_str());
        wattroff(self.win,COLOR_PAIR(1) | A_BOLD());
        self.cur_y = 2;

        if self.option_len == 0 { return }
        for i in 0..self.option_len {
            let p = (self.width - (self.options[i].text.len() as i32)) / 2;
            if i == self.option_selected {
                wattron(self.win, A_BOLD());
                mvwprintw(self.win, self.cur_y, p, self.options[i].text.as_str());
                wattroff(self.win, A_BOLD());
            }
            else{
                mvwprintw(self.win, self.cur_y, p, self.options[i].text.as_str());
            }
            self.cur_y += 1;
        }
    }

    pub fn handle_input(&mut self, x: i32) -> () {
        match x {
            115 | 258 => {
                self.option_selected = ( self.option_selected + 1 ) % self.option_len;
            },
            119 | 259 => {
                if self.option_selected == 0 {
                    self.option_selected = self.option_len - 1;
                }
                else {
                    self.option_selected = ( self.option_selected - 1 ) % self.option_len;
                }
            },
            KEY_ENTER => {
                self.options[self.option_selected].call_handler();
            },
            _ => {}
        }
    }

    pub fn del(self) {
        delwin(self.win);
    }

    pub fn refresh(&self) {
        wrefresh(self.win);
    }

    pub fn clear(&self) {
        wclear(self.win);
    }

}
