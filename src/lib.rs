use wasm_bindgen::{prelude::*, JsCast};
use wasm_game_lib::graphics::image::Image;
use wasm_game_lib::graphics::sprite::Sprite;
use wasm_game_lib::inputs::event::Event;
use wasm_game_lib::graphics::window::Window;
use wasm_game_lib::system::log;
use wasm_game_lib::inputs::event::types::*;
use wasm_game_lib::system::sleep;
use std::time::Duration;
use console_error_panic_hook::set_once;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    set_once(); // needed to see panic messages in the console of your web browser

    let (mut window, mut canvas) = Window::init_with_events(MOUSE_EVENT + KEYBOARD_EVENT + RESIZE_EVENT + FOCUS_EVENT);

    // load images and fonts here
    // you could make a progress bar

    loop {
        for event in window.poll_events() {
            // do something with events
            log(&format!("{:?}", event));
        }

        canvas.clear();
        // canvas.draw(&object);

        sleep(Duration::from_millis(16)).await;
    }

    Ok(())
}