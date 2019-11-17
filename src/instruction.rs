pub struct Instruction<'a> {
  pub code: u16,
  pub mask: u16,
  pub name: &'a str,
  pub desc: &'a str,
}

impl<'a> Instruction<'a> {
  pub fn find(opcode: u16) -> Option<&'static Self> {
    INSTRUCTIONS
      .iter()
      .find(|instruction| instruction.mask & opcode == instruction.code)
  }
}

const INSTRUCTIONS: &[Instruction] = &[
  Instruction {
    code: 0x00E0,
    mask: 0xFFFF,
    name: "CLS",
    desc: "",
  },
  Instruction {
    code: 0x00EE,
    mask: 0xFFFF,
    name: "RET",
    desc: "",
  },
  Instruction {
    code: 0x00FB,
    mask: 0xFFFF,
    name: "SCR",
    desc: "",
  },
  Instruction {
    code: 0x00FC,
    mask: 0xFFFF,
    name: "SCL",
    desc: "",
  },
  Instruction {
    code: 0x00FD,
    mask: 0xFFFF,
    name: "EXIT",
    desc: "",
  },
  Instruction {
    code: 0x00FE,
    mask: 0xFFFF,
    name: "LOW",
    desc: "",
  },
  Instruction {
    code: 0x00FF,
    mask: 0xFFFF,
    name: "HIGH",
    desc: "",
  },
  Instruction {
    code: 0x00B0,
    mask: 0xFFF0,
    name: "SCU",
    desc: "nibble",
  },
  Instruction {
    code: 0x00C0,
    mask: 0xFFF0,
    name: "SCD",
    desc: "nibble",
  },
  Instruction {
    code: 0x0000,
    mask: 0xF000,
    name: "SYS",
    desc: "addr",
  },
  Instruction {
    code: 0x1000,
    mask: 0xF000,
    name: "JP",
    desc: "addr",
  },
  Instruction {
    code: 0x2000,
    mask: 0xF000,
    name: "CALL",
    desc: "addr",
  },
  Instruction {
    code: 0x3000,
    mask: 0xF000,
    name: "SE",
    desc: "Vx, byte",
  },
  Instruction {
    code: 0x4000,
    mask: 0xF000,
    name: "SNE",
    desc: "Vx, byte",
  },
  Instruction {
    code: 0x5000,
    mask: 0xF00F,
    name: "SE",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x6000,
    mask: 0xF000,
    name: "LD",
    desc: "Vx, byte",
  },
  Instruction {
    code: 0x7000,
    mask: 0xF000,
    name: "ADD",
    desc: "Vx, byte",
  },
  Instruction {
    code: 0x8000,
    mask: 0xF00F,
    name: "LD",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x8001,
    mask: 0xF00F,
    name: "OR",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x8002,
    mask: 0xF00F,
    name: "AND",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x8003,
    mask: 0xF00F,
    name: "XOR",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x8004,
    mask: 0xF00F,
    name: "ADD",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x8005,
    mask: 0xF00F,
    name: "SUB",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x8006,
    mask: 0xF00F,
    name: "SHR",
    desc: "Vx {, Vy}",
  },
  Instruction {
    code: 0x8007,
    mask: 0xF00F,
    name: "SUBN",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0x800E,
    mask: 0xF00F,
    name: "SHL",
    desc: "Vx {, Vy}",
  },
  Instruction {
    code: 0x9000,
    mask: 0xF00F,
    name: "SNE",
    desc: "Vx, Vy",
  },
  Instruction {
    code: 0xA000,
    mask: 0xF000,
    name: "LD",
    desc: "I, addr",
  },
  Instruction {
    code: 0xB000,
    mask: 0xF000,
    name: "JP",
    desc: "V0, addr",
  },
  Instruction {
    code: 0xC000,
    mask: 0xF000,
    name: "RND",
    desc: "Vx, byte",
  },
  Instruction {
    code: 0xD000,
    mask: 0xF00F,
    name: "DRW",
    desc: "Vx, Vy, 0",
  },
  Instruction {
    code: 0xD000,
    mask: 0xF000,
    name: "DRW",
    desc: "Vx, Vy, nibble",
  },
  Instruction {
    code: 0xE09E,
    mask: 0xF0FF,
    name: "SKP",
    desc: "Vx",
  },
  Instruction {
    code: 0xE0A1,
    mask: 0xF0FF,
    name: "SKNP",
    desc: "Vx",
  },
  Instruction {
    code: 0xF007,
    mask: 0xF0FF,
    name: "LD",
    desc: "Vx, DT",
  },
  Instruction {
    code: 0xF00A,
    mask: 0xF0FF,
    name: "LD",
    desc: "Vx, K",
  },
  Instruction {
    code: 0xF015,
    mask: 0xF0FF,
    name: "LD",
    desc: "DT, Vx",
  },
  Instruction {
    code: 0xF018,
    mask: 0xF0FF,
    name: "LD",
    desc: "ST, Vx",
  },
  Instruction {
    code: 0xF01E,
    mask: 0xF0FF,
    name: "ADD",
    desc: "I, Vx",
  },
  Instruction {
    code: 0xF029,
    mask: 0xF0FF,
    name: "LD",
    desc: "F, Vx",
  },
  Instruction {
    code: 0xF030,
    mask: 0xF0FF,
    name: "LD",
    desc: "HF, Vx",
  },
  Instruction {
    code: 0xF033,
    mask: 0xF0FF,
    name: "LD",
    desc: "B, Vx",
  },
  Instruction {
    code: 0xF055,
    mask: 0xF0FF,
    name: "LD",
    desc: "[I], Vx",
  },
  Instruction {
    code: 0xF065,
    mask: 0xF0FF,
    name: "LD",
    desc: "Vx, [I]",
  },
  Instruction {
    code: 0xF075,
    mask: 0xF0FF,
    name: "LD",
    desc: "R, Vx",
  },
  Instruction {
    code: 0xF085,
    mask: 0xF0FF,
    name: "LD",
    desc: "Vx, R",
  },
];
