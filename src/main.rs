mod codes;
mod maze;
mod vm;

fn main() {
    // To decompile the binary:
    // vm::decompiler::decompile();

    // To run the program without saved commands:
    // vm::run::execute_program(&[]);

    vm::run::execute_program(&maze::maze_commands::COMMANDS);
}
