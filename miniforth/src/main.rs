use std::io::Write;

use miniforth::forth::*;

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
