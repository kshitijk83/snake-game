use direction::Direction;
use snake::Snake;
use wasm_bindgen::prelude::*;
use std::{cell::RefCell, rc::Rc};

mod direction;
mod canvas;
mod snake;
mod utils;

use canvas::Canvas;

#[wasm_bindgen(start)]
fn start(){
    let canvas = Canvas::new("canvas", 20, 20);
    
    let snake = Rc::new(RefCell::new(Snake::new(20, 20)));
    snake.borrow().draw(&canvas);
    let snake_c = snake.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent|{
        
        match event.key().as_str() {
            "ArrowLeft"=>snake_c.borrow_mut().change_direction(Direction::Left),
            "ArrowRight"=>snake_c.borrow_mut().change_direction(Direction::Right),
            "ArrowUp"=>snake_c.borrow_mut().change_direction(Direction::Up),
            "ArrowDown"=>snake_c.borrow_mut().change_direction(Direction::Down),
            _=>{}
        }
    });
    let _ = utils::document().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
    closure.forget();

    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>, time: u32){
        let snake_clone = snake.clone();
        let canvas_clone = canvas.clone();
        let cl = Closure::<dyn Fn()>::new(move ||{
            snake_clone.borrow_mut().update();
            snake_clone.borrow().draw(&canvas_clone);
            game_loop(snake_clone.clone(), canvas_clone.clone(), time);
        });
        
        let _ = utils::window().set_timeout_with_callback_and_timeout_and_arguments_0(cl.as_ref().clone().as_ref().unchecked_ref(), 100);

        cl.forget();
    }

    game_loop(snake, Rc::new(canvas), 100);

   
}