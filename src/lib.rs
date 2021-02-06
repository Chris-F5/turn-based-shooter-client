mod battle_client;
mod graphics;
mod input;
mod networking;

use battle_client::BattleClient;
use graphics::GraphicsContext;
use input::Input;
use log::{error, info};
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
    input: Input,
    server_connection: ServerConnection,
    graphics_ctx: GraphicsContext,
    battle_client: Option<BattleClient>,
}
impl Game {
    fn init() -> Game {
        let input = Input::new();
        let mut server_connection = ServerConnection::new();
        server_connection.send(ClientPacket::Test(TestRequest::new("bob".to_string())));
        server_connection.send(ClientPacket::JoinBattleMatchmaker);
        Game {
            input,
            battle_client: None,
            server_connection,
            graphics_ctx: GraphicsContext::new("canvas"),
        }
    }
    fn update(&mut self) {
        while let Some(server_packet) = self.server_connection.try_recv() {
            match server_packet {
                ServerPacket::Test(test_packet) => info!(
                    "received message {}: {}",
                    test_packet.number, test_packet.message
                ),
                ServerPacket::BattleStart(battle_info) => {
                    info!("received battle start packet");
                    if self.battle_client.is_none() {
                        self.battle_client = Some(BattleClient::new(battle_info))
                    } else {
                        error!("cant start battle when already in battle");
                    }
                }
                ServerPacket::BattleInfoUpdate(_battle_info_update) => {
                    info!("received battle info update packet");
                }
            }
        }

        if self.input.up_arrow() {
            self.graphics_ctx.camera().position().y += 0.1;
            self.graphics_ctx.camera().position().x -= 0.1;
        }
        if self.input.down_arrow() {
            self.graphics_ctx.camera().position().y -= 0.1;
            self.graphics_ctx.camera().position().x += 0.1;
        }
        if self.input.right_arrow() {
            self.graphics_ctx.camera().position().x += 0.1;
            self.graphics_ctx.camera().position().y += 0.1;
        }
        if self.input.left_arrow() {
            self.graphics_ctx.camera().position().x -= 0.1;
            self.graphics_ctx.camera().position().y -= 0.1;
        }
    }
    fn draw(&mut self) {
        self.graphics_ctx.clear();
        if let Some(battle_client) = &mut self.battle_client {
            battle_client.draw(&mut self.graphics_ctx);
        }
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
