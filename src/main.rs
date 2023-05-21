use std::fmt::Write;
use core::ops::Add;
use std::vec;

use macroquad::prelude::*;

// macroquad default window width and height is 800 and 600;
const BLOCK_SIZE: f32 = 20.0;
const GRID_X: i32 = 40;
const GRID_Y: i32 = 30;

#[derive(Clone, Copy)]
struct Point(i32, i32);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    Paused,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake_position: Vec<Point>,
    direction: Direction,
    food_position: Point,
    state: GameState,
    score: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            snake_position: vec![Point(4, 2), Point(3, 2), Point(2, 2)],
            direction: Direction::Right,
            food_position: Point(5, 5),
            state: (GameState::Playing),
            score: 0,
        }
    }

    fn next_tick(&mut self) {
        if let GameState::Paused = self.state {
            return;
        }

        let head = self.snake_position.first().unwrap();

        let next_head_position = match self.direction {
            Direction::Up => *head + Point(0, -1),
            Direction::Down => *head + Point(0, 1),
            Direction::Left => *head + Point(-1, 0),
            Direction::Right => *head + Point(1, 0),
        };

        self.snake_position.pop();
        self.snake_position.reverse();
        self.snake_position.push(next_head_position);
        self.snake_position.reverse();
    }

    fn draw_snake(&mut self) {
        for block in &self.snake_position {
            draw_blocks(&block, GREEN);
        }
        let Point(mut snake_x, mut snake_y) = self.snake_position.first().unwrap();
        if snake_x > GRID_X {
            snake_x = 0;
        }
    }

    fn draw_food(&mut self) {
        draw_blocks(&self.food_position, RED);
    }

    fn move_left(&mut self) {
        self.direction = Direction::Left;
    }
    fn move_right(&mut self) {
        self.direction = Direction::Right;
    }
    fn move_up(&mut self) {
        self.direction = Direction::Up;
    }
    fn move_down(&mut self) {
        self.direction = Direction::Down;
    }

    fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        }
    }

    fn handle_event(&mut self) {
        if is_key_down(KeyCode::O) {
        self.direction = Direction::Left;
        }
        if is_key_down(KeyCode::U) {
            self.move_right();
        }
        if is_key_down(KeyCode::E) {
            self.move_down();
        }
        if is_key_down(KeyCode::Period) {
            self.move_up();
        }
        if is_key_down(KeyCode::Space) {
            self.toggle_pause();
        }
    }

    fn render(&mut self) {
        self.generate_food();
        self.draw_snake();
        self.draw_food();
        self.handle_event();

        if self.state == GameState::Paused {
            draw_text("Paused", 400.0, 300.0, 40.0, RED);
        }
    }

    fn ate_food(&self) -> bool {
        let Point(snake_x, snake_y) = self.snake_position.first().unwrap();
        let Point(food_x, food_y) = self.food_position;

        return *snake_x == food_x && *snake_y == food_y;
    }

    fn generate_food(&mut self) {
        if self.ate_food() {
            self.food_position = Point(rand::gen_range(0, GRID_X), rand::gen_range(0, GRID_Y));
            self.score += 1;
        }
    }
}

#[macroquad::main("hello")]
async fn main() {

    let mut game = Game::new();

    let mut frame_counter = 0;

    loop {

        game.render();
        frame_counter += 1;

        if frame_counter % 5 == 0 {
            game.next_tick();
            frame_counter = 0;
        }
        display_score(&game);
        next_frame().await
    }
}

fn display_score(game: &Game) {
    let mut s = String::new();
    write!(s, "Score: {}", game.score);
    draw_text(&s, 300.0, 30.0, 30.0, GREEN);
}

fn draw_blocks(point: &Point, color: Color) {
    let Point(x, y) = point;
    draw_rectangle(
        (*x as f32) * BLOCK_SIZE,
        (*y as f32) * BLOCK_SIZE,
        BLOCK_SIZE,
        BLOCK_SIZE,
        color,
    );
}

fn _print_screen_info() {
    println!("screen width: {}", screen_width());
    println!("screen height: {}", screen_height());
}
