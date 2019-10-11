use core::ops::Deref;

use crate::sdl2::SDL_Texture;
use crate::sdl2::SDL_CreateTexture;
use crate::sdl2::Texture;
use crate::sdl2::PixelFormat;
use crate::sdl2::TextureAccess;
use crate::sdl2::SDL_DestroyRenderer;
use crate::sdl2::SDL_Rect;
use crate::sdl2::SDL_RenderClear;
use crate::sdl2::SDL_RenderFillRect;
use crate::sdl2::SDL_RenderPresent;
use crate::sdl2::SDL_RenderDrawLine;
use crate::sdl2::SDL_CreateTextureFromSurface;
use crate::sdl2::SDL_Renderer;
use crate::sdl2::SDL_SetRenderDrawColor;
use crate::sdl2::SDL_RenderCopy;
use crate::sdl2::Surface;
use crate::sdl2::Window;
use crate::sdl2::error;

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

  pub fn create_texture(
    &self,
    format: PixelFormat,
    access: TextureAccess,
    w: i32,
    h: i32,
  ) -> Result<Texture, &'static str> {
    let texture: *mut SDL_Texture = unsafe {
      SDL_CreateTexture(self.inner, format as u32, access as i32, w, h)
    };

    if texture.is_null() {
      Err(error())
    } else {
      Ok(Texture::new(texture))
    }
  }

  pub fn create_texture_from_surface(
    &self,
    surface: Surface,
  ) -> Result<Texture, &'static str> {
    let texture: *mut SDL_Texture = unsafe {
      SDL_CreateTextureFromSurface(self.inner, surface.as_ptr())
    };

    if texture.is_null() {
      Err(error())
    } else {
      Ok(Texture::new(texture))
    }
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

  #[inline]
  pub fn copy(&self, texture: &Texture, srcrect: &SDL_Rect, dstrect: &SDL_Rect) {
    try_sdl2!(SDL_RenderCopy, self.inner, texture.as_ptr(), srcrect, dstrect);
  }

  #[inline]
  pub fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
    try_sdl2!(SDL_RenderDrawLine, self.inner, x1, y1, x2, y2);
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
