use std::{fs::{read_to_string, File}, io};



enum CommandType {
  C_ARITHMETIC,
  C_PUSH,
  C_POP,
  C_LABEL,
  C_GOTO,
  C_IF,
  C_FUNCTION,
  C_RETURN,
  C_CALL
}

struct Parser {
  file: String
}

impl Parser {
  // Opens the input file/stream, and gets ready to parse it
  fn new(path: String) -> Result<Self, io::Error> {
    let file = read_to_string(path)?;

    Ok(Self { file: file })
  }

  // Are there more lines in the input ?
  fn has_more_lines(&self) {
    todo!()
  }

  // Reads the next command from the input and makes it the current command
  fn advance(&self) {
    todo!()
  }

  // Returns a constant representing the type of the current command.
  fn command_type(&self) -> CommandType {
    todo!()
  }

  // Returns the first argument of the current command
  fn arg1(&self) -> String {
    todo!()
  }

  // Returns the second argument of the current command
  fn arg2(&self) {
    todo!()
  }
}