use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;

use crate::sdl2::SDL_GetError;
use crate::sdl2::SDL_strlen;

unsafe fn stringify(ptr: *const i8) -> &'static str {
  let size: usize = SDL_strlen(ptr);
  let bytes: &[u8] = from_raw_parts(ptr as *const u8, size);

  from_utf8_unchecked(bytes)
}

pub fn error() -> &'static str {
  unsafe { stringify(SDL_GetError()) }
}
