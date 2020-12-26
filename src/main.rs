use rand::{thread_rng, Rng};
use sfml::{
    graphics::{
        CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
        Transformable,
    },
    system::{Clock, Time, Vector2f},
    window::{ContextSettings, Event, Key, Style},
};
use std::f32::consts::PI;

mod snake;

use snake::{Direction, DrawableGame};

pub const window_width: i16 = 480;
pub const window_height: i16 = 480;

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
    let mut rng = thread_rng();

    // Define some constants

    let mut game = DrawableGame::new(16, 16);

    let paddle_size = Vector2f::new(25., 100.);
    let ball_radius = 10.;

    // Create the window of the application
    let context_settings = ContextSettings::default();
    let mut window = RenderWindow::new(
        (window_width as u32, window_height as u32),
        "SFML Pong",
        Style::CLOSE,
        &context_settings,
    );
    window.set_vertical_sync_enabled(true);

    // Define the paddles properties
    let mut ai_timer = Clock::start();
    let ai_time = Time::seconds(0.1);
    let paddle_speed = 400.;
    let mut right_paddle_speed = 0.;
    let ball_speed = 400.;
    let mut ball_angle = 0.;

    let mut clock = Clock::start();

    let mut timer = Clock::start();
    let time = Time::seconds(0.25);

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
        //let delta_time = clock.restart().as_seconds();

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
        /*
        // Move the player's paddle
        if Key::Up.is_pressed() && (left_paddle.position().y - paddle_size.y / 2. > 5.) {
            left_paddle.move_((0., -paddle_speed * delta_time));
        }
        if Key::Down.is_pressed()
            && (left_paddle.position().y + paddle_size.y / 2. < game_height as f32 - 5.)
        {
            left_paddle.move_((0., paddle_speed * delta_time));
        }

        // Update the computer's paddle direction according to the ball position
        if ai_timer.elapsed_time().as_microseconds() > ai_time.as_microseconds() {
            ai_timer.restart();
            if ball.position().y + ball_radius > right_paddle.position().y + paddle_size.y / 2. {
                right_paddle_speed = paddle_speed;
            } else if ball.position().y - ball_radius
                < right_paddle.position().y - paddle_size.y / 2.
            {
                right_paddle_speed = -paddle_speed;
            } else {
                right_paddle_speed = 0.;
            }
        }

        // Move the ball
        let factor = ball_speed * delta_time;
        ball.move_((ball_angle.cos() * factor, ball_angle.sin() * factor));

        // Check collisions between the ball and the screen
        if ball.position().x - ball_radius < 0. {
            is_playing = false;
            pause_message.set_string("You lost !\nPress space to restart or\nescape to exit");
        }
        if ball.position().x + ball_radius > game_width as f32 {
            is_playing = false;
            pause_message.set_string("You won !\nPress space to restart or\nescape to exit");
        }
        if ball.position().y - ball_radius < 0. {
            //ball_sound.play();
            ball_angle = -ball_angle;
            let p = ball.position().x;
            ball.set_position((p, ball_radius + 0.1));
        }
        if ball.position().y + ball_radius > game_height as f32 {
            //ball_sound.play();
            ball_angle = -ball_angle;
            let p = ball.position().x;
            ball.set_position((p, game_height as f32 - ball_radius - 0.1));
        }

        // Check the collisions between the ball and the paddles
        // Left Paddle
        let (ball_pos, paddle_pos) = (ball.position(), left_paddle.position());
        if ball_pos.x - ball_radius < paddle_pos.x + paddle_size.x / 2.
            && ball_pos.x - ball_radius > paddle_pos.x
            && ball_pos.y + ball_radius >= paddle_pos.y - paddle_size.y / 2.
            && ball_pos.y - ball_radius <= paddle_pos.y + paddle_size.y / 2.
        {
            if ball_pos.y > paddle_pos.y {
                ball_angle = PI - ball_angle + rng.gen_range(0..20) as f32 * PI / 180.;
            } else {
                ball_angle = PI - ball_angle - rng.gen_range(0..20) as f32 * PI / 180.;
            }

            //ball_sound.play();
            ball.set_position((
                paddle_pos.x + ball_radius + paddle_size.x / 2. + 0.1,
                ball_pos.y,
            ));
        }

        // Right Paddle
        let (ball_pos, paddle_pos) = (ball.position(), right_paddle.position());
        if ball_pos.x + ball_radius > paddle_pos.x - paddle_size.x / 2.
            && ball_pos.x + ball_radius < paddle_pos.x
            && ball_pos.y + ball_radius >= paddle_pos.y - paddle_size.y / 2.
            && ball_pos.y - ball_radius <= paddle_pos.y + paddle_size.y / 2.
        {
            if ball_pos.y > paddle_pos.y {
                ball_angle = PI - ball_angle + rng.gen_range(0..20) as f32 * PI / 180.;
            } else {
                ball_angle = PI - ball_angle - rng.gen_range(0..20) as f32 * PI / 180.;
            }

            //ball_sound.play();
            ball.set_position((
                paddle_pos.x - ball_radius - paddle_size.x / 2. - 0.1,
                ball_pos.y,
            ));
        }

        // Clear the window
        window.clear(Color::rgb(50, 50, 50));
        game.draw(&window);
        // Display things on screen
        window.display()
        */
    }
}
