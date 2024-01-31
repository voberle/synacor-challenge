# Synacor Challenge

My implementation of the Synacor Challenge.

Here is a clean, spoiler-free [copy of the binary and spec of the challenge](https://github.com/Aneurysm9/vm_challenge), if you want to do it as well.

## Running the binary

    cargo run --release

## Codes

The challenge was to find a serie of 8 codes. We know if the codes are correct by matching them against the MD5 hash of the correct codes. Codes are checked by the program tests:

    cargo test

---

**WARNING: SPOILERS BELOW**

---

## Small overview

Main difficulties I encountered:

- I didn't realize at first that the memory was a single address space for instructions and data.
- At first I assumed the maze should be solved using a program, before accepting that the best was to solve it by hand. Here is the [map](resources/maze_map.svg).
- The twisty passage part in the maze was a pain to get done, with luck involved to finally find it.