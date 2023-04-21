use crate::{helper::Vector, random::random_range, renderer::Renderable};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlAudioElement};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

fn get_audio_element() -> HtmlAudioElement {
    let window = window().unwrap_throw();
    let document = window.document().unwrap_throw();
    match document.get_element_by_id("audio-player") {
        Some(audio) => audio.dyn_into().unwrap_throw(),
        None => {
            let audio = HtmlAudioElement::new_with_src("./assets/eating.mp3").unwrap_throw();
            document.body().unwrap_throw().append_child(&audio).unwrap();
            audio
        }
    }
}

pub fn save_game_data(data: GameData) -> Result<(), JsValue> {
    let window = window().ok_or("Window not found")?;
    let storage = window
        .local_storage()?
        .ok_or("Local storage is not supported")?;
    let string = serde_json::to_string(&data).map_err(|e| JsValue::from(e.to_string()))?;
    storage.set_item("snake-game-data", &string)?;
    Ok(())
}

pub fn load_game_data() -> Result<GameData, JsValue> {
    let window = window().ok_or("Window not found")?;
    let storage = window
        .local_storage()?
        .ok_or("Local storage is not supported")?;
    if let Some(string) = storage.get_item("snake-game-data")? {
        serde_json::from_str(&string).map_err(|e| JsValue::from(e.to_string()))
    } else {
        Err("Game data not found".into())
    }
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

    fn to_vec(self) -> Vector {
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
    pub snake: VecDeque<Vector>,
    foods: Vec<Vector>,
    food_icons: Vec<char>,
    direction: Direction,
    changed_direction: Direction,
    food_count: usize,
    pub finished: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameData {
    pub high_score: usize,
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
            let food_icons = ['🥕', '🍞', '🥑', '🍒'];
            self.food_icons
                .push(food_icons[random_range(0, food_icons.len())]);
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
            && !self.snake.contains(pos)
    }

    pub fn tick(&mut self) {
        if self.finished {
            return;
        }
        self.direction = self.changed_direction;
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
            let _ = get_audio_element().play().unwrap_throw();
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
