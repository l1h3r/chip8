use core::ops::RangeFrom;
use core::ops::RangeInclusive;
use core::ops::RangeTo;
use core::ops::RangeToInclusive;
use core::ptr::null_mut;
use std::fs::read;

const REGISTERS: usize = 0x10;
const UFLAGS: usize = 0x8;
const STACK: usize = 0x10;
const RAM: usize = 0xFFF;
const VF: usize = REGISTERS - 0x1;

const PROG_BASE_VIP: usize = 0x200; // COSMAC VIP
const PROG_BASE_ETI: usize = 0x600; // ETI 660

const FONT: [u8; 5 * 16] = [
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

const XFONT: [u8; 10 * 10] = [
  0x00, 0xFF, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0xFF, // 0
  0x00, 0x08, 0x18, 0x28, 0x48, 0x08, 0x08, 0x08, 0x08, 0x7F, // 1
  0x00, 0xFF, 0x01, 0x01, 0x01, 0xFF, 0x80, 0x80, 0x80, 0xFF, // 2
  0x00, 0xFF, 0x01, 0x01, 0x01, 0xFF, 0x01, 0x01, 0x01, 0xFF, // 3
  0x00, 0x81, 0x81, 0x81, 0x81, 0xFF, 0x01, 0x01, 0x01, 0x0F, // 4
  0x00, 0xFF, 0x80, 0x80, 0x80, 0xFF, 0x01, 0x01, 0x01, 0xFF, // 5
  0x00, 0xFF, 0x80, 0x80, 0x80, 0xFF, 0x81, 0x81, 0x81, 0xFF, // 6
  0x00, 0xFF, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, // 7
  0x00, 0xFF, 0x81, 0x81, 0x81, 0xFF, 0x81, 0x81, 0x81, 0xFF, // 8
  0x00, 0xFF, 0x81, 0x81, 0x81, 0xFF, 0x01, 0x01, 0x01, 0xFF, // 9
];

fn rand() -> u8 {
  unsafe { (libc::rand() as f32 / libc::RAND_MAX as f32 * 255.0) as u8 }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
  CHIP,  // CHIP-8
  SCHIP, // CHIP-48
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Pitch {
  P8 = 8,
  P16 = 16,
}

#[repr(C)]
pub struct Chip8 {
  pub(crate) delay: u8,              // delay timer - decremented at a rate of 60Hz
  pub(crate) sound: u8,              // sound timer - decremented at a rate of 60Hz
  pub(crate) pc: u16,                // program counter - stores the currently executing address
  pub(crate) sp: u8,                 // stack pointer - points to the topmost level of the stack
  pub(crate) reg_i: u16,             // register I - generally used to store memory addresses
  pub(crate) reg_v: [u8; REGISTERS], // registers V0-VF - general purpose registers
  pub(crate) reg_u: [u8; UFLAGS],    // registers R0-RF - user-flag registers
  pub(crate) stack: [u16; STACK],    // stack values
  pub(crate) display: [u8; Chip8::W * Chip8::H], // display buffer
  pub(crate) memory: [u8; RAM],      // memory buffer
  pub(crate) keys: u16,              // keypad state
  pub(crate) wait: *mut u8,          // pointer to register awaiting keypress
  pub(crate) render: bool,           // flag set if interpreter requires rendering
  pub(crate) pitch: Pitch,
  pub(crate) mode: Mode,
}

impl Chip8 {
  pub const W: usize = 0x80;
  pub const H: usize = 0x40;

  pub fn new() -> Self {
    unsafe {
      libc::srand(libc::time(null_mut()) as u32);
    }

    Self {
      delay: 0,
      sound: 0,
      pc: 0,
      sp: 0,
      reg_i: 0,
      reg_v: [0; REGISTERS],
      reg_u: [0; UFLAGS],
      stack: [0; STACK],
      display: [0; Chip8::W * Chip8::H],
      memory: [0; RAM],
      keys: 0,
      wait: null_mut(),
      render: false,
      pitch: Pitch::P8,
      mode: Mode::CHIP,
    }
  }

  #[inline(always)]
  pub fn mode(&mut self, mode: Mode) {
    self.mode = mode;
  }

  #[inline(always)]
  pub fn is_mode(&self, mode: Mode) -> bool {
    self.mode == mode
  }

  pub fn load(&mut self, path: &str, eti: bool) -> Result<(), &'static str> {
    self.reset(eti);

    self.write(0, &FONT);
    self.write(FONT.len(), &XFONT);

    read(path)
      .map(|buffer| self.write(Self::base(eti), &buffer))
      .map_err(|_| "Invalid ROM")
  }

  pub fn step(&mut self) -> Option<u16> {
    if self.wait.is_null() {
      let opcode: u16 = self.read(self.pc as usize);

      self.pc += 2;

      self.exec(opcode);

      if self.delay > 0 {
        self.delay -= 1;
      }

      if self.sound > 0 {
        self.sound -= 1;

        if self.sound != 0 {
          println!("BEEP!");
        }
      }

      return Some(opcode);
    }

    None
  }

  pub fn exec(&mut self, opcode: u16) {
    let schip: bool = self.is_mode(Mode::SCHIP);

    if opcode == 0x00E0 {
      self.cls();
    } else if opcode == 0x00EE {
      self.ret();
    } else if opcode == 0x00FB && schip {
      self.scr();
    } else if opcode == 0x00FC && schip {
      self.scl();
    } else if opcode == 0x00FD && schip {
      self.exit();
    } else if opcode == 0x00FE && schip {
      self.low();
    } else if opcode == 0x00FF && schip {
      self.high();
    } else if opcode & 0xFFF0 == 0x00B0 && schip {
      self.scu_nibble(n!(opcode));
    } else if opcode & 0xFFF0 == 0x00C0 && schip {
      self.scd_nibble(n!(opcode));
    } else if opcode & 0xF000 == 0x0000 {
      self.sys_addr(nnn!(opcode));
    } else if opcode & 0xF000 == 0x1000 {
      self.jp_addr(nnn!(opcode));
    } else if opcode & 0xF000 == 0x2000 {
      self.call_addr(nnn!(opcode));
    } else if opcode & 0xF000 == 0x3000 {
      self.se_vx_byte(x!(opcode), kk!(opcode));
    } else if opcode & 0xF000 == 0x4000 {
      self.sne_vx_byte(x!(opcode), kk!(opcode));
    } else if opcode & 0xF00F == 0x5000 {
      self.se_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF000 == 0x6000 {
      self.ld_vx_byte(x!(opcode), kk!(opcode));
    } else if opcode & 0xF000 == 0x7000 {
      self.add_vx_byte(x!(opcode), kk!(opcode));
    } else if opcode & 0xF00F == 0x8000 {
      self.ld_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x8001 {
      self.or_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x8002 {
      self.and_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x8003 {
      self.xor_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x8004 {
      self.add_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x8005 {
      self.sub_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x8006 {
      self.shr_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x8007 {
      self.subn_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x800E {
      self.shl_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF00F == 0x9000 {
      self.sne_vx_vy(x!(opcode), y!(opcode));
    } else if opcode & 0xF000 == 0xA000 {
      self.ld_i_addr(nnn!(opcode));
    } else if opcode & 0xF000 == 0xB000 {
      self.jp_v0_addr(nnn!(opcode));
    } else if opcode & 0xF000 == 0xC000 {
      self.rnd_vx_byte(x!(opcode), kk!(opcode));
    } else if opcode & 0xF00F == 0xD000 && schip {
      self.drw_vx_vy_0(x!(opcode), y!(opcode));
    } else if opcode & 0xF000 == 0xD000 {
      self.drw_vx_vy_nibble(x!(opcode), y!(opcode), n!(opcode));
    } else if opcode & 0xF0FF == 0xE09E {
      self.skp_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xE0A1 {
      self.sknp_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF007 {
      self.ld_vx_dt(x!(opcode));
    } else if opcode & 0xF0FF == 0xF00A {
      self.ld_vx_k(x!(opcode));
    } else if opcode & 0xF0FF == 0xF015 {
      self.ld_dt_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF018 {
      self.ld_st_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF01E {
      self.add_i_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF029 {
      self.ld_f_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF030 && schip {
      self.ld_hf_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF033 {
      self.ld_b_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF055 {
      self.ld_i_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF065 {
      self.ld_vx_i(x!(opcode));
    } else if opcode & 0xF0FF == 0xF075 && schip {
      self.ld_r_vx(x!(opcode));
    } else if opcode & 0xF0FF == 0xF085 && schip {
      self.ld_vx_r(x!(opcode));
    } else {
      panic!("Unexpected Opcode: {:#06X}", opcode);
    }
  }

  pub fn keypress(&mut self, key: u8) {
    self.keys |= 0x1 << key;

    if self.is_waiting() {
      unsafe {
        *self.wait = key;
      }

      self.wait = null_mut();
    }
  }

  pub fn keyrelease(&mut self, key: u8) {
    self.keys &= !(0x1 << key);
  }

  #[inline(always)]
  pub fn key_pressed(&self, key: u8) -> bool {
    self.keys >> key & 0x1 == 0x1
  }

  #[inline]
  pub fn is_waiting(&self) -> bool {
    !self.wait.is_null()
  }

  // ===========================================================================
  // Utilities
  // ===========================================================================

  fn reset(&mut self, eti: bool) {
    blankify!(self.reg_v.iter_mut());
    blankify!(self.reg_u.iter_mut());
    blankify!(self.stack.iter_mut());
    blankify!(self.display.iter_mut());
    blankify!(self.memory.iter_mut());

    self.pc = Self::base(eti) as u16;
    self.sp = 0;

    self.delay = 0;
    self.sound = 0;

    self.reg_i = 0;

    self.keys = 0;
    self.wait = null_mut();

    self.render = false;
    self.pitch = Pitch::P8;
  }

  fn read(&self, address: usize) -> u16 {
    debug_assert!(address < RAM, "[x] Read Word Overflow");
    (self.memory[address] as u16) << 8 | self.memory[address + 1] as u16
  }

  fn write(&mut self, address: usize, data: &[u8]) {
    debug_assert!(address + data.len() < RAM, "[x] Write Overflow");
    self.memory[address..address + data.len()].copy_from_slice(data);
  }

  #[inline(always)]
  fn base(eti: bool) -> usize {
    if eti {
      PROG_BASE_ETI
    } else {
      PROG_BASE_VIP
    }
  }

  // ===========================================================================
  // Instructions
  // ===========================================================================

  // Clears the screen.
  fn cls(&mut self) { // 00E0 - CLS
    blankify!(self.display.iter_mut());
    self.render = true;
  }

  // Returns from a subroutine.
  fn ret(&mut self) { // 00EE - RET
    self.sp -= 1;
    self.pc = self.stack[self.sp as usize];
  }

  fn sys_addr(&mut self, nnn: u16) { // 0nnn - SYS addr
    println!("TODO: Jump to a machine code routine at nnn({}).", nnn);
  }

  // Jumps to address NNN.
  fn jp_addr(&mut self, nnn: u16) { // 1nnn - JP addr
    self.pc = nnn;
  }

  // Calls subroutine at NNN.
  fn call_addr(&mut self, nnn: u16) { // 2nnn - CALL addr
    self.stack[self.sp as usize] = self.pc;
    self.sp += 1;
    self.pc = nnn;
  }

  // Skips the next instruction if VX equals NN.
  fn se_vx_byte(&mut self, x: u8, kk: u8) { // 3xkk - SE Vx, byte
    if self.reg_v[x as usize] == kk {
      self.pc += 2;
    }
  }

  // Skips the next instruction if VX doesn't equal NN.
  fn sne_vx_byte(&mut self, x: u8, kk: u8) { // 4xkk - SNE Vx, byte
    if self.reg_v[x as usize] != kk {
      self.pc += 2;
    }
  }

  // Sets VX to NN.
  fn ld_vx_byte(&mut self, x: u8, kk: u8) { // 6xkk - LD Vx, byte
    self.reg_v[x as usize] = kk;
  }

  // Adds NN to VX. (Carry flag is not changed)
  fn add_vx_byte(&mut self, x: u8, kk: u8) { // 7xkk - ADD Vx, byte
    self.reg_v[x as usize] = self.reg_v[x as usize].wrapping_add(kk);
  }

  // Skips the next instruction if VX equals VY.
  fn se_vx_vy(&mut self, x: u8, y: u8) { // 5xy0 - SE Vx, Vy
    if self.reg_v[x as usize] == self.reg_v[y as usize] {
      self.pc += 2;
    }
  }

  // Skips the next instruction if VX doesn't equal VY.
  fn sne_vx_vy(&mut self, x: u8, y: u8) { // 9xy0 - SNE Vx, Vy
    if self.reg_v[x as usize] != self.reg_v[y as usize] {
      self.pc += 2;
    }
  }

  // Sets VX to the value of VY.
  fn ld_vx_vy(&mut self, x: u8, y: u8) { // 8xy0 - LD Vx, Vy
    self.reg_v[x as usize] = self.reg_v[y as usize];
  }

  // Sets VX to VX or VY.
  fn or_vx_vy(&mut self, x: u8, y: u8) { // 8xy1 - OR Vx, Vy
    self.reg_v[x as usize] |= self.reg_v[y as usize];
  }

  // Sets VX to VX and VY.
  fn and_vx_vy(&mut self, x: u8, y: u8) { // 8xy2 - AND Vx, Vy
    self.reg_v[x as usize] &= self.reg_v[y as usize];
  }

  // Sets VX to VX xor VY.
  fn xor_vx_vy(&mut self, x: u8, y: u8) { // 8xy3 - XOR Vx, Vy
    self.reg_v[x as usize] ^= self.reg_v[y as usize];
  }

  // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
  fn add_vx_vy(&mut self, x: u8, y: u8) { // 8xy4 - ADD Vx, Vy
    match self.reg_v[x as usize].overflowing_add(self.reg_v[y as usize]) {
      (num, true) => {
        self.reg_v[x as usize] = num;
        self.reg_v[VF] = 0x1;
      }
      (num, false) => {
        self.reg_v[x as usize] = num;
        self.reg_v[VF] = 0x0;
      }
    }
  }

  // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
  fn sub_vx_vy(&mut self, x: u8, y: u8) { // 8xy5 - SUB Vx, Vy
    match self.reg_v[x as usize].overflowing_sub(self.reg_v[y as usize]) {
      (num, true) => {
        self.reg_v[x as usize] = num;
        self.reg_v[VF] = 0x0;
      }
      (num, false) => {
        self.reg_v[x as usize] = num;
        self.reg_v[VF] = 0x1;
      }
    }
  }

  // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
  fn subn_vx_vy(&mut self, x: u8, y: u8) { // 8xy7 - SUBN Vx, Vy
    match self.reg_v[y as usize].overflowing_sub(self.reg_v[x as usize]) {
      (num, true) => {
        self.reg_v[x as usize] = num;
        self.reg_v[VF] = 0x0;
      }
      (num, false) => {
        self.reg_v[x as usize] = num;
        self.reg_v[VF] = 0x1;
      }
    }
  }

  // Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
  fn shr_vx_vy(&mut self, x: u8, y: u8) { // 8xy6 - SHR Vx {, Vy}
    let source: u8 = if self.is_mode(Mode::SCHIP) {
      self.reg_v[x as usize]
    } else {
      self.reg_v[y as usize]
    };

    self.reg_v[VF] = source & 0x1;
    self.reg_v[x as usize] = source >> 0x1;
  }

  // Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
  fn shl_vx_vy(&mut self, x: u8, y: u8) { // 8xyE - SHL Vx {, Vy}
    let source: u8 = if self.is_mode(Mode::SCHIP) {
      self.reg_v[x as usize]
    } else {
      self.reg_v[y as usize]
    };

    self.reg_v[VF] = source >> 7;
    self.reg_v[x as usize] = source << 0x1;
  }

  // Sets I to the address NNN.
  fn ld_i_addr(&mut self, nnn: u16) { // Annn - LD I, addr
    self.reg_i = nnn;
  }

  // Jumps to the address NNN + V0.
  fn jp_v0_addr(&mut self, nnn: u16) { // Bnnn - JP V0, addr
    self.pc = self.reg_v[0] as u16 + nnn;
  }

  // Sets VX to the result of a bitwise and operation on a random number and NN.
  fn rnd_vx_byte(&mut self, x: u8, kk: u8) { // Cxkk - RND Vx, byte
    self.reg_v[x as usize] = rand() & kk;
  }

  fn drw_vx_vy_nibble(&mut self, x: u8, y: u8, mut n: u8) { // Dxyn - DRW Vx, Vy, nibble
    let x: usize = self.reg_v[x as usize] as usize;
    let y: usize = self.reg_v[y as usize] as usize;

    self.reg_v[VF] = 0x0;

    for yline in 0..n as usize {
      let pixel: u8 = self.memory[self.reg_i as usize + yline];

      for xline in 0..8 {
        if (pixel & (0x80 >> xline)) != 0 {
          let index: usize = (y + yline) * Self::W + (x + xline);

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

  // Skips the next instruction if the key stored in VX is pressed.
  fn skp_vx(&mut self, x: u8) { // Ex9E - SKP Vx
    if self.key_pressed(self.reg_v[x as usize]) {
      self.pc += 2;
    }
  }

  // Skips the next instruction if the key stored in VX isn't pressed.
  fn sknp_vx(&mut self, x: u8) { // ExA1 - SKNP Vx
    if !self.key_pressed(self.reg_v[x as usize]) {
      self.pc += 2;
    }
  }

  // Sets VX to the value of the delay timer.
  fn ld_vx_dt(&mut self, x: u8) { // Fx07 - LD Vx, DT
    self.reg_v[x as usize] = self.delay;
  }

  // A key press is awaited, and then stored in VX.
  fn ld_vx_k(&mut self, x: u8) { // Fx0A - LD Vx, K
    self.wait = &mut self.reg_v[x as usize];
  }

  // Sets the delay timer to VX.
  fn ld_dt_vx(&mut self, x: u8) { // Fx15 - LD DT, Vx
    self.delay = self.reg_v[x as usize];
  }

  // Sets the sound timer to VX.
  //
  // TODO
  // It should be noted that in the COSMAC VIP manual, it was made clear
  // that the minimum value that the timer will respond to is 02. Thus,
  // setting the timer to a value of 01 would have no audible effect.
  fn ld_st_vx(&mut self, x: u8) { // Fx18 - LD ST, Vx
    self.sound = self.reg_v[x as usize];
  }

  // Adds VX to I.
  //
  // VF is set to 1 when there is a range overflow (I + VX > 0xFFF),
  // and to 0 when there isn't. This is an undocumented feature of the
  // CHIP-8 and used by the Spacefight 2091! game.
  fn add_i_vx(&mut self, x: u8) { // Fx1E - ADD I, Vx
    if self.is_mode(Mode::SCHIP) {
      let num: u16 = self.reg_i.wrapping_add(self.reg_v[x as usize] as u16);

      self.reg_v[VF] = if num > 0xFFF {
        0x1
      } else {
        0x0
      };

      self.reg_i = num;
    } else {
      self.reg_i = self.reg_i.wrapping_add(self.reg_v[x as usize] as u16);
    }
  }

  // Sets I to the location of the sprite for the character in VX.
  fn ld_f_vx(&mut self, x: u8) { // Fx29 - LD F, Vx
    self.reg_i = self.reg_v[x as usize] as u16 * 5;
  }

  // Stores the binary-coded decimal representation of VX,
  // with the most significant of three digits at the address in I,
  // the middle digit at I + 1, and the least significant digit at I + 2.
  fn ld_b_vx(&mut self, x: u8) { // Fx33 - LD B, Vx
    let rx: u8 = self.reg_v[x as usize];

    self.memory[self.reg_i as usize] = rx / 100;
    self.memory[self.reg_i as usize + 1] = (rx / 10) % 10;
    self.memory[self.reg_i as usize + 2] = (rx % 100) % 10;
  }

  // Stores V0 to VX (including VX) in memory starting at address I.
  // The offset from I is increased by 1 for each value written, but I itself is left unmodified.
  fn ld_i_vx(&mut self, x: u8) { // Fx55 - LD [I], Vx
    let output: RangeInclusive<usize> = self.reg_i as usize..=self.reg_i as usize + x as usize;
    let source: RangeToInclusive<usize> = ..=x as usize;

    self.memory[output].copy_from_slice(&self.reg_v[source]);

    if self.is_mode(Mode::CHIP) {
      self.reg_i += x as u16 + 1;
    }
  }

  // Fills V0 to VX (including VX) with values from memory starting at address I.
  // The offset from I is increased by 1 for each value written, but I itself is left unmodified.
  fn ld_vx_i(&mut self, x: u8) { // Fx65 - LD Vx, [I]
    let output: RangeToInclusive<usize> = ..=x as usize;
    let source: RangeInclusive<usize> = self.reg_i as usize..=self.reg_i as usize + x as usize;

    self.reg_v[output].copy_from_slice(&self.memory[source]);

    if self.is_mode(Mode::CHIP) {
      self.reg_i += x as u16 + 1;
    }
  }

  // ===========================================================================
  // Super Chip-48
  // ===========================================================================

  // Scroll up N pixels (N/2 pixels in low res mode)
  fn scu_nibble(&mut self, mut n: u8) { // 00Bn - SCU nibble
    if self.pitch == Pitch::P8 {
      n >>= 0x1;
    }

    let from: usize = n as usize * Self::W;
    let clear: RangeFrom<usize> = self.display.len() - from..;

    self.display.copy_within(from.., 0);

    for pixel in self.display[clear].iter_mut() {
      *pixel = 0;
    }

    self.render = true;
  }

  // Scroll down N pixels (N/2 pixels in low res mode)
  fn scd_nibble(&mut self, mut n: u8) { // 00Cn - SCD nibble
    if self.pitch == Pitch::P8 {
      n >>= 0x1;
    }

    let from: usize = n as usize * Self::W;
    let source: RangeTo<usize> = ..self.display.len() - from;

    self.display.copy_within(source, from);

    for pixel in self.display[..from].iter_mut() {
      *pixel = 0;
    }

    self.render = true;
  }

  // Scroll right 4 pixels (2 pixels in low res mode)
  fn scr(&mut self) { // 00FB - SCR
    let pitch: usize = self.pitch as usize;
    let shift: usize = pitch >> 2;

    for y in 0..Self::H {
      for x in (0..Self::W).rev() {
        let index: usize = y * Self::W + x;

        if x > shift {
          self.display[index] = self.display[index - shift];
        } else {
          self.display[index] = 0;
        }
      }
    }

    self.render = true;
  }

  // Scroll left 4 pixels (2 pixels in low res mode)
  fn scl(&mut self) { // 00FC - SCL
    let pitch: usize = self.pitch as usize;
    let shift: usize = pitch >> 2;

    for y in 0..Self::H {
      for x in 0..Self::W {
        let index: usize = y * Self::W + x;

        if x < Self::W - shift {
          self.display[index] = self.display[index + shift];
        } else {
          self.display[index] = 0;
        }
      }
    }

    self.render = true;
  }

  // Exit the interpreter; this causes the VM to infinite loop
  fn exit(&mut self) { // 00FD - EXIT
    self.pc -= 2;
  }

  // Enter low resolution (64x32) mode; this is the default mode
  fn low(&mut self) { // 00FE - LOW
    self.pitch = Pitch::P8;
  }

  // Enter high resolution (128x64) mode
  fn high(&mut self) { // 00FF - HIGH
    self.pitch = Pitch::P16;
  }

  // Draw a 16x16 sprite at I to VX, VY (8x16 in low res mode)
  fn drw_vx_vy_0(&mut self, x: u8, y: u8) { // Dxy0 - DRW Vx, Vy, 0
    self.drw_vx_vy_nibble(x, y, 0);
  }

  // I = address of 8x10 font character in VX (0..F)
  fn ld_hf_vx(&mut self, x: u8) { // Fx30 - LD HF, Vx
    debug_assert!(x < 0xF /* 0xA */, "Fx30 Overflow = {:#03X}", x);
    self.reg_i = FONT.len() as u16 + (self.reg_v[x as usize] as u16 * 10);
  }

  // Store V0..VX (inclusive) into HP-RPL user flags R0..RX
  fn ld_r_vx(&mut self, x: u8) { // Fx75 - LD R, Vx
    debug_assert!(x < UFLAGS as u8, "Fx75 Overflow = {:#03X}", x);
    self.reg_u[..=x as usize].copy_from_slice(&self.reg_v[..=x as usize]);
  }

  // Load V0..VX (inclusive) from HP-RPL user flags R0..RX
  fn ld_vx_r(&mut self, x: u8) { // Fx85 - LD Vx, R
    debug_assert!(x < UFLAGS as u8, "Fx85 Overflow = {:#03X}", x);
    self.reg_v[..=x as usize].copy_from_slice(&self.reg_u[..=x as usize]);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cls() {
    unimplemented!()
  }

  #[test]
  fn test_ret() {
    unimplemented!()
  }

  #[test]
  fn test_scr() {
    unimplemented!()
  }

  #[test]
  fn test_scl() {
    unimplemented!()
  }

  #[test]
  fn test_exit() {
    unimplemented!()
  }

  #[test]
  fn test_low() {
    unimplemented!()
  }

  #[test]
  fn test_high() {
    unimplemented!()
  }

  #[test]
  fn test_jp_addr() {
    unimplemented!()
  }

  #[test]
  fn test_call_addr() {
    unimplemented!()
  }

  #[test]
  fn test_se_vx_byte() {
    unimplemented!()
  }

  #[test]
  fn test_sne_vx_byte() {
    unimplemented!()
  }

  #[test]
  fn test_ld_vx_byte() {
    unimplemented!()
  }

  #[test]
  fn test_add_vx_byte() {
    unimplemented!()
  }

  #[test]
  fn test_ld_i_addr() {
    unimplemented!()
  }

  #[test]
  fn test_jp_v0_addr() {
    unimplemented!()
  }

  #[test]
  fn test_rnd_vx_byte() {
    unimplemented!()
  }

  #[test]
  fn test_drw_vx_vy_nibble() {
    unimplemented!()
  }

  #[test]
  fn test_se_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_ld_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_or_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_and_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_xor_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_add_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_sub_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_shr_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_subn_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_shl_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_sne_vx_vy() {
    unimplemented!()
  }

  #[test]
  fn test_drw_vx_vy_0() {
    unimplemented!()
  }

  #[test]
  fn test_skp_vx() {
    unimplemented!()
  }

  #[test]
  fn test_sknp_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_vx_dt() {
    unimplemented!()
  }

  #[test]
  fn test_ld_vx_k() {
    unimplemented!()
  }

  #[test]
  fn test_ld_dt_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_st_vx() {
    unimplemented!()
  }

  #[test]
  fn test_add_i_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_f_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_hf_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_b_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_i_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_vx_i() {
    unimplemented!()
  }

  #[test]
  fn test_ld_r_vx() {
    unimplemented!()
  }

  #[test]
  fn test_ld_vx_r() {
    unimplemented!()
  }

  #[test]
  fn test_scu_nibble() {
    unimplemented!()
  }

  #[test]
  fn test_scd_nibble() {
    unimplemented!()
  }
}
