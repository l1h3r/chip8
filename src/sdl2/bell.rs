use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FResult;
use core::marker::PhantomData;
use core::ptr::null_mut;

use crate::sdl2::AudioDevice;
use crate::sdl2::SDLToken;
use crate::sdl2::SDL_AudioSpec;
use crate::sdl2::AUDIO_S8;

const FREQUENCY: i32 = 48000;
const SAMPLES: usize = ((FREQUENCY as f32 / 50.0) / 100.0) as usize * 100;

#[repr(C)]
pub struct AudioBell<'a> {
  device: AudioDevice<'a>,
  buffer: [i8; SAMPLES],
  _marker: PhantomData<&'a SDLToken>,
}

impl<'a> AudioBell<'a> {
  pub fn new(token: &'a SDLToken) -> Result<Self, &'static str> {
    let device: AudioDevice = token.open_audio_device(SDL_AudioSpec {
      freq: FREQUENCY,
      format: AUDIO_S8 as u16,
      channels: 1,
      silence: 0,
      samples: SAMPLES as u16,
      padding: 0,
      size: 0,
      callback: None,
      userdata: null_mut(),
    })?;

    let mut buffer: [i8; SAMPLES] = [0; SAMPLES];

    for (index, byte) in buffer.iter_mut().enumerate() {
      *byte = if index % 100 < 50 { 25 } else { -25 };
    }

    Ok(Self {
      buffer,
      device,
      _marker: PhantomData,
    })
  }

  pub fn beep(&self) {
    if self.device.size() < SAMPLES as u32 {
      self.device.pause();
      self.device.queue(&self.buffer);
      self.device.resume();
    }
  }
}

impl<'a> Debug for AudioBell<'a> {
  fn fmt(&self, f: &mut Formatter) -> FResult {
    f.debug_struct("AudioBell")
      .field("device", &self.device)
      .finish()
  }
}

impl<'a> Drop for AudioBell<'a> {
  fn drop(&mut self) {
    self.device.pause();
  }
}
