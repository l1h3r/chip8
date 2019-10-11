use crate::sdl2::SDLToken;
use crate::sdl2::Renderer;
use crate::sdl2::Texture;

#[derive(Debug)]
#[repr(C)]
pub struct SDL_Context<'a, 'b> {
  pub token: &'b SDLToken,
  pub renderer: &'b Renderer<'a>,
  pub texture: &'b Texture<'a, 'b>,
}
