pub struct Machine{
    sp: u16,
    mem: [u8; 1024],
}

use crate::parse::Statement;

impl Machine{
    pub fn exec(&mut self, inst: &Statement) -> &mut Machine{
        match inst {
            Statement::PtrIncr => {self.sp = self.sp + 1;},
            Statement::PtrDecr => {self.sp = self.sp - 1;},
            Statement::Incr => {self.mem[self.sp as usize] = self.mem[self.sp as usize] + 1;},
            Statement::Decr => {self.mem[self.sp as usize] = self.mem[self.sp as usize] - 1;},
            Statement::PutChar => {println!("{}", self.mem[self.sp as usize].to_string());},
            Statement::Loop(stmts) => {
                while self.mem[self.sp as usize]!= 0{
                    for stmt in stmts{
                        self.exec(stmt);
                    }
                }
            }
        }
        self
    }

    pub fn new()-> Machine{
        Machine{
            sp: 0,
            mem: [0; 1024],
        }
    }
}