// The numbers in the binary format can mean two things: A literal value or a register number.
#[derive(Debug, Clone, Copy)]
pub enum IntReg {
    Value(u16),
    Register(u16),
}

impl IntReg {
    fn new(n: u16) -> Self {
        match n {
            0..=32767 => IntReg::Value(n),
            32768..=32775 => IntReg::Register(n - 32768),
            _ => panic!("Invalid number"),
        }
    }
}

// Instructions are in the order of the spec, meaning their variant is the opcode.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // stop execution and terminate the program
    Halt,
    // set register <a> to the value of <b>
    Set(IntReg, IntReg),
    // push <a> onto the stack
    Push(IntReg),
    // remove the top element from the stack and write it into <a>; empty stack = error
    Pop(IntReg),
    // set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    Eq(IntReg, IntReg, IntReg),
    // set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    Gt(IntReg, IntReg, IntReg),
    // jump to <a>
    Jmp(IntReg),
    // if <a> is nonzero, jump to <b>
    Jt(IntReg, IntReg),
    // if <a> is zero, jump to <b>
    Jf(IntReg, IntReg),
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    Add(IntReg, IntReg, IntReg),
    // store into <a> the product of <b> and <c> (modulo 32768)
    Mult(IntReg, IntReg, IntReg),
    // store into <a> the remainder of <b> divided by <c>
    Mod(IntReg, IntReg, IntReg),
    // stores into <a> the bitwise and of <b> and <c>
    And(IntReg, IntReg, IntReg),
    // stores into <a> the bitwise or of <b> and <c>
    Or(IntReg, IntReg, IntReg),
    // stores 15-bit bitwise inverse of <b> in <a>
    Not(IntReg, IntReg),
    // read memory at address <b> and write it to <a>
    RMem(IntReg, IntReg),
    // write the value from <b> into memory at address <a>
    WMem(IntReg, IntReg),
    // write the address of the next instruction to the stack and jump to <a>
    Call(IntReg),
    // remove the top element from the stack and jump to it; empty stack = halt
    Ret,
    // write the character represented by ascii code <a> to the terminal
    Out(IntReg),
    // read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard instead of having to figure out how to read individual characters
    In(IntReg),
    // no operation
    Noop,
}

macro_rules! nreg {
    ($iter:expr) => {
        dbg!(*$iter.next().unwrap()) - 32768
    };
}

macro_rules! nir {
    ($iter:expr) => {
        IntReg::new(*$iter.next().unwrap())
    };
}

pub fn build(bin: &[u16]) -> Vec<Instruction> {
    use Instruction::*;
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut iter = bin.iter();
    while let Some(opcode) = iter.next() {
        instructions.push(match *opcode {
            0 => Halt,
            1 => Set(nir!(iter), nir!(iter)),
            2 => Push(nir!(iter)),
            3 => Pop(nir!(iter)),
            4 => Eq(nir!(iter), nir!(iter), nir!(iter)),
            5 => Gt(nir!(iter), nir!(iter), nir!(iter)),
            6 => Jmp(nir!(iter)),
            7 => Jt(nir!(iter), nir!(iter)),
            8 => Jf(nir!(iter), nir!(iter)),
            9 => Add(nir!(iter), nir!(iter), nir!(iter)),
            10 => Mult(nir!(iter), nir!(iter), nir!(iter)),
            11 => Mod(nir!(iter), nir!(iter), nir!(iter)),
            12 => And(nir!(iter), nir!(iter), nir!(iter)),
            13 => Or(nir!(iter), nir!(iter), nir!(iter)),
            14 => Not(nir!(iter), nir!(iter)),
            15 => RMem(nir!(iter), nir!(iter)),
            16 => WMem(nir!(iter), nir!(iter)),
            17 => Call(nir!(iter)),
            18 => Ret,
            19 => Out(nir!(iter)),
            20 => In(nir!(iter)),
            21 => Noop,
            _ => {
                // There are some invalid instructions, ignoring them for now.
                Noop
            }
        });
    }
    instructions
}
