use ncurses::*;
use viper::game_window::*;
use viper::game::Game;

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
