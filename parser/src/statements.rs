use m68k_reloaded_common::Range;
use std::ops::Deref;

/// A statement in the abstract syntax tree built by the parser. Wraps a value
/// like [Size] or [EffectiveAddress] and adds information where to find the
/// corresponding code in the source file.
/// Not to be confused with [Statement], which represents (about) a single line
/// of assembly code.
#[derive(Eq, PartialEq, Debug)]
pub struct Stmt<T> {
    pub range: Range,
    pub value: T,
}
impl<T> Deref for Stmt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub type Byte = u8;
pub type Word = u16;
pub type LongWord = u32;

pub type RegisterIndex = Byte;

#[derive(Eq, PartialEq, Debug)]
pub struct An {
    pub index: RegisterIndex,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Dn {
    pub index: RegisterIndex,
}

/// Wrapper around [An] and [Dn]. Should only be used in contexts where [An]
/// and [Dn] are the only options.
#[derive(Eq, PartialEq, Debug)]
pub enum Xn {
    An(Stmt<An>),
    Dn(Stmt<Dn>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum EffectiveAddress {
    Dn(Stmt<Dn>),
    An(Stmt<An>),
    AnInd(Stmt<An>),
    AnIndWithPostInc(Stmt<An>),
    AnIndWithPreDec(Stmt<An>),
    AnIndWithDisplacement(Stmt<Word>, Stmt<An>),
    AnIndWithIndex(Stmt<Byte>, Stmt<An>, Stmt<Xn>),
    AbsoluteWord(Stmt<Word>),
    AbsoluteLongWord(Stmt<LongWord>),
    PcIndWithDisplacement(Stmt<Word>),
    PcIndWithIndex(Stmt<Byte>, Stmt<Xn>),
}

// impl std::string::ToString for Register {
//     fn to_string(&self) -> String {
//         match self {
//             Register::PC => String::from("PC"),
//             Register::SP => String::from("SP"),
//             Register::An(n) => format!("A{}", n),
//             Register::Dn(n) => format!("D{}", n),
//         }
//     }
// }

pub type Operand = EffectiveAddress;

#[derive(Eq, PartialEq, Debug)]
pub enum OperationType {
    Add,
    Adda,
    Addi,
    Addq,
    Addx, // ...
}

#[derive(Eq, PartialEq, Debug)]
pub enum Size {
    Byte,
    Word,
    LongWord,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Operation {
    pub operation_type: Stmt<OperationType>,
    pub size: Stmt<Size>,
    pub operands: Vec<Stmt<Operand>>,
}

pub type Comment = String;

pub type Label = String;

/// About a single line in the assembler program.
#[derive(Eq, PartialEq, Debug)]
pub enum Statement {
    Label(Label),
    Operation(Operation),
    Comment(Comment),
}

pub type Program = Vec<Stmt<Statement>>;
