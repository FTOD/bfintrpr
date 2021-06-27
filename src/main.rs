mod parse;
mod machine;

use crate::parse::stmts;
use crate::machine::Machine;

fn main() {
    let res = stmts(b"++[.-");
    match res{
        Ok((_, stmts)) => {
            let mut machine = Machine::new();
            for stmt in stmts{
                machine.exec(&stmt);
            }
        }
        _ => {
            println!("ERROR!");
        }
    }
}
