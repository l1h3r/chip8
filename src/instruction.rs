use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use core::ops::Range;

use crate::chip8::Chip8;
use crate::chip8::Mode;
use crate::chip8::FONT_BASE;
use crate::chip8::VF;

#[inline(always)]
fn address(opcode: u16) -> u16 {
  opcode & 0x0FFF
}

#[inline(always)]
fn byte(opcode: u16, index: u8) -> u8 {
  ((opcode >> 8 * index) & 0xFF) as u8
}

#[inline(always)]
fn nibble(opcode: u16, index: u8) -> u8 {
  ((opcode >> 4 * index) & 0x0F) as u8
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Instruction<'a> {
  pub code: u16,
  pub mask: u16,
  pub name: &'a str,
  pub desc: &'a str,
  pub f: &'a Fn(u16, &mut Chip8),
}

impl<'a> Instruction<'a> {
  pub fn find(opcode: u16) -> Option<&'static Self> {
    INSTRUCTIONS
      .into_iter()
      .find(|instruction| opcode & instruction.mask == instruction.code)
  }
}

const INSTRUCTIONS: &'static [Instruction] = &[
  Instruction {
    code: 0x00E0,
    mask: 0xFFFF,
    name: "00E0 - CLS",
    desc: "Clear the display.",
    f: &|_opcode, chip8| {
      chip8.clear();
      chip8.next();
    },
  },
  Instruction {
    code: 0x00EE,
    mask: 0xFFFF,
    name: "00EE - RET",
    desc: "Return from a subroutine.",
    f: &|_opcode, chip8| {
      chip8.sp -= 1;
      chip8.pc = chip8.stack[chip8.sp as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0x1000,
    mask: 0xF000,
    name: "1nnn - JP addr",
    desc: "Jump to location nnn.",
    f: &|opcode, chip8| {
      chip8.pc = address(opcode);
    },
  },
  Instruction {
    code: 0x2000,
    mask: 0xF000,
    name: "2nnn - CALL addr",
    desc: "Call subroutine at nnn.",
    f: &|opcode, chip8| {
      chip8.stack[chip8.sp as usize] = chip8.pc;
      chip8.sp += 1;
      chip8.pc = address(opcode);
    },
  },
  Instruction {
    code: 0x3000,
    mask: 0xF000,
    name: "3xkk - SE Vx, byte",
    desc: "Skip next instruction if Vx = kk.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let nn: u8 = byte(opcode, 0);

      if chip8.reg_v[vx as usize] == nn {
        chip8.next();
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0x4000,
    mask: 0xF000,
    name: "4xkk - SNE Vx, byte",
    desc: "Skip next instruction if Vx != kk.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let nn: u8 = byte(opcode, 0);

      if chip8.reg_v[vx as usize] != nn {
        chip8.next();
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0x5000,
    mask: 0xF00F,
    name: "5xy0 - SE Vx, Vy",
    desc: "Skip next instruction if Vx = Vy.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      if chip8.reg_v[vx as usize] == chip8.reg_v[vy as usize] {
        chip8.next();
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0x6000,
    mask: 0xF000,
    name: "6xkk - LD Vx, byte",
    desc: "Set Vx = kk.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let nn: u8 = byte(opcode, 0);

      chip8.reg_v[vx as usize] = nn;
      chip8.next();
    },
  },
  Instruction {
    code: 0x7000,
    mask: 0xF000,
    name: "7xkk - ADD Vx, byte",
    desc: "Set Vx = Vx + kk.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let nn: u8 = byte(opcode, 0);

      chip8.reg_v[vx as usize] = chip8.reg_v[vx as usize].wrapping_add(nn);
      chip8.next();
    },
  },
  Instruction {
    code: 0x8000,
    mask: 0xF00F,
    name: "8xy0 - LD Vx, Vy",
    desc: "Set Vx = Vy.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      chip8.reg_v[vx as usize] = chip8.reg_v[vy as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0x8001,
    mask: 0xF00F,
    name: "8xy1 - OR Vx, Vy",
    desc: "Set Vx = Vx OR Vy.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      chip8.reg_v[vx as usize] |= chip8.reg_v[vy as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0x8002,
    mask: 0xF00F,
    name: "8xy2 - AND Vx, Vy",
    desc: "Set Vx = Vx AND Vy.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      chip8.reg_v[vx as usize] &= chip8.reg_v[vy as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0x8003,
    mask: 0xF00F,
    name: "8xy3 - XOR Vx, Vy",
    desc: "Set Vx = Vx XOR Vy.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      chip8.reg_v[vx as usize] ^= chip8.reg_v[vy as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0x8004,
    mask: 0xF00F,
    name: "8xy4 - ADD Vx, Vy",
    desc: "Set Vx = Vx + Vy, set VF = carry.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      match chip8.reg_v[vx as usize].overflowing_add(chip8.reg_v[vy as usize]) {
        (num, true) => {
          chip8.reg_v[vx as usize] = num;
          chip8.reg_v[VF] = 0x1;
        }
        (num, false) => {
          chip8.reg_v[vx as usize] = num;
          chip8.reg_v[VF] = 0x0;
        }
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0x8005,
    mask: 0xF00F,
    name: "8xy5 - SUB Vx, Vy",
    desc: "Set Vx = Vx - Vy, set VF = NOT borrow.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      match chip8.reg_v[vx as usize].overflowing_sub(chip8.reg_v[vy as usize]) {
        (num, true) => {
          chip8.reg_v[vx as usize] = num;
          chip8.reg_v[VF] = 0x0;
        }
        (num, false) => {
          chip8.reg_v[vx as usize] = num;
          chip8.reg_v[VF] = 0x1;
        }
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0x8006,
    mask: 0xF00F,
    name: "8xy6 - SHR Vx {, Vy}",
    desc: "Set Vx = Vx SHR 1.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      let source: u8 = if chip8.mode(Mode::SCHIP) {
        chip8.reg_v[vx as usize]
      } else {
        chip8.reg_v[vy as usize]
      };

      chip8.reg_v[VF] = source & 0x1;
      chip8.reg_v[vx as usize] = source >> 0x1;
      chip8.next();
    },
  },
  Instruction {
    code: 0x8007,
    mask: 0xF00F,
    name: "8xy7 - SUBN Vx, Vy",
    desc: "Set Vx = Vy - Vx, set VF = NOT borrow.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      match chip8.reg_v[vy as usize].overflowing_sub(chip8.reg_v[vx as usize]) {
        (num, true) => {
          chip8.reg_v[vx as usize] = num;
          chip8.reg_v[VF] = 0x0;
        }
        (num, false) => {
          chip8.reg_v[vx as usize] = num;
          chip8.reg_v[VF] = 0x1;
        }
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0x800E,
    mask: 0xF00F,
    name: "8xyE - SHL Vx {, Vy}",
    desc: "Set Vx = Vx SHL 1.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      let source: u8 = if chip8.mode(Mode::SCHIP) {
        chip8.reg_v[vx as usize]
      } else {
        chip8.reg_v[vy as usize]
      };

      // chip8.reg_v[VF] = source & 0x80;
      chip8.reg_v[VF] = source >> 7;
      chip8.reg_v[vx as usize] = source << 0x1;
      chip8.next();
    },
  },
  Instruction {
    code: 0x9000,
    mask: 0xF00F,
    name: "9xy0 - SNE Vx, Vy",
    desc: "Skip next instruction if Vx != Vy.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let vy: u8 = nibble(opcode, 1);

      if chip8.reg_v[vx as usize] != chip8.reg_v[vy as usize] {
        chip8.next();
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0xA000,
    mask: 0xF000,
    name: "Annn - LD I, addr",
    desc: "Set I = nnn.",
    f: &|opcode, chip8| {
      chip8.reg_i = address(opcode);
      chip8.next();
    },
  },
  Instruction {
    code: 0xB000,
    mask: 0xF000,
    name: "Bnnn - JP V0, addr",
    desc: "Jump to location nnn + V0.",
    f: &|opcode, chip8| {
      chip8.pc = address(opcode) + chip8.reg_v[0] as u16;
    },
  },
  Instruction {
    code: 0xC000,
    mask: 0xF000,
    name: "Cxkk - RND Vx, byte",
    desc: "Set Vx = random byte AND kk.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let nn: u8 = byte(opcode, 0);

      chip8.reg_v[vx as usize] = chip8.rand() & nn;
      chip8.next();
    },
  },
  Instruction {
    code: 0xD000,
    mask: 0xF000,
    name: "Dxyn - DRW Vx, Vy, nibble",
    desc: "Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.",
    f: &|opcode, chip8| {
      let x: usize = chip8.reg_v[nibble(opcode, 2) as usize] as usize;
      let y: usize = chip8.reg_v[nibble(opcode, 1) as usize] as usize;
      let n: usize = nibble(opcode, 0) as usize;

      chip8.draw(x, y, n);
      chip8.next();
    },
  },
  Instruction {
    code: 0xE09E,
    mask: 0xF0FF,
    name: "Ex9E - SKP Vx",
    desc: "Skip next instruction if key with the value of Vx is pressed.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let rx: u8 = chip8.reg_v[vx as usize];

      if chip8.key_pressed(rx) {
        chip8.next();
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0xE0A1,
    mask: 0xF0FF,
    name: "ExA1 - SKNP Vx",
    desc: "Skip next instruction if key with the value of Vx is not pressed.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let rx: u8 = chip8.reg_v[vx as usize];

      if !chip8.key_pressed(rx) {
        chip8.next();
      }

      chip8.next();
    },
  },
  Instruction {
    code: 0xF007,
    mask: 0xF0FF,
    name: "Fx07 - LD Vx, DT",
    desc: "Set Vx = delay timer value.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      chip8.reg_v[vx as usize] = chip8.delay;
      chip8.next();
    },
  },
  Instruction {
    code: 0xF00A,
    mask: 0xF0FF,
    name: "Fx0A - LD Vx, K",
    desc: "Wait for a key press, store the value of the key in Vx.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      chip8.wait = &mut chip8.reg_v[vx as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0xF015,
    mask: 0xF0FF,
    name: "Fx15 - LD DT, Vx",
    desc: "Set delay timer = Vx.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      chip8.delay = chip8.reg_v[vx as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0xF018,
    mask: 0xF0FF,
    name: "Fx18 - LD ST, Vx",
    desc: "Set sound timer = Vx.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      // TODO:
      // It should be noted that in the COSMAC VIP manual, it was made clear
      // that the minimum value that the timer will respond to is 02. Thus,
      // setting the timer to a value of 01 would have no audible effect.

      chip8.sound = chip8.reg_v[vx as usize];
      chip8.next();
    },
  },
  Instruction {
    code: 0xF01E,
    mask: 0xF0FF,
    name: "Fx1E - ADD I, Vx",
    desc: "Set I = I + Vx.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      chip8.reg_i = chip8.reg_i.wrapping_add(chip8.reg_v[vx as usize] as u16);
      chip8.next();
    },
  },
  Instruction {
    code: 0xF029,
    mask: 0xF0FF,
    name: "Fx29 - LD F, Vx",
    desc: "Set I = location of sprite for digit Vx.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      chip8.reg_i = FONT_BASE as u16 + chip8.reg_v[vx as usize] as u16 * 5;
      chip8.next();
    },
  },
  Instruction {
    code: 0xF033,
    mask: 0xF0FF,
    name: "Fx33 - LD B, Vx",
    desc: "Store BCD representation of Vx in memory locations I, I+1, and I+2.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);
      let rx: u8 = chip8.reg_v[vx as usize];

      chip8.memory[chip8.reg_i as usize] = rx / 100;
      chip8.memory[chip8.reg_i as usize + 1] = (rx / 10) % 10;
      chip8.memory[chip8.reg_i as usize + 2] = (rx % 100) % 10;

      chip8.next();
    },
  },
  Instruction {
    code: 0xF055,
    mask: 0xF0FF,
    name: "Fx55 - LD [I], Vx",
    desc: "Store registers V0 through Vx in memory starting at location I.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      let output: Range<usize> = (chip8.reg_i as usize)..(chip8.reg_i + vx as u16 + 1) as usize;
      let source: Range<usize> = 0..vx as usize + 1;

      chip8.memory[output].copy_from_slice(&chip8.reg_v[source]);
      chip8.next();
    },
  },
  Instruction {
    code: 0xF065,
    mask: 0xF0FF,
    name: "Fx65 - LD Vx, [I]",
    desc: "Read registers V0 through Vx from memory starting at location I.",
    f: &|opcode, chip8| {
      let vx: u8 = nibble(opcode, 2);

      let output: Range<usize> = 0..(vx as usize + 1);
      let source: Range<usize> = (chip8.reg_i as usize)..(chip8.reg_i + vx as u16 + 1) as usize;

      chip8.reg_v[output].clone_from_slice(&chip8.memory[source]);
      chip8.next();
    },
  },
];

impl<'a> Display for Instruction<'a> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    writeln!(f, "Instruction > {}", self.name)?;
    writeln!(f, "[+] {}", self.desc)?;
    writeln!(f, "[+] C: {:#06X}", self.code)?;
    writeln!(f, "[+] M: {:#06X}", self.mask)?;

    Ok(())
  }
}

impl<'a> Debug for Instruction<'a> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.debug_struct("Instruction")
      .field("code", &format_args!("{:#X}", self.code))
      .field("mask", &format_args!("{:#X}", self.mask))
      .field("name", &self.name)
      .field("desc", &self.desc)
      .finish()
  }
}
