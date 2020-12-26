use super::{WINDOW_HEIGHT, WINDOW_WIDTH};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
};
use std::collections::VecDeque;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    board: Board,
    snake: Snake,
    reward: Position,
    direction: Direction,
    score: u32,
    rng: ThreadRng,
}

impl Game {
    fn new(width: i16, height: i16) -> Game {
        let board = Board::new(width, height, vec![]);
        let snake = Snake::new();
        let mut game = Game {
            board,
            snake,
            reward: Position::new(width, height),
            direction: Direction::Right,
            score: 0,
            rng: thread_rng(),
        };
        game.position_new_reward();
        game
    }

    pub fn update_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn move_snake(&mut self) {
        let head_pos = self.snake.positions[0];
        // NOTE: View has inverted y axis
        // So down is +, up is -
        let new_head_pos = match self.direction {
            Direction::Up => Position {
                x: head_pos.x,
                y: head_pos.y - 1,
            },
            Direction::Down => Position {
                x: head_pos.x,
                y: head_pos.y + 1,
            },
            Direction::Left => Position {
                x: head_pos.x - 1,
                y: head_pos.y,
            },
            Direction::Right => Position {
                x: head_pos.x + 1,
                y: head_pos.y,
            },
        };
        if new_head_pos == self.reward {
            // snake ate an apple
            self.score += 1;
            self.snake.positions.push_front(new_head_pos);
            self.position_new_reward();
        } else {
            self.snake.positions.pop_back().unwrap();
            self.snake.positions.push_front(new_head_pos);
        }
    }

    fn position_new_reward(&mut self) {
        let mut available_positions = vec![];
        for i in 0..self.board.width {
            for j in 0..self.board.height {
                let pos = Position::new(i, j);
                if !self.board.walls.contains(&pos) && !self.snake.positions.contains(&pos) {
                    available_positions.push(pos);
                }
            }
        }
        let pos = self.rng.gen_range(0..available_positions.len());
        self.reward = available_positions[pos];
    }

    pub fn is_over(&self) -> bool {
        self.snake.has_collision(&self.board)
    }
}

pub struct Board {
    width: i16,
    height: i16,
    walls: Vec<Position>,
}

impl Board {
    fn new(width: i16, height: i16, walls: Vec<Position>) -> Board {
        Board {
            width,
            height,
            walls,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn new(x: i16, y: i16) -> Position {
        Position { x, y }
    }
}

pub struct Snake {
    positions: VecDeque<Position>,
}

impl Snake {
    fn new() -> Snake {
        let mut vec = VecDeque::new();
        vec.push_back(Position::new(0, 0));
        Snake { positions: vec }
    }
    /// Determines if the snake is colliding with the bounds of the board, any walls, or itself
    fn has_collision(&self, b: &Board) -> bool {
        let mut collision = self.has_self_collision();
        collision |= self.has_wall_collision(&b);
        collision |= self.positions[0].x < 0
            || self.positions[0].y < 0
            || self.positions[0].x >= b.width
            || self.positions[0].y >= b.height;
        collision
    }

    fn has_wall_collision(&self, b: &Board) -> bool {
        for (i, position1) in self.positions.iter().enumerate() {
            for (j, position2) in b.walls.iter().enumerate() {
                if i != j {
                    if position1 == position2 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn has_self_collision(&self) -> bool {
        for (i, position1) in self.positions.iter().enumerate() {
            for (j, position2) in self.positions.iter().enumerate() {
                if i != j {
                    if position1 == position2 {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub struct DrawableGame {
    game: Game,
    wall_objects: Vec<RectangleShape<'static>>,
    snake_objects: VecDeque<RectangleShape<'static>>,
    reward_object: RectangleShape<'static>,
    block_size: Vector2f,
    block_scale_factor: f32,
}

impl DrawableGame {
    pub fn new(width: i16, height: i16) -> DrawableGame {
        let game = Game::new(width, height);
        let mut dg = DrawableGame {
            game,
            wall_objects: vec![],
            snake_objects: VecDeque::new(),
            reward_object: RectangleShape::new(),
            block_size: Vector2f::new(WINDOW_WIDTH as f32 / 16., WINDOW_HEIGHT as f32 / 16.),
            block_scale_factor: WINDOW_HEIGHT as f32 / width as f32,
        };
        dg.setup();
        dg
    }

    fn setup(&mut self) {
        for w in self.game.board.walls.iter() {
            let mut wall = self.get_seg();
            wall.set_fill_color(Color::rgb(100, 100, 200));
            wall.set_position((
                w.x as f32 * self.block_scale_factor + self.block_size.x / 2.,
                w.y as f32 * self.block_scale_factor + self.block_size.y / 2.,
            ));
            self.wall_objects.push(wall);
        }
        for s in self.game.snake.positions.iter() {
            let mut snake_seg = self.get_seg();
            snake_seg.set_fill_color(Color::rgb(100, 200, 100));
            snake_seg.set_position((
                s.x as f32 * self.block_scale_factor + self.block_size.x / 2.,
                s.y as f32 * self.block_scale_factor + self.block_size.y / 2.,
            ));
            self.snake_objects.push_back(snake_seg);
        }
        let mut reward = self.get_seg();
        reward.set_fill_color(Color::rgb(200, 100, 100));
        self.reward_object = reward;
    }

    fn get_seg(&self) -> RectangleShape<'static> {
        let mut seg = RectangleShape::new();
        seg.set_size(self.block_size);
        seg.set_origin(self.block_size / 2.);
        seg
    }

    pub fn update_direction(&mut self, dir: Direction) {
        self.game.update_direction(dir);
    }

    pub fn move_snake(&mut self) {
        self.game.move_snake();
    }

    pub fn is_over(&self) -> bool {
        self.game.is_over()
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        self.reward_object.set_position((
            self.game.reward.x as f32 * self.block_scale_factor + self.block_size.x / 2.,
            self.game.reward.y as f32 * self.block_scale_factor + self.block_size.y / 2.,
        ));

        if self.game.snake.positions.len() > self.snake_objects.len() {
            self.snake_objects.push_front(self.get_seg());
            self.snake_objects[0].set_fill_color(Color::rgb(100, 200, 100));
        }
        for (i, position) in self.game.snake.positions.iter().enumerate() {
            self.snake_objects[i].set_position((
                position.x as f32 * self.block_scale_factor + self.block_size.x / 2.,
                position.y as f32 * self.block_scale_factor + self.block_size.y / 2.,
            ));
        }
        //let mut new_head = self.snake_objects.pop_back().unwrap();
        //println!("Pos: {:?}", self.game.snake.positions);
        /*println!(
            "Pos: {:?}",
            self.snake_objects.iter().map(|i| i.position()).collect()
        );*/
        /*self.snake_objects[0].set_position((
            self.game.snake.positions[0].x as f32 * self.block_scale_factor
                + self.block_size.x / 2.,
            self.game.snake.positions[0].y as f32 * self.block_scale_factor
                + self.block_size.y / 2.,
        ));*/
        //self.snake_objects.push_front(new_head);
        for wall in self.wall_objects.iter() {
            window.draw(wall);
        }
        for snake in self.snake_objects.iter() {
            window.draw(snake);
        }
        window.draw(&self.reward_object);
    }
}
