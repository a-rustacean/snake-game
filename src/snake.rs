use crate::{helper::{Vector, log, console_log}, random::random_range, renderer::Renderable};
use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[allow(dead_code)]
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }

    fn to_vec(&self) -> Vector {
        match self {
            Direction::Up => Vector(0, -1),
            Direction::Left => Vector(-1, 0),
            Direction::Down => Vector(0, 1),
            Direction::Right => Vector(1, 0),
        }
    }
}

#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: VecDeque<Vector>,
    foods: Vec<Vector>,
    food_icons: Vec<char>,
    direction: Direction,
    changed_direction: Direction,
    food_count: usize,
    finished: bool,
}

#[allow(dead_code)]
impl SnakeGame {
    pub fn new(width: usize, height: usize, food_count: usize) -> Self {
        let mut game = Self {
            width,
            height,
            snake: [Vector(width as isize - 1, height as isize / 2)]
                .into_iter()
                .collect(),
            foods: vec![],
            food_icons: vec![],
            direction: Direction::Left,
            changed_direction: Direction::Left,
            food_count,
            finished: false,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        let mut i = 0;
        while self.foods.len() < self.food_count {
            let new_food = Vector(
                random_range(0, self.width) as isize,
                random_range(0, self.height) as isize,
            );
            if self.foods.contains(&new_food) || self.snake.contains(&new_food) {
                continue;
            }
            let food_icons = [
                '🥕',
                '🍞',
                '🥑',
                '🍒',
            ];
            self.food_icons.push(food_icons[random_range(0, food_icons.len())]);
            self.foods.push(new_food);
            if i > 1000 {
                break;
            }
            i += 1;
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.direction == direction || self.direction == direction.opposite() {
            return;
        }
        self.changed_direction = direction;
    }

    fn valid_pos(&self, pos: &Vector) -> bool {
        pos.0 >= 0
            && pos.0 < self.width as isize
            && pos.1 >= 0
            && pos.1 < self.height as isize
            && !self.snake.contains(&pos)
    }

    pub fn tick(&mut self) {
        if self.finished {
            return;
        }
        self.direction = self.changed_direction.clone();
        let head = &self.snake[0];
        let new_head = head + self.direction.to_vec();

        if !self.valid_pos(&new_head) {
            self.finished = true;
            alert("You're a loser!");
            return;
        }

        if let Some(i) = self.foods.iter().position(|value| value == &new_head) {
            self.foods.remove(i);
            self.food_icons.remove(i);
            self.spawn_food();
            log!(new_head);
        } else {
            self.snake.pop_back();
        };
        self.snake.push_front(new_head);
    }
}

impl Renderable for SnakeGame {
    fn renderer_data(&self) -> HashMap<Vector, char> {
        let mut data = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vector(x as isize, y as isize);
                let char: char;
                if self.snake.contains(&pos) {
                    if pos == self.snake[0] {
                        char = '🟨';
                    } else {
                        char = '⬛';
                    }
                } else if let Some(i) = self.foods.iter().position(|food| food == &pos) {
                    char = self.food_icons[i];
                } else {
                    char = ' ';
                }
                data.insert(pos, char);
            }
        }
        data
    }
}
