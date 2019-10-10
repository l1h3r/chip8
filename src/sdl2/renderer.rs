use core::ops::Deref;

use crate::sdl2::SDL_DestroyRenderer;
use crate::sdl2::SDL_Rect;
use crate::sdl2::SDL_RenderClear;
use crate::sdl2::SDL_RenderFillRect;
use crate::sdl2::SDL_RenderPresent;
use crate::sdl2::SDL_Renderer;
use crate::sdl2::SDL_SetRenderDrawColor;
use crate::sdl2::Window;

#[derive(Debug)]
pub struct Renderer<'a> {
  inner: *mut SDL_Renderer,
  window: Window<'a>,
}

impl<'a> Renderer<'a> {
  #[inline]
  pub(crate) const fn new(inner: *mut SDL_Renderer, window: Window<'a>) -> Self {
    Self { inner, window }
  }

  #[inline]
  pub fn present(&self) {
    unsafe {
      SDL_RenderPresent(self.inner);
    }
  }

  #[inline]
  pub fn clear(&self) {
    try_sdl2!(SDL_RenderClear, self.inner);
  }

  #[inline]
  pub fn color(&self, r: u8, g: u8, b: u8) {
    try_sdl2!(SDL_SetRenderDrawColor, self.inner, r, g, b, 255);
  }

  #[inline]
  pub fn fill_rect(&self, x: i32, y: i32, w: i32, h: i32) {
    try_sdl2!(SDL_RenderFillRect, self.inner, &SDL_Rect { x, y, w, h });
  }
}

impl<'a> Drop for Renderer<'a> {
  fn drop(&mut self) {
    unsafe {
      SDL_DestroyRenderer(self.inner);
    }
  }
}

impl<'a> Deref for Renderer<'a> {
  type Target = Window<'a>;

  fn deref(&self) -> &Self::Target {
    &self.window
  }
}
