mod codes;
mod maze;
mod vm;

fn main() {
    maze::teleporter_code::check_fn6049();
    // To find the teleporter code (warning: slow):
    // maze::teleporter_code::find_teleporter_code();

    // To decompile the binary:
    // vm::decompiler::decompile();

    // To run the program without saved commands:
    // vm::run::execute_program(&[]);

    vm::run::execute_program(&maze::maze_commands::COMMANDS);
}
