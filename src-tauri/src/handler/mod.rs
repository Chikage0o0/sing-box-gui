mod sing_box_core;
use std::sync::OnceLock;

use actix::{Actor, Addr, Arbiter, System};

#[derive(Debug)]
pub struct Handler {
    arbiter: Arbiter,
}

static HANDLER: OnceLock<Handler> = OnceLock::new();

impl Handler {
    pub fn new() -> Self {
        let arbiter = Arbiter::new();

        let run = async move {};

        arbiter.spawn(run);

        Self { arbiter }
    }
}

pub fn init() {
    let _system = System::new();

    let _ = HANDLER.get_or_init(|| Handler::new());
}

pub fn global() -> &'static Handler {
    HANDLER.get().unwrap()
}

pub fn stop() {
    let handler = global();
    while !handler.arbiter.stop() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
