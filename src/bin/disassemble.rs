use std::env::args;
use std::fs::read;
use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::io::Write;

#[macro_use]
extern crate chip;

use chip::Instruction;

fn main() -> Result<(), Error> {
  let path: String = args().skip(1).next().unwrap_or_default();
  let buffer: Vec<u8> = read(&path)?;

  let file: File = File::create("out.asm")?;
  let mut writer: BufWriter<File> = BufWriter::new(file);

  for opcodes in buffer.chunks(2) {
    let opcode: u16 = (opcodes[0] as u16) << 8 | opcodes[1] as u16;

    if let Some(instruction) = Instruction::find(opcode) {
      let description: String = instruction
        .desc
        .replace("Vx", &format!("V{:X}", x!(opcode)))
        .replace("Vy", &format!("V{:X}", y!(opcode)))
        .replace("nibble", &format!("{:X}", n!(opcode)))
        .replace("byte", &format!("{:02X}", kk!(opcode)))
        .replace("addr", &format!("{:04X}", nnn!(opcode)));

      writeln!(
        writer,
        "{name:<padding$}{description}",
        name = instruction.name,
        description = description,
        padding = if description.is_empty() { 0 } else { 8 },
      )?;
    }
  }

  Ok(())
}
