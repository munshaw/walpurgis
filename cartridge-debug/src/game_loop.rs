use cartridge::cartridge::{Input, Output};
use mockall::predicate::*;
use mockall::*;
use std::sync::mpsc::{Receiver, Sender};
use tileset::tileset::TileId;

#[automock]
pub trait GameLoop {
    fn start(&self, input: Receiver<Input>, output: Sender<Output>);
}

#[derive(Debug, Default)]
pub struct GameLoopImpl {}

impl GameLoop for GameLoopImpl {
    fn start(&self, input: Receiver<Input>, output: Sender<Output>) {
        // Cannot gracefully exit without UI contact.
        output.send(Output::Draw(TileId::WHITE, (2, 3))).unwrap();

        if let Input::Char(c) = input.recv().unwrap() {
            println!("User input: {}", c);
        }
    }
}
