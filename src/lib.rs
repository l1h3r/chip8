#![allow(bare_trait_objects)]
#![feature(cell_update, rustc_private)]

#[macro_use]
extern crate bitflags;
extern crate libc;

#[macro_use]
mod macros;

mod chip8;
mod instruction;
mod runner;
mod sdl2;

pub use self::chip8::Chip8;
pub use self::chip8::Mode;
pub use self::instruction::Instruction;
pub use self::runner::ChipRunner;
pub use self::sdl2::*;
