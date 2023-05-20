use core::ops::Add;
use std::vec;

use macroquad::prelude::*;

// macroquad default window width and height is 800 and 600;
const BLOCK_SIZE: f32 = 20.0;

#[derive(Clone, Copy)]
struct Point(i8, i8);

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
}

impl Game {
    fn new() -> Self {
        Self {
            snake_position: vec![Point(4, 2), Point(3, 2), Point(2, 2)],
            direction: Direction::Right,
            food_position: Point(5, 5),
            state: (GameState::Playing),
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
            GameState::Paused => GameState::Playing,
            GameState::Playing => GameState::Paused,
        }
    }

    fn handle_event(&mut self) {

        if is_key_down(KeyCode::O) {
            self.move_left();
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
        self.draw_snake();
        self.draw_food();
        self.handle_event();

        if self.state == GameState::Paused {
            draw_text("Paused", 400.0, 300.0, 40.0, RED);
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

        next_frame().await
    }
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

fn print_screen_info() {
    println!("screen width: {}", screen_width());
    println!("screen height: {}", screen_height());
}
