use core::marker::PhantomData;

use crate::sdl2::error;
use crate::sdl2::OpenGL;
use crate::sdl2::Renderer;
use crate::sdl2::RendererFlags;
use crate::sdl2::SDLToken;
use crate::sdl2::SDL_CreateRenderer;
use crate::sdl2::SDL_DestroyWindow;
use crate::sdl2::SDL_GLContext;
use crate::sdl2::SDL_GL_CreateContext;
use crate::sdl2::SDL_Renderer;
use crate::sdl2::SDL_Window;
use crate::sdl2::SDL_WINDOWPOS_CENTERED_MASK;
use crate::sdl2::SDL_WINDOWPOS_UNDEFINED_MASK;

#[derive(Debug)]
#[repr(transparent)]
pub struct Window<'a> {
  inner: *mut SDL_Window,
  _marker: PhantomData<&'a SDLToken>,
}

impl<'a> Window<'a> {
  pub const CENTERED_MASK: i32 = SDL_WINDOWPOS_CENTERED_MASK as i32;
  pub const UNDEFINED_MASK: i32 = SDL_WINDOWPOS_UNDEFINED_MASK as i32;

  #[inline]
  pub(crate) const fn new(inner: *mut SDL_Window) -> Self {
    Self {
      inner,
      _marker: PhantomData,
    }
  }

  #[inline]
  pub(crate) fn as_ptr(&self) -> *mut SDL_Window {
    self.inner
  }

  pub fn try_into_renderer(
    self,
    flags: RendererFlags,
  ) -> Result<Renderer<'a>, (Self, &'static str)> {
    let renderer: *mut SDL_Renderer = unsafe { SDL_CreateRenderer(self.inner, -1, flags.bits()) };

    if renderer.is_null() {
      Err((self, error()))
    } else {
      Ok(Renderer::new(renderer, self))
    }
  }

  pub fn try_into_opengl(self) -> Result<OpenGL<'a>, (Self, &'static str)> {
    let context: SDL_GLContext = unsafe { SDL_GL_CreateContext(self.inner) };

    if context.is_null() {
      Err((self, error()))
    } else {
      Ok(OpenGL::new(context, self))
    }
  }
}

impl<'a> Drop for Window<'a> {
  fn drop(&mut self) {
    unsafe {
      SDL_DestroyWindow(self.inner);
    }
  }
}
