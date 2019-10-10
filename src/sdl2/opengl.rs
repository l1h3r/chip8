use core::ops::Deref;

use crate::sdl2::error;
use crate::sdl2::GLattr;
use crate::sdl2::SDL_GLContext;
use crate::sdl2::SDL_GL_DeleteContext;
use crate::sdl2::SDL_GL_GetAttribute;
use crate::sdl2::SDL_GL_SetAttribute;
use crate::sdl2::Window;

#[derive(Debug)]
pub struct OpenGL<'a> {
  context: SDL_GLContext,
  window: Window<'a>,
}

impl<'a> OpenGL<'a> {
  pub unsafe fn get_attr(attr: GLattr) -> Result<i32, &'static str> {
    let mut result: i32 = 0;

    if SDL_GL_GetAttribute(attr, &mut result) == 0 {
      Ok(result)
    } else {
      Err(error())
    }
  }

  pub unsafe fn set_attr(attr: GLattr, value: i32) -> Result<(), &'static str> {
    if SDL_GL_SetAttribute(attr, value) == 0 {
      Ok(())
    } else {
      Err(error())
    }
  }

  #[inline]
  pub(crate) const fn new(context: SDL_GLContext, window: Window<'a>) -> Self {
    Self { context, window }
  }
}

impl<'a> Drop for OpenGL<'a> {
  fn drop(&mut self) {
    unsafe {
      SDL_GL_DeleteContext(self.context);
    }
  }
}

impl<'a> Deref for OpenGL<'a> {
  type Target = Window<'a>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.window
  }
}
