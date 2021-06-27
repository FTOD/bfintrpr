extern crate nom;

use nom::sequence::delimited;
use nom::{IResult};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::multi::many0;
use nom::error::{ErrorKind, ParseError};
use nom::Err::*;

pub enum BfParseErr<I> {
    SyntaxError(String),
    Unimplemented,
    NomError(I, ErrorKind)
}

impl<I> ParseError<I> for BfParseErr<I>{
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        BfParseErr::NomError(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self{
        other
    }
}

pub enum Statement{
    Loop(Vec<Statement>),
    PtrIncr,
    PtrDecr,
    Incr,
    Decr,
    PutChar,
}

impl Statement{
    pub fn to_string(&self)-> String{
        match self {
            Statement::Loop(stmts) => {
                let mut str = String::from("LOOP{\n");
                for stmt in stmts{
                    str.push_str(&stmt.to_string());
                }
                str.push_str("}\n");
                str
            },
            Statement::PtrIncr => String::from("ptr++\n"),
            Statement::PtrDecr => String::from("ptr--\n"),
            Statement::Incr => String::from("++\n"),
            Statement::Decr => String::from("--\n"),
            Statement::PutChar => String::from("print;\n"),

        }
    }
}

fn inst(input:&[u8]) -> IResult<&[u8], Statement, BfParseErr<&[u8]>>{
    let res :IResult<&[u8],&[u8], BfParseErr<&[u8]>> = alt((tag(">"), tag("<"), tag("+"), tag("-"), tag("."), tag(",")))(input);
    match res{
        Ok((rest, b">"))  => Ok((rest, Statement::PtrIncr)),
        Ok((rest, b"<"))  => Ok((rest, Statement::PtrDecr)),
        Ok((rest, b"+"))  => Ok((rest, Statement::Incr)),
        Ok((rest, b"-"))  => Ok((rest, Statement::Decr)),
        Ok((rest, b"."))  => Ok((rest, Statement::PutChar)),
        Ok((_, b","))           => Err(Failure(BfParseErr::Unimplemented)),
        Err(err)=>Err(err),
        _                       => Err(Failure(BfParseErr::SyntaxError(String::from("A")))),
    }
}

fn closure(input: &[u8]) -> IResult<&[u8], &[u8], BfParseErr<&[u8]>> {
    let res = tag("]")(input);
    match res{
        Ok(_) => res,
        Err(_) => Err(Failure(BfParseErr::SyntaxError(String::from("Parenthesis not matched")))),
    }
}

fn stmt(input: &[u8]) -> IResult<&[u8], Statement, BfParseErr<&[u8]>>{
    // If it is simple instructions
    let res:IResult<&[u8], Statement, BfParseErr<&[u8]>> = inst(input);
    if let Ok((_,ref taken)) = res {
        println!("char taken:{}", taken.to_string());
        return res;
    }

    
    let res :IResult<&[u8], Vec<Statement>, BfParseErr<&[u8]>> = delimited(tag("["), stmts, closure)(input);

    match res{
        Err(e) => Err(e),
        Ok((rest, stmts)) => Ok((rest, Statement::Loop(stmts))),
    }
}

pub fn stmts(input: &[u8]) -> IResult<&[u8], Vec<Statement>, BfParseErr<&[u8]>>{
    many0(stmt)(input)
}