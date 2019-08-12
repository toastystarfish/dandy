
#[derive(Debug, PartialEq)]
pub enum Opcode {
  LOAD,
  ADD,
  SUB,
  MUL,
  DIV,
  JMP,
  JMPF,
  JMPB,
  EQ,
  NEQ,
  GT,
  LT,
  GTQ,
  LTQ,
  JEQ,
  JNEQ,
  INC,
  DEC,
  HLT = 255,
  IGL
}

impl From<u8> for Opcode {
  fn from(v: u8) -> Self {
    match v {
      0 => Opcode::LOAD,
      1 => Opcode::ADD,
      2 => Opcode::SUB,
      3 => Opcode::MUL,
      4 => Opcode::DIV,
      5 => Opcode::JMP,
      6 => Opcode::JMPF,
      7 => Opcode::JMPB,
      8 => Opcode::EQ,
      9 => Opcode::NEQ,
      10 => Opcode::GT,
      11 => Opcode::LT,
      12 => Opcode::GTQ,
      13 => Opcode::LTQ,
      14 => Opcode::JEQ,
      15 => Opcode::JNEQ,
      16 => Opcode::INC,
      17 => Opcode::DEC,
      255 => Opcode::HLT,
      _ => Opcode::IGL
    }
  }
}