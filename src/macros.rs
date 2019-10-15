#[macro_export]
macro_rules! blankify {
  ($array:expr) => {
    for byte in $array {
      *byte = 0;
    }
  };
}

#[macro_export]
macro_rules! x {
  ($opcode:expr) => {
    (($opcode >> 8) & 0x0F) as u8
  };
}

#[macro_export]
macro_rules! y {
  ($opcode:expr) => {
    (($opcode >> 4) & 0x0F) as u8
  };
}

#[macro_export]
macro_rules! n {
  ($opcode:expr) => {
    ($opcode & 0x0F) as u8
  };
}

#[macro_export]
macro_rules! kk {
  ($opcode:expr) => {
    ($opcode & 0xFF) as u8
  };
}

#[macro_export]
macro_rules! nnn {
  ($opcode:expr) => {
    $opcode & 0x0FFF
  };
}
