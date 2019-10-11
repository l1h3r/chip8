use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;

use crate::chip8::Mode;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Args<'a> {
  pub debug: bool,
  pub mode: Mode,
  pub rom: &'a str,
}

impl<'a> Args<'a> {
  #[inline(always)]
  pub const fn new() -> Self {
    Self {
      debug: false,
      mode: Mode::CHIP,
      rom: "",
    }
  }

  pub fn parse(argc: i32, argv: *const *const u8) -> Self {
    let mut args: Self = Self::new();
    let mut mflag: bool = false;

    for index in 1..argc as usize {
      unsafe {
        let arg: *const *const u8 = argv.add(index);
        let size: usize = strlen(*arg);
        let string: &str = strparse(*arg, size);

        match string {
          "--debug" | "-d" => args.debug = true,
          "--mode" | "-m" => mflag = true,
          "SCHIP" if mflag => args.mode = Mode::SCHIP,
          "CHIP" if mflag => args.mode = Mode::CHIP,
          _ if args.rom.is_empty() => args.rom = string,
          _ => println!("[x] Invalid Argument: {}", string),
        }
      }
    }

    args
  }
}

// =============================================================================
// Parsing
// TODO: Find a nice no_std crate
// =============================================================================

unsafe fn strlen(ptr: *const u8) -> usize {
  let mut byte: *const u8 = ptr;
  let mut size: usize = 0;

  while *byte != b'\0' {
    size += 1;
    byte = byte.add(1);
  }

  size
}

#[inline]
unsafe fn strparse<'a>(ptr: *const u8, length: usize) -> &'a str {
  from_utf8_unchecked(from_raw_parts(ptr, length))
}
