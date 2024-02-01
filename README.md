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

That one is *very* hard. My [analysis is here](teleport_code_analysis.md).

### Code 8

Once I found the orb and [draw the map](resources/island_map.svg), it was fairly clear what needed to do: Find the shortest path that gives the correct result.

I implemented a rough recursive approach. Since nodes can be visited multiple times, the trick was to add enough limits to the algorithm so that it ends.