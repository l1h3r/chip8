use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use crate::sdl2::error;
use crate::sdl2::Event;
use crate::sdl2::SDL_CreateWindow;
use crate::sdl2::SDL_Delay;
use crate::sdl2::SDL_Event;
use crate::sdl2::SDL_GetTicks;
use crate::sdl2::SDL_Init;
use crate::sdl2::SDL_PollEvent;
use crate::sdl2::SDL_Quit;
use crate::sdl2::SDL_Window;
use crate::sdl2::Window;
use crate::sdl2::WindowFlags;
use crate::sdl2::SDL_INIT_EVERYTHING;

static SDL2_INIT: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct SDLToken {
  _marker: PhantomData<*mut u8>,
}

impl SDLToken {
  pub fn init() -> Result<Self, &'static str> {
    if SDL2_INIT.swap(true, Ordering::SeqCst) {
      Err("SDL2 Initialized")
    } else if unsafe { SDL_Init(SDL_INIT_EVERYTHING) } == 0 {
      Ok(Self {
        _marker: PhantomData,
      })
    } else {
      SDL2_INIT.store(false, Ordering::SeqCst);
      Err(error())
    }
  }

  pub fn create_window(
    &self,
    title: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    flags: WindowFlags,
  ) -> Result<Window, &'static str> {
    let window: *mut SDL_Window =
      unsafe { SDL_CreateWindow(title.as_ptr() as *const i8, x, y, w, h, flags.bits()) };

    if window.is_null() {
      Err(error())
    } else {
      Ok(Window::new(window))
    }
  }

  pub fn poll(&self) -> Option<Event> {
    let mut event: SDL_Event = SDL_Event { type_: 0 };

    unsafe {
      if SDL_PollEvent(&mut event) == 1 {
        Event::from_sdl_event(event)
      } else {
        None
      }
    }
  }

  #[inline(always)]
  pub fn ticks() -> u32 {
    unsafe { SDL_GetTicks() }
  }

  #[inline(always)]
  pub fn delay(time: u32) {
    unsafe { SDL_Delay(time) }
  }
}

impl Drop for SDLToken {
  fn drop(&mut self) {
    unsafe { SDL_Quit() }

    SDL2_INIT.store(false, Ordering::SeqCst);
  }
}
