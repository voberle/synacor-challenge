mod codes;
mod maze;
mod vm;

use vm::run::execute_program;

fn main() {
    execute_program(&maze::maze_commands::COMMANDS);
    // execute_program(&[], false);
}
