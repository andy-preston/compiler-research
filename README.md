# Research into finding a suitable compiler for Crystal Palace Synth project

## What's here for you?

### Free Pascal

A Docker container to build and use an ARM cross compiler for
[Free Pascal](https://www.freepascal.org/)
targeting Cortex-M4 ARM-based microcontrollers such as the STM32F401.

The [Dockerfile](compiler/Dockerfile) also serves as a nice guide to
the rigmarole of building a Free Pascal cross-compiler which you might find
handy in your own projects.

### GCC

A similar dockerfile for building a GCC Toolchain for the STM32F401.

### Rust

A Board Support Crate for STM32F401 (Black Pill) Development Board.
See [rust/README.md](rust/README.md)

## Goals

My project will consist of a framework written in Thumb assembly language
for the STM32F401 which will also contain some maths-heavy routines written in
a high level language.

Based on personal preference, I'd have liked the high level language in question
to be Pascal (I always preferred Algol style syntax to BCPL style syntax) but
I'm also considering C as a good-old standby that covers all the bases and Rust
because I should be thinking about moving with the times.

## Results

### Free Pascal

Although this has the most points for "it will make me happy", currently I am
unable to get this to produce instructions for the single precision FPU that
comes with the STM32F401's Cortex-M4 core.

### GCC

My test code seems to be producing FPU instructions which is a good start.
Although there does also seem to be a lot of boilerplate in there to support
a standard library or language semantics. Needs more checking.

```C
float p[200];
for (int i = 0; i < 200; i++) {
    p[i] = 2.0f * i;
}
```

```ArmAsm
    push {r7}
    sub.w sp, sp, #812        ; 0x32c
    add r7, sp, #0
    movs r3, #0
    str.w r3, [r7, #804]      ; 0x324
    b.n 8000040 <main+0x38>
    ldr.w r3, [r7, #804]      ; 0x324
    vmov s15, r3
    vcvt.f32.s32 s15, s15
    vadd.f32 s15, s15, s15
    adds r2, r7, #4
    ldr.w r3, [r7, #804]      ; 0x324
    lsls r3, r3, #2
    add r3, r2
    vstr s15, [r3]
    ldr.w r3, [r7, #804]      ; 0x324
    adds r3, #1
    str.w r3, [r7, #804]      ; 0x324
    ldr.w r3, [r7, #804]      ; 0x324
    cmp r3, #199              ; 0xc7
    ble.n 8000018 <main+0x10>
    movs r3, #0
    mov r0, r3
    add.w r7, r7, #812        ; 0x32c
    mov sp, r7
    ldr.w r7, [sp], #4
    bx lr
```

### Rust

Not got any further than flashing a basic "blinky" as yet.
