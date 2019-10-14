#![allow(bare_trait_objects)]
#![feature(cell_update, rustc_private)]

#[macro_use]
extern crate bitflags;
extern crate libc;

mod chip8;
mod runner;
mod sdl2;

use std::env::args;

use crate::runner::ChipRunner;
use crate::sdl2::AudioBell;
use crate::sdl2::Renderer;
use crate::sdl2::RendererFlags;
use crate::sdl2::SDLToken;
use crate::sdl2::SDL_Context;
use crate::sdl2::Surface;
use crate::sdl2::Texture;
use crate::sdl2::Window;
use crate::sdl2::WindowFlags;

static FONT_BMP: &'static [u8] = include_bytes!("font.bmp");

#[derive(Debug)]
#[repr(C)]
pub struct Args {
  pub eti: bool,
  pub rom: String,
}

impl Args {
  pub fn from_env() -> Self {
    let mut data: Self = Self {
      eti: false,
      rom: String::new(),
    };

    for arg in args() {
      match arg.as_str() {
        "--eti" => data.eti = true,
        _ => data.rom = arg,
      }
    }

    data
  }
}

fn main() -> Result<(), &'static str> {
  let args: Args = Args::from_env();
  let token: SDLToken = SDLToken::init()?;

  let window: Window = token.create_window(
    "Chip-8\0",
    Window::CENTERED_MASK,
    Window::CENTERED_MASK,
    ChipRunner::W,
    ChipRunner::H,
    WindowFlags::OPENGL,
  )?;

  let renderer: Renderer = window
    .try_into_renderer(RendererFlags::ACCELERATED | RendererFlags::PRESENTVSYNC)
    .map_err(|(_, error)| error)?;

  let surface: Surface = Surface::from_const_bytes(FONT_BMP)?;

  surface.set_color(255, 0, 255);

  let texture: Texture = renderer.create_texture_from_surface(surface)?;
  let audio: AudioBell = AudioBell::new(&token)?;

  let context = SDL_Context {
    token: &token,
    renderer: &renderer,
    texture: &texture,
    audio: &audio,
  };

  let mut runner: ChipRunner = ChipRunner::new();

  runner.load(&args.rom, args.eti)?;
  runner.run(&context);

  Ok(())
}
