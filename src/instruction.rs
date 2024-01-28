// The numbers in the binary format can mean two things: A literal value or a register number.
#[derive(Debug, Clone, Copy)]
pub enum Number {
    Value(u16),
    Register(usize),
}

impl Number {
    fn new(n: u16) -> Self {
        match n {
            0..=32767 => Number::Value(n),
            32768..=32775 => Number::Register((n - 32768) as usize),
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
    Set(Number, Number),
    // push <a> onto the stack
    Push(Number),
    // remove the top element from the stack and write it into <a>; empty stack = error
    Pop(Number),
    // set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    Eq(Number, Number, Number),
    // set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    Gt(Number, Number, Number),
    // jump to <a>
    Jmp(Number),
    // if <a> is nonzero, jump to <b>
    Jt(Number, Number),
    // if <a> is zero, jump to <b>
    Jf(Number, Number),
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    Add(Number, Number, Number),
    // store into <a> the product of <b> and <c> (modulo 32768)
    Mult(Number, Number, Number),
    // store into <a> the remainder of <b> divided by <c>
    Mod(Number, Number, Number),
    // stores into <a> the bitwise and of <b> and <c>
    And(Number, Number, Number),
    // stores into <a> the bitwise or of <b> and <c>
    Or(Number, Number, Number),
    // stores 15-bit bitwise inverse of <b> in <a>
    Not(Number, Number),
    // read memory at address <b> and write it to <a>
    RMem(Number, Number),
    // write the value from <b> into memory at address <a>
    WMem(Number, Number),
    // write the address of the next instruction to the stack and jump to <a>
    Call(Number),
    // remove the top element from the stack and jump to it; empty stack = halt
    Ret,
    // write the character represented by ascii code <a> to the terminal
    Out(Number),
    // read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard instead of having to figure out how to read individual characters
    In(Number),
    // no operation
    Noop,
}

macro_rules! next {
    ($iter:expr) => {
        Number::new(*$iter.next().unwrap())
    };
}

pub fn build(bin: &[u16]) -> Vec<Instruction> {
    use Instruction::*;
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut iter = bin.iter();
    while let Some(opcode) = iter.next() {
        instructions.push(match *opcode {
            0 => Halt,
            1 => Set(next!(iter), next!(iter)),
            2 => Push(next!(iter)),
            3 => Pop(next!(iter)),
            4 => Eq(next!(iter), next!(iter), next!(iter)),
            5 => Gt(next!(iter), next!(iter), next!(iter)),
            6 => Jmp(next!(iter)),
            7 => Jt(next!(iter), next!(iter)),
            8 => Jf(next!(iter), next!(iter)),
            9 => Add(next!(iter), next!(iter), next!(iter)),
            10 => Mult(next!(iter), next!(iter), next!(iter)),
            11 => Mod(next!(iter), next!(iter), next!(iter)),
            12 => And(next!(iter), next!(iter), next!(iter)),
            13 => Or(next!(iter), next!(iter), next!(iter)),
            14 => Not(next!(iter), next!(iter)),
            15 => RMem(next!(iter), next!(iter)),
            16 => WMem(next!(iter), next!(iter)),
            17 => Call(next!(iter)),
            18 => Ret,
            19 => Out(next!(iter)),
            20 => In(next!(iter)),
            21 => Noop,
            _ => {
                // There are some invalid instructions, ignoring them for now.
                Noop
            }
        });
    }
    instructions
}
