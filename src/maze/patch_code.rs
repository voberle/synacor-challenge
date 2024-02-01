use crate::vm::{register::RegNb, storage::Storage};

use super::teleporter_code::TELEPORTER_CODE;

// Patch the binary to allow to by-pass the teleporter check.
pub fn patch(storage: &mut Storage) {
    // Replace call with noop
    storage.mem.write(5511, 21);
    storage.mem.write(5512, 21);

    // Change the check to always pass
    storage.mem.write(5516, 4);

    // Set the register 8 to correct value
    storage.regs.set(RegNb::new(7), TELEPORTER_CODE);

    println!("Teleported code patched!");
}
