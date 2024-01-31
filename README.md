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

### Debugger

I added a debugger to the game, which can be activated with the '>' + enter command.

### Challenges

Main difficulties I encountered for the 6 first codes:

- I didn't realize at first that the memory was a single address space for instructions and data.
- At first I assumed the maze should be solved using a program, before accepting that the best was to solve it by hand. Here is the [map](resources/maze_map.svg).
- The twisty passage part in the maze was a pain to get done, with luck involved to finally find it.
- I forgot that the "look" command can be used to inspect objects, like coins or books..

### Code 7

That one is *very* hard.

Using the decompiled code, I found where the method that is doing the register 8 check is called, and skipped it:

    > setm 5511 21
    Memory at 5511 set to 21
    > setm 5512 21
    Memory at 5512 set to 21
    > setm 5516 4
    Memory at 5516 set to 4
    > setr 7 44
    Register r7 set to 44
    > q
    Quitting debugger
    use teleporter

But the resulting code is not the right one.