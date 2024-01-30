# Synacor Challenge

My implementation of the Synacor Challenge.

Here is a [copy of the binary and spec of the challenge](https://github.com/Aneurysm9/vm_challenge), if you want to do it as well.

## Running the binary

    cargo run --release

## Codes

The challenge was to find a serie of 8 codes. We know if the codes are correct by matching them against the MD5 hash of the correct codes. Codes are checked by the program tests:

    cargo test
