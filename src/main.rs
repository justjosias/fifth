use std::io::Write;

mod stack;
use stack::Stack;

fn main() {
    let mut args = std::env::args();

    if args.len() < 2 {
        let mut vm = VM::new(std::io::stdout());
        vm.init_dict().unwrap();
        loop {
            let mut line = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut line).unwrap();
            match vm.run(&line) {
                Ok(()) => {
                    if line != "\n" {
                        print!(" ok\n");
                    }
                }
                Err(Error::Overflow) => {
                    eprintln!(" Stack overflow")
                }
                Err(Error::Underflow) => {
                    eprintln!(" Stack underflow")
                }
                Err(Error::UnknownWord(word)) => {
                    eprintln!(" {} ?", word)
                }
                Err(Error::Bye) => {
                    break;
                } //Err(err) => panic!("{:?}", err),
            };
        }
    } else {
        let code = std::fs::read_to_string(args.nth(1).unwrap()).unwrap();
        let mut vm = VM::new(std::io::stdout());
        vm.init_dict().unwrap();
        vm.run(&code).unwrap();
    }
}

#[derive(Debug)]
pub enum Error {
    Overflow,
    Underflow,
    Bye,
    UnknownWord(String),
}

impl From<stack::Error> for Error {
    fn from(err: stack::Error) -> Self {
        match err {
            stack::Error::Overflow => Error::Overflow,
            stack::Error::Underflow => Error::Underflow,
        }
    }
}

const BUILTIN: &'static str = r#"
: print . cr ;
"#;

pub struct VM<W> {
    stack: Stack<i64>,
    out: W,
    words: std::collections::HashMap<String, String>,
}

impl<W: Write> VM<W> {
    pub fn new(out: W) -> Self {
        Self {
            stack: Stack::new(0),
            out,
            words: std::collections::HashMap::new(),
        }
    }

    pub fn init_dict(&mut self) -> Result<(), Error> {
        self.run(BUILTIN)?;
        Ok(())
    }

    pub fn run(&mut self, code: &str) -> Result<(), Error> {
        let mut in_comment = false;
        let mut in_word = false;
        let mut word_name = None;
        let mut word = String::new();
        for command in code.split_whitespace() {
            if in_comment && command != ")" {
                continue;
            }

            if in_word == true && word_name == None {
                word_name = Some(command);
                continue;
            } else if in_word == true && command != ";" {
                word.push_str(command);
                word.push_str(" ");
                continue;
            }

            if let Ok(num) = command.parse::<i64>() {
                self.stack.push(num)?;
            } else {
                match command.to_uppercase().as_str() {
                    "+" => {
                        self.stack.add()?;
                    }
                    "-" => {
                        self.stack.sub()?;
                    }
                    "*" => {
                        self.stack.mul()?;
                    }
                    "/" => {
                        self.stack.div()?;
                    }
                    "." => {
                        self.print()?;
                    }
                    ":" => {
                        in_word = true;
                        word_name = None;
                        word = "".to_string();
                    }
                    ";" => {
                        self.words
                            .insert(word_name.unwrap().to_string().to_uppercase(), word.clone());

                        in_word = false;
                        word_name = None;
                        word = "".to_string();
                    }
                    "<" => {
                        self.stack.lt()?;
                    }
                    ">" => {
                        self.stack.gt()?;
                    }
                    "=" => {
                        self.stack.equals()?;
                    }
                    "CR" => {
                        write!(self.out, "\n").unwrap();
                    }
                    "DUP" => {
                        self.stack.dup()?;
                    }
                    "SWAP" => {
                        self.stack.swap()?;
                    }
                    "OVER" => {
                        self.stack.over()?;
                    }
                    "EMIT" => {
                        self.emit()?;
                    }
                    "DROP" => {
                        let _ = self.stack.pop()?;
                    }
                    "ROT" => {
                        self.stack.rot()?;
                    }
                    "BYE" => {
                        return Err(Error::Bye);
                    }
                    ".S" => {
                        write!(self.out, "{:?}\n", &self.stack.items[0..self.stack.top]).unwrap();
                    }
                    "(" => {
                        in_comment = true;
                    }
                    ")" => {
                        in_comment = false;
                        // TODO if not in comment, error
                    }
                    word => {
                        self.run_word(word)?;
                        // recursive, alternative: use call stack
                    }
                }
            }
        }

        Ok(())
    }

    pub fn push_bool(&mut self, b: bool) -> Result<(), Error> {
        if b {
            self.stack.push(-1)?;
        } else {
            self.stack.push(0)?;
        }
        Ok(())
    }

    pub fn run_word(&mut self, word: &str) -> Result<(), Error> {
        let words = self.words.clone(); // TODO don't clone
        let commands = words.get(word);
        if let Some(commands) = commands {
            self.run(commands.as_str())?;
        } else {
            return Err(Error::UnknownWord(word.to_string()));
        }
        Ok(())
    }

    pub fn print(&mut self) -> Result<(), stack::Error> {
        let a = self.stack.pop()?;
        write!(self.out, "{}", a).unwrap();
        Ok(())
    }

    pub fn emit(&mut self) -> Result<(), stack::Error> {
        let a = self.stack.pop()?;
        write!(
            self.out,
            "{}",
            std::char::from_u32(a.try_into().unwrap()).unwrap()
        )
        .unwrap();
        Ok(())
    }
}
