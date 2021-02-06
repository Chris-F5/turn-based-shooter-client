use log::{error, info, warn};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use turn_based_shooter_shared::{ClientPacket, ServerPacket};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use ConnectionStatus::{Closed, Closing, Connected, Connecting};

struct Sender {
    ws: WebSocket,
}

impl Sender {
    fn new(ws: WebSocket) -> Sender {
        Sender { ws }
    }
    fn send(&mut self, packet: ClientPacket) {
        let bytes = bincode::serialize(&packet).unwrap();
        match self.ws.send_with_u8_array(&bytes) {
            Ok(_) => (),
            Err(err) => error!("error sending message: {:?}", err),
        }
    }
    fn get_status(&self) -> ConnectionStatus {
        match self.ws.ready_state() {
            0 => Connecting,
            1 => Connected,
            2 => Closing,
            3 => Closed,
            _ => panic!("invalid web socket ready state"),
        }
    }
}

pub struct ServerConnection {
    sender: Rc<RefCell<Sender>>,
    unread_responses: Rc<RefCell<VecDeque<ServerPacket>>>,
    unsent: Rc<RefCell<VecDeque<ClientPacket>>>,
}

impl ServerConnection {
    pub fn new() -> ServerConnection {
        let ws = WebSocket::new("ws://localhost:8080/game_server/").unwrap();
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        let sender = Rc::new(RefCell::new(Sender::new(ws)));
        let unread_responses = Rc::new(RefCell::new(VecDeque::new()));
        let unsent = Rc::new(RefCell::new(VecDeque::new()));

        let cloned_responses = unread_responses.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let js_array = js_sys::Uint8Array::new(&abuf);
                let bytes = js_array.to_vec();
                let packet: ServerPacket = bincode::deserialize(&bytes).unwrap();
                cloned_responses.borrow_mut().push_back(packet);
            } else {
                error!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            error!("error event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);

        let cloned_sender = sender.clone();
        let cloned_unsent = unsent.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            info!("socket opened!");
            let mut unsent = cloned_unsent.borrow_mut();
            let mut sender = cloned_sender.borrow_mut();
            while let Some(packet) = unsent.pop_front() {
                sender.send(packet);
                info!(
                    "sent pending packet. {} pending packets left.",
                    unsent.len()
                );
            }
        }) as Box<dyn FnMut(JsValue)>);
        let borrowed_sender = sender.borrow();
        borrowed_sender
            .ws
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        borrowed_sender
            .ws
            .set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        borrowed_sender
            .ws
            .set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        drop(borrowed_sender);
        onmessage_callback.forget();
        onerror_callback.forget();
        onopen_callback.forget();

        ServerConnection {
            sender,
            unread_responses,
            unsent,
        }
    }
    pub fn send(&mut self, packet: ClientPacket) {
        let mut sender = self.sender.borrow_mut();
        match sender.get_status() {
            Connected => {
                sender.send(packet);
            }
            Connecting | Closed | Closing => {
                let mut unsent = self.unsent.borrow_mut();
                unsent.push_back(packet);
                warn!("{} packets pending", unsent.len());
            }
        }
    }
    pub fn try_recv(&mut self) -> Option<ServerPacket> {
        self.unread_responses.borrow_mut().pop_front()
    }
    pub fn get_status(&self) -> ConnectionStatus {
        self.sender.borrow().get_status()
    }
}

pub enum ConnectionStatus {
    Connecting,
    Connected,
    Closing,
    Closed,
}
