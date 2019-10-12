use core::ffi::c_void;
use core::marker::PhantomData;

use crate::sdl2::SDLToken;
use crate::sdl2::SDL_CloseAudioDevice;
use crate::sdl2::SDL_GetQueuedAudioSize;
use crate::sdl2::SDL_PauseAudioDevice;
use crate::sdl2::SDL_QueueAudio;

#[derive(Debug)]
#[repr(transparent)]
pub struct AudioDevice<'a> {
  inner: u32,
  _marker: PhantomData<&'a SDLToken>,
}

impl<'a> AudioDevice<'a> {
  #[inline]
  pub(crate) const fn new(inner: u32) -> Self {
    Self {
      inner,
      _marker: PhantomData,
    }
  }

  #[inline]
  pub fn pause(&self) {
    unsafe {
      SDL_PauseAudioDevice(self.inner, 1);
    }
  }

  #[inline]
  pub fn resume(&self) {
    unsafe {
      SDL_PauseAudioDevice(self.inner, 0);
    }
  }

  pub fn queue<T>(&self, data: &[T]) {
    try_sdl2!(
      SDL_QueueAudio,
      self.inner,
      data.as_ptr() as *const c_void,
      data.len() as u32,
    );
  }

  pub fn size(&self) -> u32 {
    unsafe { SDL_GetQueuedAudioSize(self.inner) }
  }
}

impl<'a> Drop for AudioDevice<'a> {
  fn drop(&mut self) {
    unsafe {
      SDL_CloseAudioDevice(self.inner);
    }
  }
}
