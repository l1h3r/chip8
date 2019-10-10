use core::mem::transmute;

use crate::sdl2::SDLK_Keycode;
use crate::sdl2::SDL_Button;
use crate::sdl2::SDL_Event;
use crate::sdl2::SDL_EventType;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub enum Event {
  Quit,
  KeyDown(SDLK_Keycode),
  KeyUp(SDLK_Keycode),
  MouseDown(SDL_Button, i32, i32),
  MouseUp(SDL_Button, i32, i32),
  MouseMove(i32, i32),
  MouseWheel(i32, i32),
  TouchDown(i32, i32),
  TouchUp(i32, i32),
  TouchMove(i32, i32),
}

impl Event {
  pub unsafe fn from_sdl_event(event: SDL_Event) -> Option<Self> {
    match transmute(event.type_) {
      SDL_EventType::QUIT => Some(Self::Quit),
      SDL_EventType::KEYDOWN => Some(Self::KeyDown(transmute(event.key.keysym.sym))),
      SDL_EventType::KEYUP => Some(Self::KeyUp(transmute(event.key.keysym.sym))),
      SDL_EventType::MOUSEBUTTONDOWN => {
        Some(Self::MouseDown(transmute(event.button.button as u32), 0, 0))
      }
      SDL_EventType::MOUSEBUTTONUP => {
        Some(Self::MouseUp(transmute(event.button.button as u32), 0, 0))
      }
      SDL_EventType::MOUSEMOTION => Some(Self::MouseMove(event.motion.x, event.motion.y)),
      SDL_EventType::MOUSEWHEEL => Some(Self::MouseWheel(event.wheel.x, event.wheel.y)),
      SDL_EventType::FINGERDOWN => Some(Self::TouchDown(
        event.tfinger.x.round() as i32,
        event.tfinger.y.round() as i32,
      )),
      SDL_EventType::FINGERUP => Some(Self::TouchUp(
        event.tfinger.x.round() as i32,
        event.tfinger.y.round() as i32,
      )),
      SDL_EventType::FINGERMOTION => Some(Self::TouchMove(
        event.tfinger.x.round() as i32,
        event.tfinger.y.round() as i32,
      )),
      _ => None,
    }
  }
}
