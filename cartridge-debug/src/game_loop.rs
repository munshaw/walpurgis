use cartridge::cartridge::{Input, Output};
use std::sync::mpsc::{Receiver, Sender};

pub trait GameLoop {
    fn new(input: Receiver<Input>, output: Sender<Output>) -> Self;
    fn start(&self);
}

#[derive(Debug)]
pub struct GameLoopImpl {
    input: Receiver<Input>,
    output: Sender<Output>,
}

impl GameLoop for GameLoopImpl {
    fn new(input: Receiver<Input>, output: Sender<Output>) -> Self {
        Self { input, output }
    }

    fn start(&self) {}
}
