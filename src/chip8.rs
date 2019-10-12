use core::cell::Cell;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FResult;
use core::ptr::null_mut;
use std::fs::read;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::instruction::Instruction;
use crate::sdl2::Event;
use crate::sdl2::SDLK_Keycode;
use crate::sdl2::SDL_Context;
use crate::sdl2::SDL_Rect;

pub const REGISTERS: usize = 0x10;
pub const STACK: usize = 0x10;
pub const RAM: usize = 0xFFF;

pub const FONT_BASE: usize = 0x050;
pub const PROG_BASE_VIP: usize = 0x200; // COSMAC VIP
pub const PROG_BASE_ETI: usize = 0x200; // ETI 660

pub const VF: usize = REGISTERS - 0x1;

// TODO: Support 64x48 size (ETI 660)
// TODO: Support 64x64 size (ETI 660)
// TODO: Support 128x64 size (CHIP-48)
pub const W: usize = 0x40;
pub const H: usize = 0x20;

pub const CW: i32 = 5; // debug font char width
pub const CH: i32 = 7; // debug font char height

pub const MAX_SPEED: u64 = 15000;
pub const MIN_SPEED: u64 = 100;
pub const INCR_SPEED: u64 = 200;
pub const DEFAULT_SPEED: u64 = 700;

static FONT: [u8; 0x50] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
  CHIP,  // CHIP-8
  SCHIP, // CHIP-48
}

macro_rules! blankify {
  ($array:expr) => {
    for byte in $array {
      *byte = 0;
    }
  };
}

#[repr(C)]
pub struct Chip8 {
  pub delay: u8,              // delay timer - decremented at a rate of 60Hz
  pub sound: u8,              // sound timer - decremented at a rate of 60Hz
  pub pc: u16,                // program counter - stores the currently executing address
  pub sp: u8,                 // stack pointer - points to the topmost level of the stack
  pub reg_i: u16,             // register I - generally used to store memory addresses
  pub reg_v: [u8; REGISTERS], // registers V0-VF - general purpose registers
  pub stack: [u16; STACK],    // stack (should actually be part memory)
  pub display: [u8; W * H],   // display buffer
  pub memory: [u8; RAM],      // memory buffer
  pub keys: u16,              // keypad state
  pub wait: *mut u8,

  pub cycles: u64,
  pub speed: u64,
  pub time: u64,

  pub mode: Mode,
  pub paused: bool,
  pub render: bool,
  pub history: History,
}

impl Chip8 {
  #[inline(always)]
  pub fn new() -> Self {
    unsafe {
      libc::srand(libc::time(0 as *mut i64) as u32);
    }

    Self {
      delay: 0,
      sound: 0,
      pc: 0,
      sp: 0,
      reg_i: 0,
      reg_v: [0; REGISTERS],
      stack: [0; STACK],
      display: [0; W * H],
      memory: [0; RAM],
      keys: 0,
      wait: null_mut(),

      cycles: 0,
      speed: DEFAULT_SPEED,
      time: 0,

      mode: Mode::CHIP,
      paused: false,
      render: false,
      history: History::new(),
    }
  }

  #[inline(always)]
  pub fn mode(&self, mode: Mode) -> bool {
    self.mode == mode
  }

  #[inline(always)]
  pub fn next(&mut self) {
    self.pc += 2;
  }

  #[inline(always)]
  pub fn keypress(&mut self, key: u8) {
    self.keys |= 0x1 << key;

    if !self.wait.is_null() {
      unsafe {
        *self.wait = key;
      }

      self.wait = null_mut();
    }
  }

  #[inline(always)]
  pub fn keyrelease(&mut self, key: u8) {
    self.keys &= !(0x1 << key);
  }

  #[inline(always)]
  pub fn key_pressed(&self, key: u8) -> bool {
    self.keys >> key & 0x1 == 0x1
  }

  pub fn faster(&mut self) {
    if self.speed < MAX_SPEED {
      self.cycles = 0;
      self.speed += INCR_SPEED;
      self.time = time();
    }
  }

  pub fn slower(&mut self) {
    if self.speed > MIN_SPEED {
      self.cycles = 0;
      self.speed -= INCR_SPEED;
      self.time = time();
    }
  }

  pub fn rand(&self) -> u8 {
    unsafe { (libc::rand() as f32 / libc::RAND_MAX as f32 * 255.0) as u8 }
  }

  pub fn clear(&mut self) {
    blankify!(self.display.iter_mut());
    self.render = true;
  }

  pub fn draw(&mut self, x: usize, y: usize, n: usize) {
    self.reg_v[VF] = 0x0;

    for yline in 0..n {
      let pixel: u8 = self.memory[self.reg_i as usize + yline];

      for xline in 0..8 {
        if (pixel & (0x80 >> xline)) != 0 {
          let index: usize = (y + yline) * 64 + (x + xline);

          if let Some(pixel) = self.display.get_mut(index) {
            if *pixel == 0x1 {
              self.reg_v[VF] = 0x1;
            }

            *pixel ^= 0x1;
          }
        }
      }
    }

    self.render = true;
  }

  #[inline]
  pub fn load_vip(&mut self, path: &str) {
    self.load(path, false)
  }

  #[inline]
  pub fn load_eti(&mut self, path: &str) {
    self.load(path, true)
  }

  // TODO: Propagate errors
  pub fn load(&mut self, path: &str, eti: bool) {
    let base: usize = if eti { PROG_BASE_ETI } else { PROG_BASE_VIP };

    blankify!(self.reg_v.iter_mut());
    blankify!(self.stack.iter_mut());
    blankify!(self.display.iter_mut());
    blankify!(self.memory.iter_mut());

    self.delay = 0;
    self.sound = 0;
    self.pc = base as u16;
    self.sp = 0;
    self.reg_i = 0;
    self.reg_v = [0; REGISTERS];
    self.stack = [0; STACK];
    self.display = [0; W * H];
    self.memory = [0; RAM];
    self.keys = 0;
    self.render = false;

    self.cycles = 0;
    self.time = time();

    self.load_memory(FONT_BASE, &FONT);

    if let Ok(buffer) = read(path) {
      self.load_memory(base, &buffer);
    }
  }

  // TODO: timing
  // - clock ticks at 1
  // - delay/sound ticks at 1000 / 60
  pub fn start(&mut self, context: &SDL_Context) {
    'running: loop {
      if self.poll(context) {
        break 'running;
      }

      let count: u64 = (time() - self.time) * self.speed / 1000000000;

      if self.paused {
        self.cycles = count;
      } else {
        while self.cycles < count {
          self.step(context);

          if !self.wait.is_null() {
            self.cycles = count;
          }
        }
      }

      self.render(context);

      sleep(Duration::from_millis(1));
    }
  }

  fn toggle_pause(&mut self) {
    self.paused = !self.paused;
  }

  fn step(&mut self, context: &SDL_Context) {
    if self.wait.is_null() {
      let opcode: u16 = word(&self.memory, self.pc);

      self.next();
      self.exec(opcode);
      self.tick_delay();
      self.tick_sound(context);
    }
  }

  fn poll(&mut self, context: &SDL_Context) -> bool {
    while let Some(event) = context.token.poll() {
      // TODO: Something nicer than a giant 'match'
      match event {
        Event::Quit => return true,
        Event::KeyDown(SDLK_Keycode::SDLK_ESCAPE) => return true,
        Event::KeyDown(SDLK_Keycode::SDLK_RETURN) if self.paused => self.step(context),
        Event::KeyDown(SDLK_Keycode::SDLK_UP) => self.faster(),
        Event::KeyDown(SDLK_Keycode::SDLK_DOWN) => self.slower(),
        Event::KeyDown(SDLK_Keycode::SDLK_SPACE) => self.toggle_pause(),
        Event::KeyDown(SDLK_Keycode::SDLK_1) => self.keypress(0x1),
        Event::KeyDown(SDLK_Keycode::SDLK_2) => self.keypress(0x2),
        Event::KeyDown(SDLK_Keycode::SDLK_3) => self.keypress(0x3),
        Event::KeyDown(SDLK_Keycode::SDLK_4) => self.keypress(0xC),
        Event::KeyDown(SDLK_Keycode::SDLK_q) => self.keypress(0x4),
        Event::KeyDown(SDLK_Keycode::SDLK_w) => self.keypress(0x5),
        Event::KeyDown(SDLK_Keycode::SDLK_e) => self.keypress(0x6),
        Event::KeyDown(SDLK_Keycode::SDLK_r) => self.keypress(0xD),
        Event::KeyDown(SDLK_Keycode::SDLK_a) => self.keypress(0x7),
        Event::KeyDown(SDLK_Keycode::SDLK_s) => self.keypress(0x8),
        Event::KeyDown(SDLK_Keycode::SDLK_d) => self.keypress(0x9),
        Event::KeyDown(SDLK_Keycode::SDLK_f) => self.keypress(0xE),
        Event::KeyDown(SDLK_Keycode::SDLK_z) => self.keypress(0xA),
        Event::KeyDown(SDLK_Keycode::SDLK_x) => self.keypress(0x0),
        Event::KeyDown(SDLK_Keycode::SDLK_c) => self.keypress(0xB),
        Event::KeyDown(SDLK_Keycode::SDLK_v) => self.keypress(0xF),
        Event::KeyUp(SDLK_Keycode::SDLK_1) => self.keyrelease(0x1),
        Event::KeyUp(SDLK_Keycode::SDLK_2) => self.keyrelease(0x2),
        Event::KeyUp(SDLK_Keycode::SDLK_3) => self.keyrelease(0x3),
        Event::KeyUp(SDLK_Keycode::SDLK_4) => self.keyrelease(0xC),
        Event::KeyUp(SDLK_Keycode::SDLK_q) => self.keyrelease(0x4),
        Event::KeyUp(SDLK_Keycode::SDLK_w) => self.keyrelease(0x5),
        Event::KeyUp(SDLK_Keycode::SDLK_e) => self.keyrelease(0x6),
        Event::KeyUp(SDLK_Keycode::SDLK_r) => self.keyrelease(0xD),
        Event::KeyUp(SDLK_Keycode::SDLK_a) => self.keyrelease(0x7),
        Event::KeyUp(SDLK_Keycode::SDLK_s) => self.keyrelease(0x8),
        Event::KeyUp(SDLK_Keycode::SDLK_d) => self.keyrelease(0x9),
        Event::KeyUp(SDLK_Keycode::SDLK_f) => self.keyrelease(0xE),
        Event::KeyUp(SDLK_Keycode::SDLK_z) => self.keyrelease(0xA),
        Event::KeyUp(SDLK_Keycode::SDLK_x) => self.keyrelease(0x0),
        Event::KeyUp(SDLK_Keycode::SDLK_c) => self.keyrelease(0xB),
        Event::KeyUp(SDLK_Keycode::SDLK_v) => self.keyrelease(0xF),
        _ => {}
      }
    }

    false
  }

  fn exec(&mut self, opcode: u16) {
    if let Some(instruction) = Instruction::find(opcode) {
      (instruction.f)(opcode, self);

      self.cycles += 1;
      self.history.push(opcode);
    } else {
      panic!("Unexpected Opcode: {:#X}", opcode)
    }
  }

  fn tick_delay(&mut self) {
    if self.delay > 0 {
      self.delay -= 1;
    }
  }

  fn tick_sound(&mut self, context: &SDL_Context) {
    if self.sound > 0 {
      self.sound -= 1;
    }

    if self.sound != 0 {
      context.audio.beep();
    }
  }

  fn render(&mut self, context: &SDL_Context) {
    context.renderer.color(0, 0, 0);
    context.renderer.clear();
    context.renderer.color(255, 255, 255);

    self.render_frame(context, 8, 8, 640, 320);
    self.render_frame(context, 8 + 640 + 8, 8, 320 - 8, 320);
    self.render_frame(context, 8, 8 + 320 + 8, 640, 320);
    // self.render_frame(context, 8 + 640 + 8, 8, 320 - 8, 320);

    // TODO: Consider self.render
    self.render_display(context, 8, 8);

    self.render_debug(context, 8 + 640 + 8 + 4, 8 + 4);
    self.render_history(context, 8 + 4, 8 + 320 + 8 + 4);

    context.renderer.present();
  }

  fn render_frame(&self, context: &SDL_Context, x: i32, y: i32, w: i32, h: i32) {
    context.renderer.color(128, 128, 128);
    context.renderer.line(x, y, x + w, y);
    context.renderer.line(x, y, x, y + h);

    context.renderer.color(255, 255, 255);
    context.renderer.line(x, y + h, x + w, y + h);
    context.renderer.line(x + w, y, x + w, y + h);
  }

  fn render_display(&self, context: &SDL_Context, dx: i32, dy: i32) {
    for y in 0..H {
      for x in 0..W {
        if self.display[y * W + x] == 1 {
          context
            .renderer
            .fill_rect(dx + (x as i32 * 10), dy + (y as i32 * 10), 10, 10);
        }
      }
    }
  }

  fn render_debug(&self, context: &SDL_Context, dx: i32, dy: i32) {
    let x: Cell<i32> = Cell::new(dx);
    let y: Cell<i32> = Cell::new(dy);

    let line = |text: &str| {
      if !text.is_empty() {
        self.render_text(context, text, x.get(), y.get());
      }

      y.update(|y| y + 10);
    };

    for (index, value) in self.reg_v.iter().enumerate() {
      line(&format!("V{:x} = {:#04X}", index, value));
    }

    line("");
    line(&format!("Keys  = {:#018b}", self.keys));
    line("");
    line(&format!("Mode  = {:?}", self.mode));
    line(&format!("Speed = {}", self.speed));
    line(&format!("Cycle = {}", self.cycles));

    x.set(dx + 98);
    y.set(dy);

    for (index, value) in self.stack.iter().enumerate() {
      line(&format!("S{:x} = {:#04X}", index, value));
    }

    x.set(dx + 98 + 98);
    y.set(dy);

    line(&format!("I  = {:#04X}", self.reg_i));
    line("");
    line(&format!("PC = {:#04X}", self.pc));
    line(&format!("SP = {:#04X}", self.sp));
    line("");
    line(&format!("DT = {:#04X}", self.delay));
    line(&format!("ST = {:#04X}", self.sound));
  }

  fn render_history(&self, context: &SDL_Context, dx: i32, dy: i32) {
    let x: Cell<i32> = Cell::new(dx);
    let y: Cell<i32> = Cell::new(dy);

    let line = |text: &str| {
      if !text.is_empty() {
        self.render_text(context, text, x.get(), y.get());
      }

      y.update(|y| y + 10);
    };

    for opcode in self.history.window(31) {
      let instruction: &'static Instruction = Instruction::find(*opcode).unwrap();
      line(&format!("[{:#06X}] {}", opcode, instruction.name));
    }
  }

  fn render_text(&self, context: &SDL_Context, text: &str, x: i32, y: i32) {
    let mut source: SDL_Rect = SDL_Rect {
      x: 0,
      y: 0,
      w: CW,
      h: CH,
    };

    let mut output: SDL_Rect = SDL_Rect { x, y, w: CW, h: CH };

    for byte in text.bytes() {
      if byte > 32 && byte < 127 {
        source.x = (byte as i32 - 33) * 6;
        context.renderer.copy(context.texture, &source, &output);
      }

      output.x += CW + 2;
    }
  }

  fn load_memory(&mut self, address: usize, data: &[u8]) {
    self.memory[address..address + data.len()].copy_from_slice(data);
  }
}

impl Debug for Chip8 {
  fn fmt(&self, f: &mut Formatter) -> FResult {
    writeln!(f, "Chip8 >\n")?;
    writeln!(f, "[+] delay: {}", self.delay)?;
    writeln!(f, "[+] sound: {}", self.sound)?;
    writeln!(f, "[+] program counter: {}", &self.pc)?;
    writeln!(f, "[+] stack pointer: {}", &self.sp)?;
    writeln!(f, "[+] register (I): {}", &self.reg_i)?;
    writeln!(f, "[+] register (VX): {:?}", self.reg_v)?;
    writeln!(f, "[+] stack: {:?}", self.stack)?;
    writeln!(f, "")?;

    Ok(())
  }
}

fn word(memory: &[u8], address: u16) -> u16 {
  (memory[address as usize] as u16) << 8 | (memory[(address + 1) as usize] as u16)
}

fn time() -> u64 {
  match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(duration) => duration.as_nanos() as u64,
    Err(_) => 0,
  }
}
