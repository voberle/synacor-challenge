// Instructions are in the order of the spec, meaning their variant is the opcode.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // stop execution and terminate the program
    Halt,
    // set register <a> to the value of <b>
    Set(u16, u16),
    // push <a> onto the stack
    Push(u16),
    // remove the top element from the stack and write it into <a>; empty stack = error
    Pop(u16),
    // set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    Eq(u16, u16, u16),
    // set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    Gt(u16, u16, u16),
    // jump to <a>
    Jmp(u16),
    // if <a> is nonzero, jump to <b>
    Jt(u16, u16),
    // if <a> is zero, jump to <b>
    Jf(u16, u16),
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    Add(u16, u16, u16),
    // store into <a> the product of <b> and <c> (modulo 32768)
    Mult(u16, u16, u16),
    // store into <a> the remainder of <b> divided by <c>
    Mod(u16, u16, u16),
    // stores into <a> the bitwise and of <b> and <c>
    And(u16, u16, u16),
    // stores into <a> the bitwise or of <b> and <c>
    Or(u16, u16, u16),
    // stores 15-bit bitwise inverse of <b> in <a>
    Not(u16, u16),
    // read memory at address <b> and write it to <a>
    RMem(u16, u16),
    // write the value from <b> into memory at address <a>
    WMem(u16, u16),
    // write the address of the next instruction to the stack and jump to <a>
    Call(u16),
    // remove the top element from the stack and jump to it; empty stack = halt
    Ret,
    // write the character represented by ascii code <a> to the terminal
    Out(u16),
    // read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard instead of having to figure out how to read individual characters
    In(u16),
    // no operation
    Noop,
}

pub fn build(bin: &[u16]) -> Vec<Instruction> {
    use Instruction::*;
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut iter = bin.iter();
    while let Some(opcode) = iter.next() {
        instructions.push(match *opcode {
            0 => Halt,
            1 => Set(*iter.next().unwrap(), *iter.next().unwrap()),
            2 => Push(*iter.next().unwrap()),
            3 => Pop(*iter.next().unwrap()),
            4 => Eq(
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ),
            5 => Gt(
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ),
            6 => Jmp(*iter.next().unwrap()),
            7 => Jt(*iter.next().unwrap(), *iter.next().unwrap()),
            8 => Jf(*iter.next().unwrap(), *iter.next().unwrap()),
            9 => Add(
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ),
            10 => Mult(
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ),
            11 => Mod(
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ),
            12 => And(
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ),
            13 => Or(
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ),
            14 => Not(*iter.next().unwrap(), *iter.next().unwrap()),
            15 => RMem(*iter.next().unwrap(), *iter.next().unwrap()),
            16 => WMem(*iter.next().unwrap(), *iter.next().unwrap()),
            17 => Call(*iter.next().unwrap()),
            18 => Ret,
            19 => Out(*iter.next().unwrap()),
            20 => In(*iter.next().unwrap()),
            21 => Noop,
            _ => {
                // There are some invalid instructions, ignoring them for now.
                Noop
            }
        });
    }
    instructions
}
