mod graphics;
mod networking;

use graphics::GraphicsContext;
use log::{error, info, trace, warn};
use networking::ServerConnection;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use turn_based_shooter_shared::{ClientPacket, ServerPacket, TestRequest};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).unwrap();
    let game = Game::init();
    game.start_game_loop();
}

struct Game {
    server_connection: ServerConnection,
    graphics_ctx: GraphicsContext,
}
impl Game {
    fn init() -> Game {
        let mut server_connection = ServerConnection::new();
        server_connection.send(ClientPacket::Test(TestRequest::new("bob".to_string())));
        Game {
            server_connection,
            graphics_ctx: GraphicsContext::new("canvas"),
        }
    }
    fn update(&mut self) {
        trace!("update");
        while let Some(server_packet) = self.server_connection.try_recv() {
            match server_packet {
                ServerPacket::Test(test_packet) => info!(
                    "received message {}: {}",
                    test_packet.number, test_packet.message
                ),
            }
        }
    }
    fn draw(&mut self) {
        self.graphics_ctx.clear();
        self.graphics_ctx.draw_tile(&graphics::TilePos::new(0, 0));
        self.graphics_ctx.draw_tile(&graphics::TilePos::new(1, 0));
        self.graphics_ctx.draw_tile(&graphics::TilePos::new(2, 0));
        self.graphics_ctx.draw_tile(&graphics::TilePos::new(1, 1));
    }
    fn game_loop(&mut self) {
        self.update();
        self.draw();
    }
    fn start_game_loop(mut self) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            self.game_loop();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
