

enum InstructionType {
  AInstruction,
  CInstruction,
  LInstruction,
}

pub struct Parser {
  pub input: String,
  pub index: usize,
}

impl Parser {
  pub fn new(input: String) -> Self {
      Self { input, index: 0 }
  }

  pub fn has_more_lines(&self) -> bool {
    let lines: Vec<&str> = self.input.split("\n").collect();
    lines.len() > self.index
  }

//   fn advance(&self) {
//       todo!()
//   }

//   fn instruction_type(&self) -> InstructionType {
//       todo!()
//   }

//   fn symbol(&self) -> String {
//       todo!()
//   }

//   fn dest(&self) -> String {
//       todo!()
//   }

//   fn comp(&self) -> String {
//       todo!()
//   }

//   fn jump(&self) -> String {
//       todo!()
//   }
}



#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn init_new_parser_with_test_file() {
        let input = read_to_string("asm-files/test.asm").unwrap();
        let parser = Parser::new(input);
        assert_eq!(parser.input, "// Computes R1=1+...+R0\n// i = 1\n@i".to_string());
    }

    #[test]
    fn check_if_there_is_more_lines() {
        let input = read_to_string("asm-files/test.asm").unwrap();
        let parser = Parser::new(input);
        assert_eq!(parser.has_more_lines(), true)
    }
}