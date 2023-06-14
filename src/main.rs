use ncurses::*;
use viper::game::Game;
use viper::game_window::*;
use viper::window_component::Action;

fn main() {
    let mut window = GameWindow::new();
    'new_game: loop {
        let mut game: Game = Game::new(
            "Rupak".to_string(),
            window.window_width,
            window.window_height,
        );
        'curr_game: loop {
            'game: loop {
                clear();
                game.display();
                game.update_score(window.window_width, window.window_height);
                refresh();

                match game.snake.propagate() {
                    Ok(()) => {}
                    Err(_) => {
                        break 'game;
                    }
                };
                napms(100);
                match getch() {
                    ERR => {}
                    27 => break 'game,
                    n => game.control_snake(n),
                }
            }
            let msg: String = format!("Score : {}", game.score);
            match window.pause_menu(6, 30, msg) {
                Action::QUIT => break 'new_game,
                Action::RESTART => break 'curr_game,
                Action::RESUME => {}
            }
        }
    }
    window.exit();
}
