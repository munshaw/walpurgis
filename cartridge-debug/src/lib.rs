//! `CartridgeDebug` is an implementation of `Cartridge` designed to test
//! players.

pub mod cartridge_debug;

pub mod game_loop;

#[cfg(test)]
mod cartridge_debug_tests;

#[cfg(test)]
mod game_loop_tests;
