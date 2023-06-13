extern crate ncurses;
use ncurses::*;

#[derive(Debug)]
pub struct GameWindow {
    pub window_width: i32,
    pub window_height: i32,
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

        GameWindow {
            window_width: max_x,
            window_height: max_y,
        }
    }

    pub fn exit(self) {
        endwin();
    }

    pub fn new_menu(&mut self, height: i32, width: i32, title: String) -> () {
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