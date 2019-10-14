use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::chip8::Chip8;
use crate::sdl2::Event;
use crate::sdl2::SDLK_Keycode;
use crate::sdl2::SDL_Context;
use crate::sdl2::SDL_Rect;

type FrameSpec = (i32, i32, i32, i32);

const HISTORY: usize = 0x20;

const PAD: i32 = 8;
const PAD2: i32 = PAD * 2;
const PAD3: i32 = PAD * 3;

const F1: FrameSpec = (PAD, PAD, 640, 320);
const F2: FrameSpec = (PAD2 + 640, PAD, 320, 320);
const F3: FrameSpec = (PAD, PAD2 + 320, 640, 320);
const F4: FrameSpec = (PAD2 + 640, PAD2 + 320, 320, 320);

const CW: i32 = 5; // font char width
const CH: i32 = 7; // font char height

fn time() -> u64 {
  match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(duration) => duration.as_nanos() as u64,
    Err(_) => 0,
  }
}

#[repr(C)]
pub struct ChipRunner {
  paused: bool,      // flag set if interpreter is halted by user-interaction
  cycles: u64,       // total cycles executed
  speed: u64,        // execution speed
  time: u64,         // execution timestamp
  hindex: i32,       // opcode history index
  history: Vec<u16>, // history of opcode execution
  chip8: Chip8,
}

impl ChipRunner {
  pub const SCALE: i32 = 10;
  pub const W: i32 = (Chip8::W as i32 * Self::SCALE) + 320 + PAD3;
  pub const H: i32 = (Chip8::H as i32 * Self::SCALE) + 320 + PAD3;

  pub fn new() -> Self {
    Self {
      paused: false,
      cycles: 0,
      speed: 700,
      time: 0,
      hindex: 0,
      history: Vec::with_capacity(HISTORY * 8),
      chip8: Chip8::new(),
    }
  }

  pub fn faster(&mut self) {
    if self.speed < 15000 {
      self.cycles = 0;
      self.speed += 200;
      self.time = time();
    }
  }

  pub fn slower(&mut self) {
    if self.speed > 100 {
      self.cycles = 0;
      self.speed -= 200;
      self.time = time();
    }
  }

  // TODO: timing
  // - clock ticks at 1
  // - delay/sound ticks at 1000 / 60
  pub fn run(&mut self, context: &SDL_Context) {
    'running: loop {
      if self.poll(context) {
        break 'running;
      }

      let count: u64 = (time() - self.time) * self.speed / 1_000_000_000;

      if self.paused {
        self.cycles = count;
      } else {
        while self.cycles < count {
          if let Some(opcode) = self.chip8.step() {
            self.push_history(opcode);
          }

          if self.chip8.is_waiting() {
            self.cycles = count;
          } else {
            self.cycles += 1;
          }
        }
      }

      self.render(context);

      sleep(Duration::from_millis(1));
    }
  }

  pub fn load(&mut self, path: &str, eti: bool) -> Result<(), &'static str> {
    self.reset();
    self.chip8.load(path, eti)
  }

  fn render(&mut self, context: &SDL_Context) {
    self.render_frame(context, F2);
    self.render_frame(context, F3);
    self.render_frame(context, F4);

    context.renderer.color(255, 255, 255);

    if self.chip8.render {
      self.render_frame(context, F1);
      self.render_display(context, F1.0, F1.1);
      self.chip8.render = false;
    }

    self.render_debug(context, F2.0 + 4, F2.1 + 4);
    self.render_history(context, F3.0 + 4, F3.1 + 4);

    context.renderer.present();
  }

  fn clear_frame(&self, context: &SDL_Context, spec: FrameSpec) {
    context.renderer.color(0, 0, 0);
    context.renderer.fill_rect(spec.0, spec.1, spec.2, spec.3);
  }

  fn render_frame(&self, context: &SDL_Context, spec: FrameSpec) {
    self.clear_frame(context, spec);

    context.renderer.color(128, 128, 128);

    context
      .renderer
      .line(spec.0, spec.1, spec.0 + spec.2, spec.1);

    context
      .renderer
      .line(spec.0, spec.1, spec.0, spec.1 + spec.3);

    context.renderer.color(255, 255, 255);

    context
      .renderer
      .line(spec.0, spec.1 + spec.3, spec.0 + spec.2, spec.1 + spec.3);

    context
      .renderer
      .line(spec.0 + spec.2, spec.1, spec.0 + spec.2, spec.1 + spec.3);
  }

  fn render_display(&self, context: &SDL_Context, dx: i32, dy: i32) {
    for (index, &pixel) in self.chip8.display.iter().enumerate() {
      if pixel == 1 {
        let x: i32 = dx + ((index % Chip8::W) as i32 * Self::SCALE);
        let y: i32 = dy + ((index / Chip8::W) as i32 * Self::SCALE);

        context.renderer.fill_rect(x, y, Self::SCALE, Self::SCALE);
      }
    }
  }

  fn render_debug(&self, context: &SDL_Context, dx: i32, dy: i32) {
    let mut lines: Lines = Lines::new(context, dx, dy);

    for (index, value) in self.chip8.reg_v.iter().enumerate() {
      lines.write(&format!("V{:x} = {:#04X}", index, value));
    }

    lines.write("");
    lines.write(&format!("Keys  = {:#018b}", self.chip8.keys));
    lines.write("");
    lines.write(&format!("Speed = {}", self.speed));
    lines.write(&format!("Cycle = {}", self.cycles));

    lines.set(dx + 98, dy);

    for (index, value) in self.chip8.stack.iter().enumerate() {
      lines.write(&format!("S{:x} = {:#04X}", index, value));
    }

    lines.set(dx + 98 + 98, dy);

    lines.write(&format!("I  = {:#04X}", self.chip8.reg_i));
    lines.write("");
    lines.write(&format!("PC = {:#04X}", self.chip8.pc));
    lines.write(&format!("SP = {:#04X}", self.chip8.sp));
    lines.write("");
    lines.write(&format!("DT = {:#04X}", self.chip8.delay));
    lines.write(&format!("ST = {:#04X}", self.chip8.sound));
  }

  fn render_history(&self, context: &SDL_Context, dx: i32, dy: i32) {
    let mut lines: Lines = Lines::new(context, dx, dy);

    for opcode in self.history_window(HISTORY as i32 - 1) {
      lines.write(&format!("[{:#06X}] {}", opcode, instruction(*opcode)));
    }
  }

  #[inline]
  fn reset(&mut self) {
    self.cycles = 0;
    self.time = time();
    self.hindex = 0;
    self.history.clear();
  }

  fn poll(&mut self, context: &SDL_Context) -> bool {
    while let Some(event) = context.token.poll() {
      match event {
        Event::Quit => return true,
        Event::KeyDown(SDLK_Keycode::SDLK_ESCAPE) => return true,
        Event::KeyDown(SDLK_Keycode::SDLK_UP) => self.faster(),
        Event::KeyDown(SDLK_Keycode::SDLK_DOWN) => self.slower(),
        Event::KeyDown(SDLK_Keycode::SDLK_SPACE) => self.paused = !self.paused,
        Event::KeyDown(SDLK_Keycode::SDLK_1) => self.chip8.keypress(0x1),
        Event::KeyDown(SDLK_Keycode::SDLK_2) => self.chip8.keypress(0x2),
        Event::KeyDown(SDLK_Keycode::SDLK_3) => self.chip8.keypress(0x3),
        Event::KeyDown(SDLK_Keycode::SDLK_4) => self.chip8.keypress(0xC),
        Event::KeyDown(SDLK_Keycode::SDLK_q) => self.chip8.keypress(0x4),
        Event::KeyDown(SDLK_Keycode::SDLK_w) => self.chip8.keypress(0x5),
        Event::KeyDown(SDLK_Keycode::SDLK_e) => self.chip8.keypress(0x6),
        Event::KeyDown(SDLK_Keycode::SDLK_r) => self.chip8.keypress(0xD),
        Event::KeyDown(SDLK_Keycode::SDLK_a) => self.chip8.keypress(0x7),
        Event::KeyDown(SDLK_Keycode::SDLK_s) => self.chip8.keypress(0x8),
        Event::KeyDown(SDLK_Keycode::SDLK_d) => self.chip8.keypress(0x9),
        Event::KeyDown(SDLK_Keycode::SDLK_f) => self.chip8.keypress(0xE),
        Event::KeyDown(SDLK_Keycode::SDLK_z) => self.chip8.keypress(0xA),
        Event::KeyDown(SDLK_Keycode::SDLK_x) => self.chip8.keypress(0x0),
        Event::KeyDown(SDLK_Keycode::SDLK_c) => self.chip8.keypress(0xB),
        Event::KeyDown(SDLK_Keycode::SDLK_v) => self.chip8.keypress(0xF),
        Event::KeyUp(SDLK_Keycode::SDLK_1) => self.chip8.keyrelease(0x1),
        Event::KeyUp(SDLK_Keycode::SDLK_2) => self.chip8.keyrelease(0x2),
        Event::KeyUp(SDLK_Keycode::SDLK_3) => self.chip8.keyrelease(0x3),
        Event::KeyUp(SDLK_Keycode::SDLK_4) => self.chip8.keyrelease(0xC),
        Event::KeyUp(SDLK_Keycode::SDLK_q) => self.chip8.keyrelease(0x4),
        Event::KeyUp(SDLK_Keycode::SDLK_w) => self.chip8.keyrelease(0x5),
        Event::KeyUp(SDLK_Keycode::SDLK_e) => self.chip8.keyrelease(0x6),
        Event::KeyUp(SDLK_Keycode::SDLK_r) => self.chip8.keyrelease(0xD),
        Event::KeyUp(SDLK_Keycode::SDLK_a) => self.chip8.keyrelease(0x7),
        Event::KeyUp(SDLK_Keycode::SDLK_s) => self.chip8.keyrelease(0x8),
        Event::KeyUp(SDLK_Keycode::SDLK_d) => self.chip8.keyrelease(0x9),
        Event::KeyUp(SDLK_Keycode::SDLK_f) => self.chip8.keyrelease(0xE),
        Event::KeyUp(SDLK_Keycode::SDLK_z) => self.chip8.keyrelease(0xA),
        Event::KeyUp(SDLK_Keycode::SDLK_x) => self.chip8.keyrelease(0x0),
        Event::KeyUp(SDLK_Keycode::SDLK_c) => self.chip8.keyrelease(0xB),
        Event::KeyUp(SDLK_Keycode::SDLK_v) => self.chip8.keyrelease(0xF),
        _ => {}
      }
    }

    false
  }

  fn history_window(&self, size: i32) -> &[u16] {
    let start: i32 = (self.hindex - size).max(0);

    if start + size > self.history.len() as i32 {
      &self.history[start as usize..]
    } else {
      &self.history[start as usize..(start + size) as usize]
    }
  }

  fn push_history(&mut self, opcode: u16) {
    let wrap: bool = self.hindex == self.history.len() as i32;

    self.history.push(opcode);

    while self.history.len() > (HISTORY * 8) {
      self.history.remove(0);
    }

    if wrap {
      self.hindex = self.history.len() as i32;
    }
  }
}

fn render_text(context: &SDL_Context, text: &str, x: i32, y: i32) {
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

#[derive(Debug)]
#[repr(C)]
pub struct Lines<'a, 'b, 'c> {
  x: i32,
  y: i32,
  context: &'c SDL_Context<'a, 'b>,
}

impl<'a, 'b, 'c> Lines<'a, 'b, 'c> {
  pub const fn new(context: &'c SDL_Context<'a, 'b>, x: i32, y: i32) -> Self {
    Self { x, y, context }
  }

  pub fn set(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }

  pub fn write(&mut self, text: &str) {
    if !text.is_empty() {
      render_text(self.context, text, self.x, self.y);
    }

    self.y += 10;
  }
}

fn instruction(opcode: u16) -> &'static str {
  match opcode & 0xFFFF {
    0x00E0 => "00E0 - CLS",
    0x00EE => "00EE - RET",
    _ => match opcode & 0xF000 {
      0x0000 => "0nnn - SYS addr",
      0x1000 => "1nnn - JP addr",
      0x2000 => "2nnn - CALL addr",
      0x3000 => "3xkk - SE Vx, byte",
      0x4000 => "4xkk - SNE Vx, byte",
      0x6000 => "6xkk - LD Vx, byte",
      0x7000 => "7xkk - ADD Vx, byte",
      0xA000 => "Annn - LD I, addr",
      0xB000 => "Bnnn - JP V0, addr",
      0xC000 => "Cxkk - RND Vx, byte",
      0xD000 => "Dxyn - DRW Vx, Vy, nibble",
      _ => match opcode & 0xF00F {
        0x5000 => "5xy0 - SE Vx, Vy",
        0x8000 => "8xy0 - LD Vx, Vy",
        0x8001 => "8xy1 - OR Vx, Vy",
        0x8002 => "8xy2 - AND Vx, Vy",
        0x8003 => "8xy3 - XOR Vx, Vy",
        0x8004 => "8xy4 - ADD Vx, Vy",
        0x8005 => "8xy5 - SUB Vx, Vy",
        0x8006 => "8xy6 - SHR Vx {, Vy}",
        0x8007 => "8xy7 - SUBN Vx, Vy",
        0x800E => "8xyE - SHL Vx {, Vy}",
        0x9000 => "9xy0 - SNE Vx, Vy",
        _ => match opcode & 0xF0FF {
          0xE09E => "Ex9E - SKP Vx",
          0xE0A1 => "ExA1 - SKNP Vx",
          0xF007 => "Fx07 - LD Vx, DT",
          0xF00A => "Fx0A - LD Vx, K",
          0xF015 => "Fx15 - LD DT, Vx",
          0xF018 => "Fx18 - LD ST, Vx",
          0xF01E => "Fx1E - ADD I, Vx",
          0xF029 => "Fx29 - LD F, Vx",
          0xF033 => "Fx33 - LD B, Vx",
          0xF055 => "Fx55 - LD [I], Vx",
          0xF065 => "Fx65 - LD Vx, [I]",
          _ => panic!("Unexpected Opcode: {:#06X}", opcode),
        },
      },
    },
  }
}
