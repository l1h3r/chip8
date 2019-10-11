#![allow(bare_trait_objects)]
#![feature(cell_update, rustc_private)]
#![no_main]

#[macro_use]
extern crate bitflags;
extern crate libc;

mod chip8;
mod arguments;
mod instruction;
mod sdl2;

use crate::arguments::Args;
use crate::chip8::Chip8;
use crate::chip8::H;
use crate::chip8::W;
use crate::sdl2::Renderer;
use crate::sdl2::RendererFlags;
use crate::sdl2::SDLToken;
use crate::sdl2::Surface;
use crate::sdl2::Texture;
use crate::sdl2::Window;
use crate::sdl2::WindowFlags;
use crate::sdl2::SDL_Context;

static FONT_BMP: &'static [u8] = include_bytes!("font.bmp");

const WW: i32 = (W as i32 * 10) + 320 + 20;
const WH: i32 = (H as i32 * 10) + 20;

#[no_mangle]
pub extern "C" fn main(argc: i32, argv: *const *const u8) -> i32 {
  let args: Args = Args::parse(argc, argv);

  match run(args) {
    Ok(()) => 0,
    Err(_) => 1,
  }
}

fn run(args: Args) -> Result<(), &'static str> {
  let token: SDLToken = SDLToken::init()?;

  let window: Window = token.create_window(
    "Chip-8\0",
    Window::CENTERED_MASK,
    Window::CENTERED_MASK,
    WW,
    WH,
    WindowFlags::OPENGL,
  )?;

  let renderer: Renderer = window
    .try_into_renderer(RendererFlags::ACCELERATED | RendererFlags::PRESENTVSYNC)
    .map_err(|(_, error)| error)?;

  let surface: Surface = Surface::from_const_bytes(FONT_BMP)?;

  surface.set_color(255, 0, 255);

  let texture: Texture = renderer.create_texture_from_surface(surface)?;

  let context = SDL_Context {
    token: &token,
    renderer: &renderer,
    texture: &texture,
  };

  let mut interpreter: Chip8 = Chip8::new();

  interpreter.debug = args.debug;
  interpreter.mode = args.mode;
  interpreter.load(args.rom);
  interpreter.start(&context);

  Ok(())
}
