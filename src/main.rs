use ncurses::*;
use viper::game::Game;
use viper::game_window::*;
use viper::window_component::Action;

fn main() {
    timeout(0);

    let mut window = GameWindow::new();
    'new_game: loop {
        if let Action::QUIT = window.start_menu(6, 30) {
            break 'new_game;
        }
        let name: String = window.get_name(6, 30);
        let mut game: Game = Game::new(name, window.window_width, window.window_height);
        'curr_game: loop {
            let mut is_gameover: bool = false;
            'game: loop {
                clear();
                game.display();
                game.update_score(window.window_width, window.window_height);
                refresh();

                match game.snake.propagate() {
                    Ok(()) => {}
                    Err(_) => {
                        is_gameover = true;
                        break 'game;
                    }
                };
                match getch() {
                    ERR => {
                        game.snake.speed = 1;
                    }
                    27 => break 'game,
                    n => game.control_snake(n),
                }
                let speed = 100 / game.snake.speed;
                napms(speed);
            }
            let msg: String = format!("Score : {}", game.score);
            match window.pause_menu(6, 30, msg, is_gameover) {
                Action::QUIT => break 'new_game,
                Action::RESTART => break 'curr_game,
                Action::RESUME => {}
                _ => {}
            }
        }
    }
    window.exit();
}

