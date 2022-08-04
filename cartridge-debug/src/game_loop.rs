use cartridge::cartridge::{Input, Output};
use mockall::predicate::*;
use mockall::*;
use std::sync::mpsc::{Receiver, Sender};
use tileset::tileset::TileId;

#[automock]
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

    fn start(&self) {
        // Cannot gracefully exit without UI contact.
        self.output
            .send(Output::Draw(TileId::WHITE, (2, 3)))
            .unwrap();

        if let Input::Char(c) = self.input.recv().unwrap() {
            println!("User input: {}", c);
        }
    }
}
