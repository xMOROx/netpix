use futures_util::stream::StreamExt;
use log::{error, info};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, CloseEvent, ErrorEvent, MessageEvent, WebSocket};

pub struct WebSocketManager {
    ws: Option<WebSocket>,
    message_queue: Rc<RefCell<Vec<Vec<u8>>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            ws: None,
            message_queue: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub async fn connect(&mut self, url: &str) -> Result<(), JsValue> {
        info!("Connecting to WebSocket at {}", url);
        
        let ws = WebSocket::new(url)?;
        ws.set_binary_type(BinaryType::Arraybuffer);

        let message_queue = self.message_queue.clone();

        // Set up message handler
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let array = js_sys::Uint8Array::new(&abuf);
                let vec = array.to_vec();
                message_queue.borrow_mut().push(vec);
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Set up error handler
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            error!("WebSocket error: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        // Set up close handler
        let onclose_callback = Closure::wrap(Box::new(move |e: CloseEvent| {
            info!("WebSocket closed: code={}, reason={}", e.code(), e.reason());
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        // Set up open handler
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            info!("WebSocket connected");
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        self.ws = Some(ws);
        Ok(())
    }

    pub async fn receive_message(&self) -> Option<Vec<u8>> {
        self.message_queue.borrow_mut().pop()
    }

    pub fn send(&self, data: &[u8]) -> Result<(), JsValue> {
        if let Some(ref ws) = self.ws {
            ws.send_with_u8_array(data)?;
        }
        Ok(())
    }
}
