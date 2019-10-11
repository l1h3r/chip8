use core::marker::PhantomData;
use core::ffi::c_void;

use crate::sdl2::SDL_MapRGB;
use crate::sdl2::SDL_SetColorKey;
use crate::sdl2::SDL_PixelFormat;
use crate::sdl2::SDL_LoadBMP_RW;
use crate::sdl2::SDL_RWFromConstMem;
use crate::sdl2::error;
use crate::sdl2::SDL_Surface;
use crate::sdl2::SDLToken;
use crate::sdl2::SDL_FreeSurface;

#[derive(Debug)]
#[repr(transparent)]
pub struct Surface<'a> {
  inner: *mut SDL_Surface,
  _marker: PhantomData<&'a SDLToken>,
}

impl<'a> Surface<'a> {
  #[inline]
  pub(crate) const fn new(inner: *mut SDL_Surface) -> Self {
    Self {
      inner,
      _marker: PhantomData,
    }
  }

  pub fn from_const_bytes(bytes: &'static [u8]) -> Result<Self, &'static str> {
    let ptr: *const c_void = bytes.as_ptr() as *const c_void;
    let len: i32 = bytes.len() as i32;

    let inner: *mut SDL_Surface = unsafe {
      SDL_LoadBMP_RW(SDL_RWFromConstMem(ptr, len), 1)
    };

    if inner.is_null() {
      Err(error())
    } else {
      Ok(Self::new(inner))
    }
  }

  #[inline]
  pub(crate) fn as_ptr(&self) -> *mut SDL_Surface {
    self.inner
  }

  #[inline]
  pub fn w(&self) -> i32 {
    unsafe { (*self.inner).w }
  }

  #[inline]
  pub fn h(&self) -> i32 {
    unsafe { (*self.inner).h }
  }

  #[inline]
  pub fn pitch(&self) -> i32 {
    unsafe { (*self.inner).pitch }
  }

  #[inline]
  pub fn format(&self) -> *mut SDL_PixelFormat {
    unsafe { (*self.inner).format }
  }

  #[inline]
  pub fn set_color(&self, r: u8, g: u8, b: u8) {
    self.mod_color(1, r, g, b);
  }

  #[inline]
  pub fn unset_color(&self, r: u8, g: u8, b: u8) {
    self.mod_color(0, r, g, b);
  }

  fn mod_color(&self, flag: i32, r: u8, g: u8, b: u8) {
    let pixel: u32 = unsafe {
      SDL_MapRGB(self.format(), r, g, b)
    };

    try_sdl2!(SDL_SetColorKey, self.inner, flag, pixel);
  }
}

impl<'a> Drop for Surface<'a> {
  fn drop(&mut self) {
    unsafe {
      SDL_FreeSurface(self.inner);
    }
  }
}

