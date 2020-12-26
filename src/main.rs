use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    system::{Clock, Time},
    window::{ContextSettings, Event, Key, Style},
};

mod snake;

use snake::{Direction, DrawableGame};

pub const WINDOW_WIDTH: i16 = 480;
pub const WINDOW_HEIGHT: i16 = 480;

fn draw_text(text: &str, pos_x: f32, pos_y: f32) {
    let font = Font::from_file("resources/Dearest.ttf").unwrap();
    let mut pause_message = Text::default();
    pause_message.set_font(&font);
    pause_message.set_character_size(40);
    pause_message.set_position((pos_x, pos_y));
    pause_message.set_fill_color(Color::WHITE);
    pause_message.set_string(text);
}

fn main() {
    // Define some constants

    let mut game = DrawableGame::new(16, 16);

    // Create the window of the application
    let context_settings = ContextSettings::default();
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        "SFML Pong",
        Style::CLOSE,
        &context_settings,
    );
    window.set_vertical_sync_enabled(true);

    let mut timer = Clock::start();
    let time = Time::seconds(0.15);

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        if Key::Up.is_pressed() {
            game.update_direction(Direction::Up);
        }
        if Key::Down.is_pressed() {
            game.update_direction(Direction::Down);
        }
        if Key::Left.is_pressed() {
            game.update_direction(Direction::Left);
        }
        if Key::Right.is_pressed() {
            game.update_direction(Direction::Right);
        }

        if timer.elapsed_time().as_microseconds() > time.as_microseconds() {
            timer.restart();
            game.move_snake();
            if game.is_over() {
                game = DrawableGame::new(16, 16);
            }
        }

        // Clear the window
        window.clear(Color::rgb(50, 50, 50));
        game.draw(&mut window);
        // Display things on screen
        window.display();
    }
}
