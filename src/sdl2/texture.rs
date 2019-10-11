use core::marker::PhantomData;

use crate::sdl2::SDL_Texture;
use crate::sdl2::SDL_DestroyTexture;
use crate::sdl2::Renderer;

#[derive(Debug)]
#[repr(transparent)]
pub struct Texture<'a, 'b> {
  inner: *mut SDL_Texture,
  _marker: PhantomData<&'b Renderer<'a>>,
}

impl<'a, 'b> Texture<'a, 'b> {
  #[inline]
  pub(crate) const fn new(inner: *mut SDL_Texture) -> Self {
    Self {
      inner,
      _marker: PhantomData,
    }
  }

  #[inline]
  pub(crate) fn as_ptr(&self) -> *mut SDL_Texture {
    self.inner
  }
}

impl<'a, 'b> Drop for Texture<'a, 'b> {
  fn drop(&mut self) {
    unsafe {
      SDL_DestroyTexture(self.inner);
    }
  }
}
