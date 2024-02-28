use crate::{canvas::Canvas, direction::Direction};
use rand::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

#[derive(Debug)]
pub struct Snake{
    head: Block,
    tail: Vec<Block>,
    food: Block,
    height: u32,
    width: u32,
    direction:  Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction
}

impl Snake {
    pub fn new(width: u32, height: u32)->Snake{
        let mut rng = thread_rng();
        let head_x:u32 = rng.gen_range(0..width);
        let head_y:u32 = rng.gen_range(0..height);

        let food_x:u32 = rng.gen_range(0..width);
        let food_y:u32 = rng.gen_range(0..height);

        let head = Block(head_x, head_y);
        let food = Block(food_x, food_y);

        let tail = Vec::new();

        Snake{
            head,
            tail,
            food,
            height,
            width,
            direction: None,
            next_direction: None,
            last_direction: Direction::Right
        }
    }

    fn gen_rand_tuple_wh(&self)->(u32, u32){
        let mut rng = thread_rng();
        let x:u32 = rng.gen_range(0..self.width);
        let y:u32 = rng.gen_range(0..self.height);
        (x, y)
    }
    pub fn change_direction(&mut self, direction: Direction){
        if !self.last_direction.opposite(direction) && self.direction.is_none(){
            self.direction = Some(direction);
        }else if self.direction.iter().any(|d| !d.opposite(direction)) {
            self.next_direction = Some(direction);

        }
    }

    pub fn update(&mut self){
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(
                (self.head.0) % self.width,
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height,
            ),
            Direction::Down => Block((self.head.0) % self.width, (self.head.1 + 1) % self.height),
            Direction::Right => Block((self.head.0 + 1) % self.width, (self.head.1) % self.height),
            Direction::Left => Block(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height,
            ),
        };

        self.tail.insert(0, self.head);// can be done using queue
        let last_end = self.tail.pop();

        if self.tail.contains(&new_head){
            *self = Snake::new(self.width, self.height);
        }

        self.head=new_head;

        if self.head==self.food{
            let mut food = self.food;
            while food == self.head || self.tail.contains(&food) {
                let (food_x, food_y) = self.gen_rand_tuple_wh();

                food = Block(food_x, food_y);
            }

            self.food = food;
            last_end.map(|x| self.tail.push(x));
        }
        self.direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas){
        canvas.clear_all();
        canvas.draw(self.head.0, self.head.1, "green");
        for &Block(x, y) in &self.tail {
            canvas.draw(x, y, "lightgreen");
        }
        canvas.draw(self.food.0, self.food.1, "red");
    }
}
