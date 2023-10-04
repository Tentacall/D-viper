extern crate ncurses;
use crate::window_component::*;
use ncurses::*;

pub struct GameWindow {
    pub window_width: i32,
    pub window_height: i32,
}

impl Default for GameWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl GameWindow {
    pub fn new() -> Self {
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
        init_pair(2, COLOR_YELLOW, COLOR_BLACK);

        GameWindow {
            window_width: max_x,
            window_height: max_y,
        }
    }

    pub fn exit(self) {
        endwin();
    }

    pub fn pause_menu(
        &mut self,
        height: i32,
        width: i32,
        title: String,
        is_gameover: bool,
    ) -> Action {
        let mut component: Component =
            Component::new(height, width, self.window_height, self.window_width);

        component.set_title(title);

        if !is_gameover {
            let choice1: Choice = Choice::new("Resume".to_string(), Action::RESUME);
            component.add_choice(choice1);
        }

        let choice2: Choice = Choice::new("Restart".to_string(), Action::RESTART);
        let choice3: Choice = Choice::new("Quit".to_string(), Action::QUIT);
        component.add_choice(choice2);
        component.add_choice(choice3);

        loop {
            component.clear();
            component.set_border();
            component.display();
            component.refresh();
            napms(100);
            match getch() {
                ERR => {}
                27 => {
                    component.del();
                    return Action::QUIT;
                }
                x => match component.handle_input(x) {
                    Err(n) => return n,
                    _ => {}
                },
            }
        }
    }

    pub fn start_menu(&mut self, height: i32, width: i32) -> Action {
        let mut component: Component =
            Component::new(height, width, self.window_height, self.window_width);

        component.set_title("VIPER".to_string());
        let choice1: Choice = Choice::new("Start".to_string(), Action::START);
        let choice2: Choice = Choice::new("Quit".to_string(), Action::QUIT);
        component.add_choice(choice1);
        component.add_choice(choice2);

        loop {
            component.clear();
            component.set_border();
            component.display();
            component.refresh();
            napms(100);
            match getch() {
                ERR => {}
                27 => {
                    component.del();
                    return Action::QUIT;
                }
                x => match component.handle_input(x) {
                    Err(n) => return n,
                    _ => {}
                },
            }
        }
    }

    pub fn get_name(&mut self, height: i32, width: i32) -> String {
        let title: String = String::from("You Name?");
        let mut component: Component =
            Component::new(height, width, self.window_height, self.window_width);

        component.set_title(title);
        let index: i32 = component.set_inputbox();
        let mut name: String = String::new();
        loop {
            component.clear();
            component.set_border();
            component.display();
            component.refresh();
            napms(100);
            match getch() {
                ERR => {}
                10 | 27 => break,
                c => {
                    if c == 263 && name.len() > 0 {
                        name.truncate(name.len() - 1);
                    }
                    else if ( c >= 48 && c <= 57) || ( c >= 65 && c <= 90 ) || ( c >= 97 && c <= 122 ){
                        let ch = char::from_u32(c as u32).unwrap();
                        name = format!("{}{}", name, ch);
                    }
                }
            };
            component.update_input(name.clone(), index);
        }
        component.del();
        name
    }
}
