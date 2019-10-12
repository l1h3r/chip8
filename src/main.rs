#![allow(bare_trait_objects)]
#![feature(cell_update, rustc_private)]

#[macro_use]
extern crate bitflags;
extern crate libc;

mod chip8;
mod instruction;
mod sdl2;

use crate::chip8::Chip8;
use crate::chip8::Mode;
use crate::chip8::H;
use crate::chip8::W;
use crate::sdl2::AudioDevice;
use crate::sdl2::Renderer;
use crate::sdl2::RendererFlags;
use crate::sdl2::SDLToken;
use crate::sdl2::SDL_AudioSpec;
use crate::sdl2::SDL_Context;
use crate::sdl2::Surface;
use crate::sdl2::Texture;
use crate::sdl2::Window;
use crate::sdl2::WindowFlags;
use crate::sdl2::AUDIO_S16SYS;

static FONT_BMP: &'static [u8] = include_bytes!("font.bmp");

const WW: i32 = (W as i32 * 10) + 320 + 20;
const WH: i32 = (H as i32 * 10) + 320 + 20;

fn main() -> Result<(), &'static str> {
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

  let audio: AudioDevice = token.open_audio_device(SDL_AudioSpec {
    freq: AudioDevice::FREQUENCY,
    format: AUDIO_S16SYS as u16,
    channels: AudioDevice::CHANNELS,
    silence: 0,
    samples: AudioDevice::SAMPLES,
    padding: 0,
    size: 0,
    callback: None,
    userdata: 0 as *mut _,
  })?;

  let context = SDL_Context {
    token: &token,
    renderer: &renderer,
    texture: &texture,
    audio: &audio,
  };

  let mut interpreter: Chip8 = Chip8::new();

  interpreter.mode = Mode::CHIP;
  interpreter.load("roms/CHIP/PONG");
  interpreter.start(&context);

  Ok(())
}
