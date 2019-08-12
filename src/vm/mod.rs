
mod opcode;
pub use opcode::Opcode;

mod instruction;
pub use instruction::Instruction;

pub struct VM {
  pub registers: [i32; 32],
  pub program: Vec<u8>,
  pc: usize,
  remainder: usize,
  equal_flag: bool
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      program: vec![],
      pc: 0,
      remainder: 0,
      equal_flag: false
    }
  }

  pub fn run (&mut self) {
    let mut is_done = false;
    while !is_done {
      is_done = self.execute_instruction();
    }
  }

  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  pub fn add_byte(&mut self, byte: u8) {
    self.program.push(byte);
  }

  fn execute_instruction(&mut self) -> bool {
    if self.pc >= self.program.len() {
      return true;
    }

    match self.decode_opcode() {
      Opcode::LOAD => {
        let register = self.next_8_bits() as usize; // usize so can index into vec
        let number   = self.next_16_bits() as i32;
        self.registers[register] = number;
      },
      Opcode::HLT => {
        println!("HLT encountered");
        return true;
      },
      Opcode::ADD => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 + r2;
      },
      Opcode::SUB => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 - r2;
      },
      Opcode::MUL => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 * r2;
      },
      Opcode::DIV => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];

        self.remainder = (r1 % r2) as usize;
        self.registers[self.next_8_bits() as usize] = r1 / r2;
      },
      Opcode::JMP => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc = target as usize;
      },
      Opcode::JMPF => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc += target as usize;
      },
      Opcode::JMPB => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc -= target as usize;
      },
      Opcode::EQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 == r2;
        self.next_8_bits();
      },
      Opcode::NEQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 != r2;
        self.next_8_bits();
      },
      Opcode::GT => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 > r2;
        self.next_8_bits();
      },
      Opcode::LT => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 < r2;
        self.next_8_bits();
      },
      Opcode::GTQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 >= r2;
        self.next_8_bits();
      },
      Opcode::LTQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 <= r2;
        self.next_8_bits();
      },
      Opcode::JEQ => {
        let target = self.registers[self.next_8_bits() as usize];
        if self.equal_flag {
          self.pc = target as usize
        }
      },
      Opcode::JNEQ => {
        let target = self.registers[self.next_8_bits() as usize];
        if !self.equal_flag {
          self.pc = target as usize
        }
      },
      Opcode::INC => {
        self.registers[self.next_8_bits() as usize] += 1;
        self.next_16_bits();
      },
      Opcode::DEC => {
        self.registers[self.next_8_bits() as usize] -= 1;
        self.next_16_bits();
      },
      Opcode::IGL => ()
    }

    false
  }

  fn decode_opcode(&mut self) -> Opcode {
    let opcode = Opcode::from(self.program[self.pc]);
    self.pc += 1;
    return opcode;
  }

  fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    return result;
  }

  fn next_16_bits(&mut self) -> u16 {
    let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
    self.pc += 2;
    return result;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_test_vm(bytes: Vec<u8>) -> VM {
    let mut vm = VM::new();
    vm.program = bytes;
    vm
  }

  #[test]
  fn test_create_vm() {
    let test_vm = VM::new();
    assert_eq!(test_vm.registers[0], 0);
  }

  #[test]
  fn test_opcode_hlt() {
    let hlt = Opcode::HLT as u8;
    let mut vm = create_test_vm(vec![hlt, 0, 0, 0]);
    vm.run_once();
    assert_eq!(vm.pc, 1);
  }

  #[test]
  fn test_opcode_igl() {
    let mut vm = create_test_vm(vec![185, 0, 0, 0]);
    vm.run_once();
    assert_eq!(vm.pc, 1);
  }

  #[test]
  fn test_load_opcode() {
    let load = Opcode::LOAD as u8;
    let mut vm = create_test_vm(vec![load, 0, 1, 244]);
    vm.run_once();
    assert_eq!(vm.registers[0], 500);
  }

  #[test]
  fn test_add_opcode() {
    let load = Opcode::LOAD as u8;
    let add  = Opcode::ADD as u8;
    let mut vm = create_test_vm(vec![load, 0, 0, 5, load, 1, 0, 3, add, 0, 1, 2]);
    vm.run();
    assert_eq!(vm.registers[0], 5);
    assert_eq!(vm.registers[1], 3);
    assert_eq!(vm.registers[2], 8);
  }

  #[test]
  fn test_sub_opcode() {
    let load = Opcode::LOAD as u8;
    let sub  = Opcode::SUB as u8;
    let mut vm = create_test_vm(vec![load, 0, 0, 5, load, 1, 0, 3, sub, 0, 1, 2]);
    vm.run();
    assert_eq!(vm.registers[0], 5);
    assert_eq!(vm.registers[1], 3);
    assert_eq!(vm.registers[2], 2);
  }

  #[test]
  fn test_mul_opcode() {
    let load = Opcode::LOAD as u8;
    let mul  = Opcode::MUL as u8;
    let mut vm = create_test_vm(vec![load, 0, 0, 5, load, 1, 0, 3, mul, 0, 1, 2]);
    vm.run();
    assert_eq!(vm.registers[0], 5);
    assert_eq!(vm.registers[1], 3);
    assert_eq!(vm.registers[2], 15);
  }

  #[test]
  fn test_div_opcode() {
    let load = Opcode::LOAD as u8;
    let div  = Opcode::DIV as u8;
    let mut vm = create_test_vm(vec![load, 0, 0, 16, load, 1, 0, 5, div, 0, 1, 2]);
    vm.run();

    assert_eq!(vm.registers[0], 16);
    assert_eq!(vm.registers[1], 5);
    assert_eq!(vm.registers[2], 3);
    assert_eq!(vm.remainder, 1);
  }

  #[test]
  fn test_div_opcode_on_operand_register() {
    let load = Opcode::LOAD as u8;
    let div  = Opcode::DIV as u8;
    let mut vm = create_test_vm(vec![load, 0, 0, 16, load, 1, 0, 5, div, 0, 1, 0]);
    vm.run();

    assert_eq!(vm.registers[0], 3);
    assert_eq!(vm.registers[1], 5);
    assert_eq!(vm.remainder, 1);
  }

  #[test]
  fn test_jmp_opcode() {
    let jmp = Opcode::JMP as u8;
    let mut vm = VM::new();
    vm.registers[0] = 5;
    vm.program = vec![jmp, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.pc, 5);
  }

  #[test]
  fn test_jmpf_opcode() {
    let jmpf = Opcode::JMPF as u8;
    let mut vm = VM::new();
    vm.registers[0] = 2;
    vm.program = vec![jmpf, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.pc, 4);
  }

  #[test]
  fn test_jmpb_opcode() {
    let jmpb = Opcode::JMPB as u8;
    let mut vm = VM::new();
    vm.registers[0] = 1;
    vm.program = vec![jmpb, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.pc, 1);
  }

  #[test]
  fn test_eq_opcode() {
    let eq = Opcode::EQ as u8;
    let mut vm = VM::new();

    vm.registers[0] = 10;
    vm.registers[1] = 10;
    vm.program = vec![eq, 0, 1, 0, eq, 0, 1, 0];
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.registers[1] = 20;
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_neq_opcode() {
    let neq = Opcode::NEQ as u8;
    let mut vm = VM::new();

    vm.registers[0] = 10;
    vm.registers[1] = 10;
    vm.program = vec![neq, 0, 1, 0, neq, 0, 1, 0];
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
    vm.registers[1] = 20;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
  }

  #[test]
  fn test_gt_opcode() {
    let gt = Opcode::GT as u8;
    let mut vm = VM::new();

    vm.registers[0] = 10;
    vm.registers[1] = 10;
    vm.program = vec![gt, 0, 1, 0, gt, 0, 1, 0];
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
    vm.registers[0] = 20;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
  }

  #[test]
  fn test_lt_opcode() {
    let lt = Opcode::LT as u8;
    let mut vm = VM::new();

    vm.registers[0] = 8;
    vm.registers[1] = 10;
    vm.program = vec![lt, 0, 1, 0, lt, 0, 1, 0];
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.registers[0] = 20;
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_gtq_opcode() {
    let gtq = Opcode::GTQ as u8;
    let mut vm = VM::new();

    vm.registers[0] = 10;
    vm.registers[1] = 10;
    vm.program = vec![gtq, 0, 1, 0, gtq, 0, 1, 0];
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.registers[0] = 8;
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_ltq_opcode() {
    let ltq = Opcode::LTQ as u8;
    let mut vm = VM::new();

    vm.registers[0] = 10;
    vm.registers[1] = 10;
    vm.program = vec![ltq, 0, 1, 0, ltq, 0, 1, 0];
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.registers[0] = 20;
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_jeq_opcode() {
    let jeq = Opcode::JEQ as u8;
    let mut vm = VM::new();
    vm.registers[0] = 7;
    vm.equal_flag = true;
    vm.program = vec![jeq, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.pc, 7);
  }

  #[test]
  fn test_jneq_opcode() {
    let jneq = Opcode::JNEQ as u8;
    let mut vm = VM::new();
    vm.registers[0] = 7;
    vm.program = vec![jneq, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.pc, 7);
  }

  #[test]
  fn test_inc_opcode() {
    let inc = Opcode::INC as u8;
    let mut vm = VM::new();
    vm.program = vec![inc, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.pc, 4);
    assert_eq!(vm.registers[0], 1);
  }

  #[test]
  fn test_dec_opcode() {
    let dec = Opcode::DEC as u8;
    let mut vm = VM::new();
    vm.registers[0] = 7;
    vm.program = vec![dec, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.pc, 4);
    assert_eq!(vm.registers[0], 6);
  }
}