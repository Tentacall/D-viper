use ncurses::{wattroff, wattron, box_, delwin, mvwprintw, newwin, wrefresh, A_BOLD, COLOR_PAIR, KEY_ENTER, wclear };
use crate::utils::Position;

#[derive(Clone, Copy)]
pub enum Action{
    QUIT,
    RESUME,
    RESTART,
}

pub struct Choice
{
    text: String,
    handler: Action,
}


pub struct Component {
    win: *mut i8,
    height: i32,
    width: i32,
    cur_y : i32,
    options: Vec<Choice>,
    option_len: usize,
    option_selected: usize,
    inputs: String,
    input_len: usize,
    title :String,
    title_pos: Position
}

impl Choice{
    pub fn new(text: String, handler: Action) -> Self {
        Choice {
            text: text,
            handler: handler,
        }
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
            input_len: 0,
            inputs : String::new()
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
        // displaying the title
        wattron(self.win,COLOR_PAIR(1) | A_BOLD());
        mvwprintw(self.win, self.title_pos.posy, self.title_pos.posx, self.title.as_str());
        wattroff(self.win,COLOR_PAIR(1) | A_BOLD());
        self.cur_y = 2;

        //displaying inputs
        if self.input_len > 0 {
            self.cur_y += 1;
            // for i in 0..self.input_len {
            //     let p = (self.width - (self.inputs.len() as i32)) / 2;
            //     mvwprintw(self.win, self.cur_y, p, self.inputs.as_str());
            // }
            let p = (self.width - (self.inputs.len() as i32)) / 2;
            mvwprintw(self.win, self.cur_y, p, self.inputs.as_str());
        }


        // displaying the options
        if self.option_len > 0 { 
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
    }

    pub fn handle_input(&mut self, x: i32) -> Result<(), Action> {
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
            KEY_ENTER | 10 => {
                match self.option_handler(self.options[self.option_selected].handler){
                    Err(n) => return Err(n),
                    _ => {}
                };
            },
            _ => {}
        }
        Ok(())
    }

    pub fn set_inputbox(&mut self) -> i32 {
        self.input_len += 1;
        self.input_len as i32
    }

    pub fn update_input(&mut self, input: String, index: i32) -> () {
        let _ = index - 1;
        self.inputs = input;
    }

    fn option_handler(&mut self, action : Action ) -> Result<(), Action> {
        match action {
            Action::QUIT => { self.del(); return Err(Action::QUIT) },
            Action::RESTART => { self.del(); return Err(Action::RESTART) },
            Action::RESUME => { self.del(); return Err(Action::RESUME) },
        }
    }

    pub fn del(&self) {
        delwin(self.win);
    }

    pub fn refresh(&self) {
        wrefresh(self.win);
    }

    pub fn clear(&self) {
        wclear(self.win);
    }

}
