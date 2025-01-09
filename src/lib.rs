use std::fmt::{Display, Formatter, Result};

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;
#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}
impl Display for GameStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GameStatus::Won => {
                write!(f, "Won")
            }
            GameStatus::Lost => {
                write!(f, "Lost")
            }
            GameStatus::Played => {
                write!(f, "Playing")
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}
impl Snake {
    fn new(spawn_index: usize, direction: Direction, size: usize) -> Snake {
        let mut body = vec![];
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }
        Snake { body, direction }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(
        width: usize,
        spawn_idx: usize,
        snake_direction: Direction,
        snake_size: usize,
    ) -> World {
        let size = width * width;
        let snake = Snake::new(spawn_idx, snake_direction, snake_size);
        let mut reward: Option<usize>;
        loop {
            reward = Some(rnd(size));
            if !snake.body.contains(&SnakeCell(reward.unwrap())) {
                break;
            }
        }

        World {
            width,
            size,
            snake,
            next_cell: None,
            reward_cell: reward,
            status: None,
            points:0,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }
    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
    pub fn get_game_status(&self) -> Option<GameStatus> {
        self.status
    }
    pub fn get_game_status_label(&self) -> String {
        match self.status {
            None => {String::from("None")},
            _ => {self.status.unwrap().to_string()}
        }
    }
    //returning a raw pointer, borrowing rules not applied
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }
    pub fn change_game_status(&mut self) -> GameStatus {
        match self.status {
            None => {
                self.status = Some(GameStatus::Played);
            }
            _ => {
                self.status = None;
            }
        }
        self.status.unwrap()
    }
    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }
        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }
    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                match self.next_cell {
                    Some(cell) => {
                        self.snake.body.insert(0, cell);
                    }
                    None => {
                        self.snake
                            .body
                            .insert(0, self.generate_next_snake_cell(&self.snake.direction));
                    }
                }
                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }
                self.next_cell = None;
                if self.reward_cell.unwrap() != self.snake_head_idx() {
                    self.snake.body.pop();
                } else if self.snake.body.len() < self.size {
                    self.points += 1;
                    self.reward_cell = Some(rnd(self.size));
                } else {
                    self.reward_cell = None;
                    self.status = Some(GameStatus::Won);
                }
            }
            _ => {}
        }
    }
    pub fn points(&self) -> usize {
        self.points
    }
    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row: usize = snake_idx / self.width();
        return match direction {
            Direction::Right => SnakeCell((row * self.width()) + (snake_idx + 1) % self.width()),
            Direction::Left => SnakeCell((row * self.width()) + (snake_idx - 1) % self.width()),
            Direction::Up => SnakeCell((snake_idx - self.width()) % (self.size)),
            Direction::Down => SnakeCell((snake_idx + self.width()) % (self.size)),
        };
    }
}
