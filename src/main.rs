mod codes;
mod vm;

use std::collections::VecDeque;

use vm::instructions::get_instruction;
use vm::storage::Storage;
use vm::terminal::Terminal;

const DEBUG: bool = false;

fn main() {
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(!DEBUG);
    let mut ir: u16 = 0;

    let mut saved_actions = VecDeque::from(vec![
        "go doorway",
        "go north",
        "go north",
        "go bridge",
        "go continue",
        "go down",
        "go east",
        "take empty lantern",
        "go west",
        "go west",
        "go passage",
        "go ladder",
        "go west",
        "go south",
        "go north", // code
        "take can",
        "go west",
        "go ladder",
        "go darkness",
        "use can",
        "use lantern",
        "go continue",
        "go west",
        "go west",
        "go west",
        "go west",
        "go north",
        "take red coin",
        "go north",
        "go east",
        "take concave coin",
        "go down",
        "take corroded coin",
        "go up",
        "go west",
        "go west",
        "take blue coin",
        "go up",
        "take shiny coin",
        "go down",
        "go east",
        "use blue coin",
        "use red coin",
        "use shiny coin",
        "use concave coin",
        "use corroded coin",
        "go north",
        "take teleporter",
        "use teleporter", // code
        "take business card",
        "take strange book",
    ]);

    let mut i = 0;
    loop {
        let ins = get_instruction(&storage, ir);
        if DEBUG {
            // println!("\t{}", storage.regs);
            println!("{i}: [{}] {}", ir, ins);
            i += 1;
        }
        if ins.name() == "in" && terminal.is_input_empty() {
            if !saved_actions.is_empty() {
                let next_action = saved_actions.pop_front().unwrap();
                let cmd = format!("{}\n", next_action);
                terminal.set_input(&cmd);
            }
        }

        ins.exec(&mut ir, &mut storage, &mut terminal);
    }
}
