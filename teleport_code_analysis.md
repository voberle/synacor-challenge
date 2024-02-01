# Teleport code analyis.

Having a clean decompiled version of the code is very useful here. See my [decompiler](src/vm/decompiler.rs).

## By-passing the check

Using the decompiled code and the debugging support, I found where the method that is doing the register 8 check is called:

    5505	set	r0	4
    5508	set	r1	1
    5511	call	6049
    5513	eq	r1	r0	6

6049 is the method, and the check is just after.

We can skip this check by replacing the call with noop:

    > setm 5511 21
    Memory at 5511 set to 21
    > setm 5512 21
    Memory at 5512 set to 21

And hacking the check after:

    > setm 5516 4
    Memory at 5516 set to 4

Setting the register 8 to a non-zero value:

    > setr 7 44
    Register r7 set to 44
    > q
    Quitting debugger
    use teleporter

allows to by-pass the check, but the resulting code is not the right one.

## Reverse-engineering the checking code

We need to understand what the checking code does, so we can which value to use for register 8.

From how the checking code is called, we know that it gets r0 and r1 as parameter, and the result is put in r0, as this is checked against 6.
The checking function (let's call it `fn6049`) is therefore called with 4 and 0 as parameters and must return 6.

Here is everything that happens at 6049:

    6049	jt	r0	6057
    6052	add	r0	r1	1
    6056	ret
    6057	jt	r1	6070
    6060	add	r0	r0	32767
    6064	set	r1	r7
    6067	call	6049
    6069	ret
    6070	push	r0
    6072	add	r1	r1	32767
    6076	call	6049
    6078	set	r1	r0
    6081	pop	r0
    6083	add	r0	r0	32767
    6087	call	6049
    6089	ret

The important thing to realize is that `ret` marks the end of functions, meaning we have actually 3 functions there.

### `fn6049`

    6049	jt	r0	6057
    6052	add	r0	r1	1
    6056	ret

Not too hard, we have

- if r0 != 0 call fn6057. We see below that fn6057 also use r0 and r1 for parameters.
- r0 = r1 + 1
- return, aka return (a, b)

Translated into high-level code:

    fn fn6049(a: u16, b: u16) -> u16 {
        if a != 0 {
            return fn6057(a, b);
        }
        return b + 1;
    }

### `fn6057`

    6057	jt	r1	6070
    6060	add	r0	r0	32767
    6064	set	r1	r7
    6067	call	6049
    6069	ret

It's similar to the previous one, we need to noticed that + 32767 is actually -1. So the end being:

    a -= 1;
    b = REG8;
    call Fn_6049(a, b)
    return (a, b)

so:

    fn fn6057(a: u16, b: u16) -> u16 {
        if b != 0 {
            return Fn_6070(a, b);
        }
        return fn6049(a - 1, REG8);
    }

### `fn6070`

    6070	push	r0
    6072	add	r1	r1	32767
    6076	call	6049
    6078	set	r1	r0
    6081	pop	r0
    6083	add	r0	r0	32767
    6087	call	6049
    6089	ret

With the use of the stack, it's a bit harder. A first simplification gives:

    push a
    b -= 1;
    (a, b) = Fn_6049(a, b);
    b = a;
    a = pop
    a -= 1;
    return Fn_6049(a, b);

Then:

    push a
    a = Fn_6049(a, b - 1);
    b = a;
    a = pop
    a -= 1;
    return Fn_6049(a, b);

The push/pop a makes sure we use the initial a when removing 1 from it. So it simplifies into:

    b = Fn_6049(a, b - 1);

    a -= 1;
    return Fn_6049(a, b);

And finally:

    fn fn6070(a: u16, b: u16) -> u16 {
        a = a - 1;
        return Fn_6049(a - 1, Fn_6049(a, b - 1));
    }

### Merging them

If we reorder the checks:

    fn fn6049(a: u16, b: u16) -> u16 {
        if a == 0 {
            return b + 1;
        }
        return fn6057(a, b);
    }

    fn fn6057(a: u16, b: u16) -> u16 {
        if b == 0 {
            return fn6049(a - 1, REG8);
        }
        return fn6070(a, b);
    }

    fn fn6070(a: u16, b: u16) -> u16 {
        return fn6049(a - 1, fn6049(a, b - 1));
    }

We can merge it into one recursive function:

    fn fn6049(a: u16, b: u16) -> u16 {
        if a == 0 {
            return b + 1;
        }
        if b == 0 {
            return fn6049(a - 1, REG8);
        }
        return fn6049(a - 1, fn6049(a, b - 1));
    }

## Running `fn6049`

This function is deeply recursive and not very fast, and we need to run it up to 32767 times to find which value of REG8 produces 6.

To run it, see details in [teleporter_code.rs](src/maze/teleporter_code.rs).

In basically means:

- Using memoization to reduce recursion.
- Increase stack size.
- Use threads to parallelize checks.
- Have a fast computer and patience.