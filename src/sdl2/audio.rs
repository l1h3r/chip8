use core::f32::consts::PI;
use core::ffi::c_void;
use core::i16::MAX;
use core::marker::PhantomData;

use crate::sdl2::SDLToken;
use crate::sdl2::SDL_CloseAudioDevice;
use crate::sdl2::SDL_PauseAudioDevice;
use crate::sdl2::SDL_QueueAudio;

#[derive(Debug)]
#[repr(transparent)]
pub struct AudioDevice<'a> {
  inner: u32,
  _marker: PhantomData<&'a SDLToken>,
}

impl<'a> AudioDevice<'a> {
  pub const AMPLITUDE: f32 = 0.65 * MAX as f32;
  pub const CHANNELS: u8 = 1;
  pub const FREQUENCY: i32 = 44100;
  pub const SAMPLES: u16 = 1024;

  const BUFFER: usize = (Self::FREQUENCY as usize) / 8;
  const C6: f32 = 1046.50; // https://pages.mtu.edu/~suits/notefreqs.html

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

  pub fn beep(&self) {
    // TODO: Below crashes rustc
    // let mut data: [i16; Self::BUFFER] = [0; Self::BUFFER];
    let mut data: Vec<i16> = vec![0; Self::BUFFER];

    for (index, frequency) in data.iter_mut().enumerate() {
      let sine: f32 = ((2.0 * PI * index as f32 * Self::C6) / Self::FREQUENCY as f32).sin();

      *frequency = (Self::AMPLITUDE * sine) as i16;
    }

    self.queue(&data);
    self.resume();
  }
}

impl<'a> Drop for AudioDevice<'a> {
  fn drop(&mut self) {
    unsafe {
      SDL_CloseAudioDevice(self.inner);
    }
  }
}
